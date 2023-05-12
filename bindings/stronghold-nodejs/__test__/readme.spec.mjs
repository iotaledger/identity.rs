import test from 'ava'

import { execSync } from "child_process"

test('README examples works', (t) => {
    t.pass(execSync("txm README.md"))
  })