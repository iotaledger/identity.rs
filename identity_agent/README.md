# IOTA Identity Agent

The identity agent is a peer-to-peer communication framework for building digital agents on IOTA Identity. It is intended to host implementations of the [DIDComm protocols](https://wiki.iota.org/identity.rs/specs/didcomm/overview) with future updates. Together with these protocols, this will, for example, allow for did-authenticated communication between two identities to exchange verifiable credentials or presentations.

For a high-level and less technical introduction, see the [blog post](https://blog.iota.org/the-iota-identity-actor-explained/) on the agent (formerly known as identity actor).

The most important dependency of the agent is libp2p. [What is libp2p?](https://docs.libp2p.io/introduction/what-is-libp2p/)

> The one-liner pitch is that libp2p is a modular system of _protocols_, _specifications_ and _libraries_ that enable the development of peer-to-peer network applications.

We use libp2p because it can easily secure transports using the noise protocol, is agnostic of transports (so agents could conceivably communicate over TCP, websockets or Bluetooth), and because of how flexible it is we can make it suit the agent nicely.

## Building an agent

```rust
let id_keys: IdentityKeypair = IdentityKeypair::generate_ed25519();
let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;

let mut agent: Agent = AgentBuilder::new()
  .keypair(id_keys)
  .build()
  .await?;

agent.start_listening(addr).await?;
```

To build a minimal working agent, we generate a new `IdentityKeypair` from which the `AgentId` of the agent is derived. The `AgentId` is an alias for a `libp2p::PeerId`, which allows for cryptographically verifiable identification of a peer. This decouples the identity concept from the underlying network address, which is important if the agent roams across networks. If we want the agent to have the same `AgentId` across program executions, we need to store this keypair. Next we create the address for the agent to listen on. A `Multiaddr` is the address format in libp2p to encode addresses of various transports. Finally, we build the agent with a default transport, that supports DNS resolution and can use TCP or websockets.

## Processing incoming requests

To make the agent do something useful, we need handlers. A handler is some state with associated behavior that processes incoming requests. It will be invoked if the agent is able to deserialize the incoming request to the type the handler expects. The `Handler` is a trait that looks like this:

```rust
#[async_trait::async_trait]
pub trait Handler<REQ: HandlerRequest>: Debug + 'static {
  async fn handle(&self, request: RequestContext<REQ>) -> REQ::Response;
}
```

- It takes `&self` so it can modify its state through appropriate mechanisms, such as locks. A handler will thus typically implement a shallow copy mechanism (e.g. using `Arc`) to share state.
- It takes the request it wants to handle, which needs to implement the `HandlerRequest` trait and needs to return the defined response type.
- This trait can be implemented multiple times so the same handler can process different request types.

Here is an example of a handler being attached on an `AgentBuilder`. We implement `RemoteAccounts`, an exemplary type that manages `Account`s remotely.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemoteAccountsError {
  IdentityNotFound,
}

/// The struct that will be sent over the network.
/// When received by an agent, the contained DID is looked up.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteAccountsGet(pub IotaDID);

impl HandlerRequest for RemoteAccountsGet {
  /// The result of the lookup procedure, either the corresponding IotaDocument, or an error.
  type Response = Result<IotaDocument, RemoteAccountsError>;

  /// `Endpoint`s are identifiers for requests which lets the remote agent determine the appropriate handler to invoke.
  fn endpoint() -> Endpoint {
    "remote_accounts/get".try_into().unwrap()
  }
}

/// Our thread-safe type that holds accounts that can be looked up by their DID.
#[derive(Debug, Clone, Default)]
pub struct RemoteAccounts {
  accounts: Arc<dashmap::DashMap<IotaDID, Account>>,
}

#[async_trait::async_trait]
impl Handler<RemoteAccountsGet> for RemoteAccounts {
  /// To handle the request, we take the HandlerRequest type wrapped in a RequestContext, which provides some
  /// useful information about the caller like their `AgentId`.
  async fn handle(&self, request: RequestContext<RemoteAccountsGet>) -> Result<IotaDocument, RemoteAccountsError> {
    self
      .accounts
      .get(&request.input.0)
      .map(|account| account.document().to_owned())
      .ok_or(RemoteAccountsError::IdentityNotFound)
  }
}

/// To build the agent with our custom functionality, we first build the agent itself
/// and attach the handler.
async fn build_agent() {
  let mut builder = AgentBuilder::new();
  builder.attach(RemoteAccounts::default());
}
```

An agent that receives a request will check whether a handler is attached that can handle the request's `Endpoint` and if so, will invoke it. In our case, the agent will call the `handle` function of the handler when a `RemoteAccountsGet` request is received. If we wanted, we could attach more handlers to the same agent, and even implement `Handler` for `RemoteAccounts` multiple times, in order to handle different request types.

## Sending requests

To invoke a handler on a remote agent, we send a type that implements `HandlerRequest`, such as `RemoteAccountsGet`.

```rust
let mut agent: Agent = builder.build().await?;

agent.add_agent_address(remote_agent_id, addr).await?;

let result: Result<IotaDocument, RemoteAccountsError> = agent
  .send_request(remote_agent_id, RemoteAccountsGet("did:iota:...".parse()?))
  .await?;
```

After building the agent and adding the address of the remote agent, we can send a request. The agent takes care of serializing the request, and attempts to deserialize the response into `<RemoteAccountsGet as HandlerRequest>::Response`.

## Agent modes

We've just seen an example of a synchronous request, one where we invoke a handler on a remote agent and wait for it to finish execution and return a result. Next to the `Agent` type we also have a `DidCommAgent` type. The latter additionally supports an asynchronous mode, where we send a request without waiting for the result of the handler invocation. Instead, we can explicitly await a request:

```rust
async fn didcomm_protocol(agent_id: AgentId, didcomm_agent: &DidCommAgent) -> AgentResult<()> {
  let thread_id: ThreadId = ThreadId::new();

  didcomm_agent
    .send_didcomm_request(agent_id, &thread_id, PresentationOffer::default())
    .await?;

  let request: DidCommPlaintextMessage<PresentationRequest> = didcomm_agent.await_didcomm_request(&thread_id).await?;

  Ok(())
}
```

This request mode is implemented to support the implementation of [DIDComm](https://identity.foundation/didcomm-messaging/spec/) protocols, which is why a separate `DidCommAgent` is defined that extends the `Agent`s functionality and handles the specifics of DIDComm. Note that the base `Agent` doesn't support the asynchronous mode, but the `DidCommAgent` supports the sychronous mode.

Here, the protocol expects us to first send a `PresentationOffer` request to the remote agent. This method call returns successfully if the request can be deserialized properly and if an appropriate handler exists on the remote agent, but the call might return before the handler on the remote has finished. According to the protocol we implement, we should expect the remote to send us a `PresentationRequest` so we explicitly call `await_didcomm_request` to await the incoming request on the same `ThreadId` that we sent our previous request on. This allows for imperative protocol implementations within a single handler. This is nice to have, because the alternative would be that each request invokes a separate handler in an agent, which would force protocol implementors to hold the state in some shared state, rather than implicitly in the function (such as the `thread_id` here). This setup is intended for DIDComm protocols, as it directly implements DIDComm concepts such as threads.

## Examples

There are currently no examples for the agent in the `examples` directory. This is mostly due to the instability of the agent. Still, there are two "examples" for each mode of operation as part of the `tests` module, the remote account as a synchronous example, and the IOTA DIDComm presentation protocol as an asynchronous example (this doesn't implement the actual protocol, it just asserts that requests can be sent back and forth as expected). The DIDComm example in particular is very simple and minimal and mostly exists as a proof of concept for the async mode, but it also serves as an example for how a DIDComm protocol could potentially be implemented.

### DIDComm example setup

The async mode didcomm examples are worth explaining a little more. The implementation difficulty for these protocols comes mostly because of how flexible they are. In the presentation protocol for example, both the holder and verifier can initiate the exchange. On the agent level this means either calling the protocol explicitly to initiate it, or attaching a handler to let the agent handle the protocol in the background when a remote agent initiates. Thus, there is one function that implements the actual protocol for each of the roles (i.e. _holder_ and _verifier_ in the `presentation` protocol). As an example, this is what the signature of the holder role would look like:

```rust
pub(crate) async fn presentation_holder_handler(
  mut agent: DidCommAgent,
  agent_id: AgentId,
  request: Option<DidCommPlaintextMessage<PresentationRequest>>,
) -> AgentResult<()> { ... }
```

The holder can call this function to initiate the protocol imperatively by passing `None` as `request`. On the other hand, if the verifier initiates, the holder defines a handler that will inject the received `request`:

```rust
#[async_trait::async_trait]
impl DidCommHandler<DidCommPlaintextMessage<PresentationRequest>> for DidCommState {
  async fn handle(&self, agent: DidCommAgent, request: RequestContext<DidCommPlaintextMessage<PresentationRequest>>) {
    let result = presentation_holder_handler(agent, request.agent_id, Some(request.input)).await;

    if let Err(err) = result {
      log::error!("presentation holder handler errored: {err:?}");
    }
  }
}
```

and attaches it:

```rust
didcomm_builder.attach::<DidCommPlaintextMessage<PresentationRequest>, _>(DidCommState::new());
```

`DidCommState` holds the state for one or more DIDComm protocols. When a `PresentationRequest` is received, it calls the protocol function (`presentation_holder_handler`) to run through the protocol. This allows us to nicely reuse the `presentation_holder_handler` function as the core protocol implementation and only requires defining a thin handler method. The verifier can follow the same pattern for their side of the protocol.

## Implementation Details

This section goes into some details of agent internals.

### Agent internals

The overall architecture can be seen as four layers. A libp2p layer, a commander layer to interact with the libp2p layer, the raw agent layer (which uses the commander) and the `DidCommAgent` on top. This architecture is strongly inspired by [stronghold-p2p](https://github.com/iotaledger/stronghold.rs/tree/dev/p2p).

- The p2p layer consists of a `libp2p::RequestResponse` protocol, which enforces on a type level that each request has a response. This naturally maps to the sync mode of the identity agent where each request has some response, as well as to the async mode where each request will be acknowledged.
- This layer has an `EventLoop` that concurrently polls the libp2p `Swarm` to handle its events as well as commands that are sent to it from the `NetCommander`.
- The commander layer, or `NetCommander` communicates with the event loop via channels and is thus the interface for the `EventLoop`.
- When the agent is built, it spawns an `EventLoop` in the background and interacts with it using the `NetCommander`.
- On incoming requests, the `EventLoop` spawns a new task and injects a clone of the agent into it (see `EventLoop::run` and its argument).

### DidCommAgent internals

- In async mode, the `DidCommAgent` returns an acknowledgment if 1) a handler for the endpoint or a thread exists and 2) if the request can be deserialized into the expected type for the handler or thread (e.g. a DIDComm plaintext message)
- Timeouts can occur in two ways and both are configured via `AgentBuilder::timeout`.
  - A request sender can receive an `InboundFailure::Timeout` if the peer did not respond within the configured timeout. This happens on the event loop level and is handled by the `RequestResponse` protocol.
  - `DidCommAgent::await_didcomm_request` can time out. This is the same timeout value as for the underlying `RequestResponse` protocol. In such a case, the event loop will receive a timeout error, but since no entry in the thread hash map is waiting for a response, it is silently dropped. Thus, `await_didcomm_request` implements its own timeout, and automatically uses the same duration as the underlying protocol to ensure consistent behaviour. For this reason, the `await_didcomm_request` timeout is a per-agent configuration value, and not a parameter on the function, although that would also be possible if desired.
