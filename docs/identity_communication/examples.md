# Examples
This document describes some use cases and examples about DID and their communication.

## Example 1: DIDComm interaction
- Alice wants to buy something online, Bob is the seller
- Bob need to deliver an DID Document to Alice
    - Have to provide an endpoint (web, email, etc) where message can be delivered
    - Public key, for the Alice:Bob relationship
- Alice press buy button
    - Alice encrypt order with Bob's public key
    - Add authentication with own private key
    - Send the encrypted message to Bob’s endpoint
- Bob becomes the message and respond to it 
    - Decrypt and authenticate its origin
    - Send response (plaintext -> lookup endpoint and public key for Alice -> encrypt with authentication -> arrange delivery)

## Example 2: Door Opener
- Mobile Device
    - Read QR Code. get Information (via link (http) or is it also offline possible? (DID Data in QR Code?))
    - Send decrypted data to  the given endpoint
- Door with QR Code
    - Get the request and validate for access
    - Open door (or not)
    - [optional] Sends data to the response endpoint.
