# Changelog for toml-cli

## Unreleased


## 0.2.3

* `toml get` on a missing key no longer panics.  This gives it the same
  behavior as `git config`: print nothing, and exit with failure. (#14)
* Fix query parse error on empty quoted key `""`,
  as in `toml get data.toml 'foo."".bar'`. (#20)


## 0.2.2

* New option `toml get -r` / `--raw`. (#19)


## 0.2.1

* Update `lexical-core` dependency, fixing build on recent Rust toolchains. (#12)
* Update `toml_edit` dependency, fixing parse error on dotted keys. (#2)
* Update dependencies generally.
* Adjust so `cargo fmt` and `cargo clippy` are clean.


## 0.2.0

* **Breaking**: Change query format from `.foo.bar` to `foo.bar`,
  like TOML itself.


## 0.1.0

Initial release.

* `toml get`.
* `toml set`, just printing the modified version.
