# flipper

A very simple example contract to illustrate a new execution engine that is developed for the Casper Network.

> [!WARNING]
> This is a work in progress and this contract is prepared for informational purposes. The code, APIs, and tooling may change at any time without notice.

# Run

You can run unit tests:

```sh
cargo test
```

# Tools

You need custom tools installed from a git repo.

```sh
# cargo-casper
cargo install --git https://github.com/casper-network/casper-node --branch dev cargo-casper
```

# Building the contract

```sh
cargo-casper build
```

This builds both .wasm and .json schema files.

# Contract schema

To obtain schema for this contract you can run:

```
cargo-casper build-schema
```
