wiki-cli
========



[![oclif](https://img.shields.io/badge/cli-oclif-brightgreen.svg)](https://oclif.io)
[![Version](https://img.shields.io/npm/v/wiki-cli.svg)](https://npmjs.org/package/wiki-cli)
[![Downloads/week](https://img.shields.io/npm/dw/wiki-cli.svg)](https://npmjs.org/package/wiki-cli)
[![License](https://img.shields.io/npm/l/wiki-cli.svg)](https://github.com/eike-hass/wiki-cli/blob/master/package.json)

<!-- toc -->
* [Usage](#usage)
* [Commands](#commands)
<!-- tocstop -->
# Usage
<!-- usage -->
```sh-session
$ npm install -g wiki-cli
$ wiki-cli COMMAND
running command...
$ wiki-cli (-v|--version|version)
wiki-cli/0.0.0 linux-x64 node-v14.16.1
$ wiki-cli --help [COMMAND]
USAGE
  $ wiki-cli COMMAND
...
```
<!-- usagestop -->
# Commands
<!-- commands -->
* [`wiki-cli hello [FILE]`](#wiki-cli-hello-file)
* [`wiki-cli help [COMMAND]`](#wiki-cli-help-command)
* [`wiki-cli setup`](#wiki-cli-setup)
* [`wiki-cli start [FILE]`](#wiki-cli-start-file)

## `wiki-cli hello [FILE]`

describe the command here

```
USAGE
  $ wiki-cli hello [FILE]

OPTIONS
  -f, --force
  -h, --help       show CLI help
  -n, --name=name  name to print

EXAMPLE
  $ wiki-cli hello
  hello world from ./src/hello.ts!
```

_See code: [src/commands/hello.ts](https://github.com/eike-hass/wiki-cli/blob/v0.0.0/src/commands/hello.ts)_

## `wiki-cli help [COMMAND]`

display help for wiki-cli

```
USAGE
  $ wiki-cli help [COMMAND]

ARGUMENTS
  COMMAND  command to show help for

OPTIONS
  --all  see all commands in CLI
```

_See code: [@oclif/plugin-help](https://github.com/oclif/plugin-help/blob/v3.2.3/src/commands/help.ts)_

## `wiki-cli setup`

setup local wiki

```
USAGE
  $ wiki-cli setup

OPTIONS
  -h, --help     show CLI help
  -r, --ref=ref  wiki revison to checkout

EXAMPLE
  $ wiki-cli setup --ref main
```

_See code: [src/commands/setup.ts](https://github.com/eike-hass/wiki-cli/blob/v0.0.0/src/commands/setup.ts)_

## `wiki-cli start [FILE]`

describe the command here

```
USAGE
  $ wiki-cli start [FILE]

OPTIONS
  -f, --force
  -h, --help       show CLI help
  -n, --name=name  name to print
```

_See code: [src/commands/start.ts](https://github.com/eike-hass/wiki-cli/blob/v0.0.0/src/commands/start.ts)_
<!-- commandsstop -->
