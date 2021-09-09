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
* [`wiki-cli help [COMMAND]`](#wiki-cli-help-command)
* [`wiki-cli nuke`](#wiki-cli-nuke)
* [`wiki-cli setup`](#wiki-cli-setup)
* [`wiki-cli start`](#wiki-cli-start)

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

## `wiki-cli nuke`

completely removes local wiki

```
USAGE
  $ wiki-cli nuke

OPTIONS
  -h, --help  show CLI help
```

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

## `wiki-cli start`

start local wiki

```
USAGE
  $ wiki-cli start

OPTIONS
  -h, --help  show CLI help
```
<!-- commandsstop -->
