# Identity Communication

Identity Communication or DID Communication (short: DIDComm), describes the communication between People, Organizations and Things.


## DIDComm specifics
- Don’t have to involve turn-taking and request-response
- Can involve more than 2 parties (agents), for example: an auction
- May include formats other than JSON

## Goals 
- **Secure**
- **Private**
- **Interoperable**
    - Works across programming languages, Distributed Ledger Technlologies, Vendors, Operating Systems, Networks, Time, and more.
- **Transport-agnostic**
    - Can be used over HTTP, Websockets, IRC, Bluetooth, Email,, Push notifications to mobile devices, Carrier pigeon, IOTA Streams, and more.
- **Extensible**

## Communication Paradigm

- The web communication is mostly duplex request-response 
    - Communication over a client and a web server.
- Agents are different than web servers
    - Lack of stable internet connection
    - No internet connection
- Fundamental paradigm
    - Message-based
    - Asynchronous
    - Simplex
> DIDComm need to be close to email paradigm than a standard web paradigm.

## Summary
- Communication standard
- Uses public key cryptography
    - No certificates from some parties or passworts
- It’s Interoperable and Transport-agnostic
- Minimal two parties are included, sometimes more
