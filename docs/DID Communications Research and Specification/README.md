![banner](./.meta/identity_banner.png)

# DID Communications Message Specification

*version 0.3, last changed April 2021*

## Interactions

◈ <a href="#trust-ping">**trust-ping**</a> (*ping*, *pingResponse*): Testing a pairwise channel.

◈ <a href="#did-discovery">**did-discovery**</a> (*didRequest*, *didResponse*): Requesting a DID from an agent.

◈ <a href="#did-introduction">**did-introduction**</a> (*introductionProposal*, *introductionResponse*, *introduction*): Describes how a go-between can introduce parties that it already knows, but that do not know each other.

◈ <a href="#features-discovery">**features-discovery**</a> (*featuresRequest*, *featuresResponse*): Enabling agents to discover which interactions other agents support.

◈ <a href="#did-resolution">**did-resolution**</a> (*resolutionRequest*, *resolutionResponse*): Using another agent as a Resolver.

◈ <a href="#authentication">**authentication**</a> (*authenticationRequest*, *authenticationResponse*): Proving control over a DID.

◈ <a href="#credential-options">**credential-options**</a> (*credentialOptionsRequest*, *credentialOptionsResponse*): Querying an agent for the VCs that the agent can issue.

◈ <a href="#credential-schema">**credential-schema**</a> (*credentialSchemaRequest*, *credentialSchemaResponse*): Querying an agent for the schema of a specific VC that the agent can issue.

◈ <a href="#credential-issuance">**credential-issuance**</a> (*credentialSelection*, *credentialIssuance*): Creating an authenticated statement about a DID.

◈ <a href="#credential-revocation">**credential-revocation**</a> (*revocation*): Notifying a holder that a previously issued credential has been revoked.

◈ <a href="#presentation-verification">**presentation-verification**</a> (*presentationRequest*, *presentationResponse*): Proving a set of statements about an identifier.
