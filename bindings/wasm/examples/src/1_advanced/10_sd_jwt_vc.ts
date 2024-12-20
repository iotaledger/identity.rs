// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IssuerMetadata, SdJwtVcBuilder, Sha256Hasher, Timestamp, TypeMetadataHelper, IResolver, JwkType, IJwk, Jwk, IJwkParams } from "@iota/identity-wasm/node";
import { base64url, exportJWK, generateKeyPair, JWK, JWTHeaderParameters, JWTPayload, SignJWT } from "jose";

const vc_metadata: TypeMetadataHelper = JSON.parse(`{
  "vct": "https://example.com/education_credential",
  "name": "Betelgeuse Education Credential - Preliminary Version",
  "description": "This is our development version of the education credential. Don't panic.",
  "claims": [
    {
      "path": ["name"],
      "display": [
        {
          "lang": "de-DE",
          "label": "Vor- und Nachname",
          "description": "Der Name des Studenten"
        },
        {
          "lang": "en-US",
          "label": "Name",
          "description": "The name of the student"
        }
      ],
      "sd": "allowed"
    },
    {
      "path": ["address"],
      "display": [
        {
          "lang": "de-DE",
          "label": "Adresse",
          "description": "Adresse zum Zeitpunkt des Abschlusses"
        },
        {
          "lang": "en-US",
          "label": "Address",
          "description": "Address at the time of graduation"
        }
      ],
      "sd": "always"
    },
    {
      "path": ["address", "street_address"],
      "display": [
        {
          "lang": "de-DE",
          "label": "Straße"
        },
        {
          "lang": "en-US",
          "label": "Street Address"
        }
      ],
      "sd": "always",
      "svg_id": "address_street_address"
    },
    {
      "path": ["degrees", null],
      "display": [
        {
          "lang": "de-DE",
          "label": "Abschluss",
          "description": "Der Abschluss des Studenten"
        },
        {
          "lang": "en-US",
          "label": "Degree",
          "description": "Degree earned by the student"
        }
      ],
      "sd": "allowed"
    }
  ]
}`)

const issuer_jwks = async (): Promise<[JWK, JWK]> => {
  const [sk, pk] = await generateKeyPair("ES256").then(res => [res.privateKey, res.publicKey]);
  const sk_jwk = await exportJWK(sk);
  const pk_jwk = await exportJWK(pk);

  return [sk_jwk, pk_jwk];
}

export async function sdJwtVc() {
  const hasher = new Sha256Hasher();
  const issuer = "https://example.com/";
  const [sk_jwk, pk_jwk] = await issuer_jwks();
  const issuer_public_jwk = { ...pk_jwk, kty: JwkType.Ec, kid: "key1" } as IJwk;
  const issuer_metadata = new IssuerMetadata(issuer, { jwks: { keys: [issuer_public_jwk] } })
  const dummy_resolver = {
    resolve: async (input: string) => {
      if (input == "https://example.com/.well-known/jwt-vc-issuer/") return new TextEncoder().encode(JSON.stringify(issuer_metadata.toJSON()));
      if (input == "https://example.com/.well-known/vct/education_credential") return new TextEncoder().encode(JSON.stringify(vc_metadata));
    }
  } as IResolver<string, Uint8Array>;

  const sign = async (header: object, payload: object) => {
    console.log(`about to sign payload ${typeof payload} with values ${JSON.stringify(payload)}`)
    return new SignJWT(payload as JWTPayload)
      .setProtectedHeader(header as JWTHeaderParameters)
      .sign(sk_jwk)
      .then(jws => new TextEncoder().encode(jws))
  }

  let sd_jwt_vc = await new SdJwtVcBuilder({
    name: "John Doe",
    address: {
      street_address: "A random street",
      number: "3a",
    },
    degree: []
  }, hasher)
    .header({ kid: "key1" })
    .vct("https://example.com/education_credential")
    .iat(Timestamp.nowUTC())
    .iss(issuer)
    // .requireKeyBinding({ kid: "key2" })
    .makeConcealable("/address/street_address")
    .makeConcealable("/address")
    .finish({ sign }, "ES256");

  console.log(sd_jwt_vc.toString());


  await sd_jwt_vc.validate(dummy_resolver, hasher);

  console.log("SdJwtVc is valid!");
}