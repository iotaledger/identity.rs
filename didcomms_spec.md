
---
# Authentication


Proving control over an Identifier.

The Authentication flow consists of a simple request-response message exchange, where the contents of the response must match those of the request. Because all Messages are signed and authenticated, the response functions as proof of control by nature of being correctly signed by the keys listed in the DID Document of the issuer. Because of this, in scenarios where a more complex functionality (e.g. Credential Verification) is needed, an additional Authentication flow is not necessary.

### Roles
- **Verifier**: Agent who requests and verifies the Authenticity of the Authenticator
- **Authenticator**: Agent who proves control over their Identifier

### Messages

#### Authentication Request
The verifier sends the **authentication request** to the authentication service endpoint of the authenticator, specifying a `callbackURL` for the reponse to be posted to, as well as an arbitrary `description` which is to be signed by the authenticator. 

###### Layout

```json
{
    callbackURL: <URL as String>,
    description: <Text as String>,
}
```

#### Authentication Response
The authenticator answers with an **authentication response**, quoting the request it answers to and providing a `signature` of the `authenticationRequest` field, which is the complete original **authentication request**.

###### Layout

```json
{
    authenticationRequest: {
        callbackURL: <URL as String>,
        description: <Text as String>,
    },
    signature: {
      "type": <Signature Type as String>,
      "verificationMethod": <Verification Method as String>,
      "signatureValue": <Signature as String>
   }
}
```

### Examples

Alice wants to know whether an identity she received earlier corresponds to the domain https://www.bob.com. She sends an **authentication request** to the domain specified in the identity's service endpoint:

```json
{
    callbackURL: "https://example.com/auth",
    description: "Are you Bob?",
}
```

The service endpoints receives the **authentication request** and answers with e.g. the following **authentication response**:

```json
{
    authenticationRequest: {
        callbackURL: "https://www.bob.com/auth",
        description: "Are you Bob?",
    },
    signature: {
      "type": "JcsEd25519Signature2020",
      "verificationMethod": "#authentication",
      "signatureValue": "5Hw1JWv4a6hZH5obtAshbbKZQAJK6h8YbEwZvdxgWCXSL81fvRYoMCjt22vaBtZewgGq641dqR31C27YhDusoo4N"
   }
}
```

The `signature` provided here must correspond with the `#authentication` public key provided in the DID Document of the identity that Alice has received earlier. If that is the case, the domain is authenticated successfully.



---
# (Interaction Flow Title)

(Information and description about the flow)

### Roles
- (Role 1): (Description 1)
- (Role 2): (Description 2)

### Messages

##### (Message 1)
(Information about Message 1)

###### Layout

###### Response

##### (Message 2)
(Information about Message 2)

###### Layout

###### Response

### Examples