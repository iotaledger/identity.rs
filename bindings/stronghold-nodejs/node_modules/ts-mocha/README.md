[![Latest Stable Version](https://img.shields.io/npm/v/ts-mocha.svg)](https://www.npmjs.com/package/ts-mocha)
[![NPM Downloads](https://img.shields.io/npm/dt/ts-mocha.svg)](https://www.npmjs.com/package/ts-mocha)
[![NPM Downloads](https://img.shields.io/npm/dm/ts-mocha.svg)](https://www.npmjs.com/package/ts-mocha)

[![Codeship Status for piotrwitek/ts-mocha](https://app.codeship.com/projects/cb8cc460-1719-0137-28fd-3a09a0997096/status?branch=master)](https://app.codeship.com/projects/328034)
[![Greenkeeper badge](https://badges.greenkeeper.io/piotrwitek/ts-mocha.svg)](https://greenkeeper.io/)
[![Dependency Status](https://img.shields.io/david/piotrwitek/ts-mocha.svg)](https://david-dm.org/piotrwitek/ts-mocha)
[![peerDependency Status](https://img.shields.io/david/peer/piotrwitek/ts-mocha.svg)](https://david-dm.org/piotrwitek/ts-mocha#info=devDependencies)

# TS-Mocha
> Mocha thin wrapper that allows running TypeScript tests with TypeScript runtime (ts-node) to get rid of compilation complexity.
> All Mocha features are available without any limitation.

## Why?
To setup Mocha with TypeScript you have to figure out how to integrate them to work together, it's not an easy task and require some advanced knowledge.
This package handles for you this complexity and let you use ts-mocha just as regular mocha that will handle TypeScript `.ts` and `.tsx` files. Also added some useful options, you can find them below.

## How?
TS-Mocha has one only dependency - ts-node, which is used as a TypeScript runtime to execute tests that can import and run imported TypeScript source files as well. It is as a thin wrapper that run local mocha package and set up ts-node environment to handle `.ts` and `.tsx` files. To speed up TypeScript tests execution type-checking is disabled, using only transpile module.

> __NOTE__: This package does not include Mocha - I have set Mocha as peer dependency on purpose, so I don't lock consumer to a specific Mocha version but most importantly, I don't have to update this package when Mocha is updated, and all the new features will be available automatically from your local Mocha package. Also integration with your existing Mocha setup is non-invasive.

> __PRO TIP__: To make your developer experience better I recommend to run type-checking in a separate process by starting TSC compiler (preferably in watch mode) in you terminal with --noEmit and --project flags.

## Installation

```bash
# remember to install mocha if you don't have it already (npm i -D mocha)

npm i -D ts-mocha

# install recent Mocha and Expect @types packages for best DX
npm i -D @types/mocha @types/expect
```

## Usage

### - CLI Usage:

CLI options consist of all the options of regular Mocha plus extra options below:

`-p, --project <value>` - relative or absolute path to a `tsconfig.json` file (equivalent of `tsc -p <value>`) [default: "./tsconfig.json"]

**Example:**
```bash
ts-mocha -p src/tsconfig.json src/**/*.spec.ts
```

`--paths` - feature toggle flag to enable [`tsconfig-paths`](https://www.npmjs.com/package/tsconfig-paths) integration [default: false]
> `tsconfig-paths` is an optional dependency, make sure to install it locally in your project

When using [path mapping](https://www.typescriptlang.org/docs/handbook/module-resolution.html#path-mapping) via the `paths` compiler option in `tsconfig.json` this library utilizes the [`tsconfig-paths`](https://www.npmjs.com/package/tsconfig-paths) package, allowing for automatic resolution of aliased modules locations during test execution.

Check our test suite for a reference implementation: [Link](./test/paths/tsconfig.json)

**Example:**
```bash
ts-mocha --paths -p src/ src/**/*.spec.ts
```

`--type-check` - feature toggle flag to enable type checking in ts-node [default: false]

By default ts-mocha uses the `--transpile-only` option of ts-node to make tests run faster. Use the `--type-check` option to enable type checking in ts-node.

**Example:**
```bash
ts-mocha --type-check -p src/ src/**/*.spec.ts
```

### Watch Mode:
If you want your tests to be automatically rerun when your code changes, add both the `-w` flag and the `--watch-extensions` flag telling it to watch for typescript files.

**Example:**
```bash
ts-mocha --paths -p src/ src/**/*.spec.ts -w --watch-extensions ts
```

### - Programmatic usage:

In code you can use ts-mocha by adding a single require at the beginning of your script:

```javascript
// set env variable to the `tsconfig.json` path before loading mocha (default: './tsconfig.json')
process.env.TS_NODE_PROJECT = './src/tsconfig.json'

// Optional: set env variable to enable `tsconfig-paths` integration
process.env.TS_CONFIG_PATHS = true;

// register mocha wrapper
require('ts-mocha');
```

For example:

```javascript
process.env.TS_NODE_PROJECT = './src/tsconfig.json';
require('ts-mocha');
const Mocha = require('mocha');

const mocha = new Mocha();
mocha.addFile(`./src/file.spec.ts`);
mocha.run((failures) => {
  process.on('exit', () => {
    process.exit(failures); // exit with non-zero status if there were failures
  });
});
```
