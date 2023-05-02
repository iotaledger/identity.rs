import test from 'ava'

import { sum, flipKeytype, NapiKeyType } from '../dist/index.js'

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})

test('flip keytype', (t) => {
  t.is(flipKeytype(NapiKeyType.Ed25519), NapiKeyType.X25519)
})