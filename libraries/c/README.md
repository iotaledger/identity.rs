
# IOTA Identity C lib

> This is an experimental C binding to IOTA's Identity API.


## Development

It uses [cbindgen](https://github.com/eqrion/cbindgen) to creates C/C++11 headers for Rust libraries which expose a public C API.

To install cbindgen, you just need to run

```bash
cargo install --force cbindgen
```

### 1. Build new header file
```bash
cbindgen --config cbindgen.toml --crate identity-c --output identity.h
```

This produces a header file for C++. For C, add the --lang c switch.
cbindgen also supports generation of Cython bindings, use --lang [cython](https://cython.org/) for that.

### 2. Build the rust code
```bash
cargo build --release
```

### 3. Compile and run example
```bash
gcc examples/example.c -o examples/target/example -L. target/debug/libidentity_c.a 
```

### 4. Run the programm

```bash
./examples/target/example
```