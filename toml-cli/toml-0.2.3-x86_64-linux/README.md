# toml-cli

This is the home of the `toml` command, a simple CLI for editing
and querying TOML files.

The intent of the `toml` command is to be useful
 * in shell scripts, for consulting or editing a config file;
 * and in instructions a human can follow for editing a config file,
   as a command to copy-paste and run.

A source of inspiration for the interface is the `git config` command,
which serves both of these purposes very well without knowing anything
about the semantics of Git config files -- only their general
structure.

A key property is that when editing, we seek to *preserve formatting
and comments* -- the only change to the file should be the one the
user specifically asked for.  To do this we rely on the `toml_edit`
crate, which also underlies `cargo-edit`.  There are a few edge cases
where `toml_edit` can rearrange an oddly-formatted file (described in
the `toml_edit` documentation); but for typical TOML files, we
maintain this property with perfect fidelity.

The command's status is **experimental**.  The current interface does
not yet serve its purposes as well as it could, and **incompatible
changes** are anticipated.

## Usage

### Reading: `toml get`

To read specific data, pass a *TOML path*: a sequence of *path
segments*, each of which is either:
 * `.KEY`, to index into a table or inline-table, or
 * `[INDEX]`, to index into an array-of-tables or array.

Data is emitted by default as JSON:

```
$ toml get Cargo.toml bin[0]
{"name":"toml","path":"src/main.rs"}
```

When the data is a string, the `--raw`/`-r` option prints it directly,
for convenience in contexts like a shell script:

```
$ toml get Cargo.toml dependencies.serde --raw
1.0
```

If you need a more complex query, consider a tool like `jq`, with
`toml` simply transforming the file to JSON:

```
$ toml get pyoxidizer.toml . | jq '
    .embedded_python_config[] | select(.build_target | not) | .raw_allocator
  ' -r
jemalloc
```

(The TOML path `.` is an alias for the empty path, describing the
whole file.)

### Writing (ish): `toml set`

To edit the data, pass a TOML path specifying where in the parse tree
to put it, and then the data value to place there:

```
$ cat >foo.toml <<EOF
[a]
b = "c"
EOF

$ toml set foo.toml x.y z
[a]
b = "c"

[x]
y = "z"
```

This subcommand is quite raw in two respects:
 * We don't actually edit the file; we only print out the new version.
 * The value to be set must be a string; input of booleans, arrays, etc.
   is unimplemented.

## Reference

### Base command `toml`

```
$ toml --help
toml-cli 0.2.3
A simple CLI for editing and querying TOML files.

USAGE:
    toml <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    get     Print some data from the file
    help    Prints this message or the help of the given subcommand(s)
    set     Edit the file to set some data (currently, just print modified version)
```

### `toml get`

```
$ toml get --help
toml-get 0.2.3
Print some data from the file

Read the given TOML file, find the data within it at the given query,
and print.

If the TOML document does not have the given key, exit with a
failure status.

Output is JSON by default.  With `--raw`/`-r`, if the data is a
string, print it directly.  With `--output-toml`, print the data
as a fragment of TOML.

USAGE:
    toml get [FLAGS] <path> <query>

FLAGS:
    -h, --help           Prints help information
        --output-toml    Print as a TOML fragment (default: print as JSON)
    -r, --raw            Print strings raw, not as JSON
    -V, --version        Prints version information

ARGS:
    <path>     Path to the TOML file to read
    <query>    Query within the TOML data (e.g. `dependencies.serde`, `foo[0].bar`)
```

### `toml set`

```
$ toml set --help
toml-set 0.2.3
Edit the file to set some data (currently, just print modified version)

USAGE:
    toml set <path> <query> <value-str>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>         Path to the TOML file to read
    <query>        Query within the TOML data (e.g. `dependencies.serde`, `foo[0].bar`)
    <value-str>    String value to place at the given spot (bool, array, etc. are TODO)
```
