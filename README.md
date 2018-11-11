# rustlang-first-try

## Install / Build

```shell
cargo build
```

## Run

```shell
cargo run
```

## Test

```shell
cargo test
```

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
