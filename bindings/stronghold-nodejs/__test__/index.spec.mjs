import test from 'ava'

import { sum, flipKeytype, NapiKeyType, CoreDID } from '../dist/index.js'

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})

test('flip keytype', (t) => {
  t.is(flipKeytype(NapiKeyType.Ed25519), NapiKeyType.X25519)
})

test('CoreDID parse', (t) => {
  t.is(CoreDID.parse("did:example:12345678").method(), "example")
})