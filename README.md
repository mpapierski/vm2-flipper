# flipper

A very simple example contract to illustrate a new execution engine that is developed for the Casper Network.

> [!WARNING]
> This is a work in progress and this contract is prepared for informational purposes. The code, APIs, and tooling may change at any time without notice.

# Run

You can run unit tests:

```sh
cargo test
```

# Building for Wasm

```sh
cargo build --release --target wasm32-unknown-unknown -p vm2-flipper
```

# Install

You need custom tools from a git repo.

```sh
# cargo-casper
cargo install --git https://github.com/mpapierski/casper-node --branch vm2 cargo-casper
```

# Contract schema

To obtain schema for this contract you can run:

```
cargo-casper get-schema -p vm2-flipper
```
