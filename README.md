# rustlang-first-try

[![Build Status](https://travis-ci.org/topheman/rustlang-first-try.svg?branch=master)](https://travis-ci.org/topheman/rustlang-first-try)
[![Documentation](https://img.shields.io/badge/documentation-online-blue.svg)](https://topheman.github.io/rustlang-first-try/rustlang_first_try)

## Install / Build

```shell
cargo build
```

## Run

```shell
cargo run
```

- `cargo run` / `cargo run help`: displays help section
- `cargo run hello`: simple hello worlds

## Test

```shell
cargo test
```

## Documentation

The documentation is generated on Travis CI with `cargo doc` and then published on [github pages](https://topheman.github.io/rustlang-first-try/rustlang_first_try).

## Notes

### Manage dependencies

If you are used to `npm install my-package`, `cargo` doesn't work like this out of the box. `cargo install my-package` will only install global binaries.

To be able to install crates to your local project (and update your `Cargo.toml`), you'll need [cargo-edit](https://github.com/killercup/cargo-edit).

```shell
cargo install cargo-edit
```

Then you'll be able to:

- `cargo add my-package` (optinally with flags `--dev` or `--build`)
- `cargo rm my-package`
- `cargo upgrade my-package`
