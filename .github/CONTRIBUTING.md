# Contributor Quickstart

Want to help? Great! Here's how you can get started:

```bash
git clone git@github.com:sagan-software/rust-eos.git
cd rust-eos
make
```

That will clone the repo, install Rust, build crates, and run unit tests.

### Optional Dependencies

Please be sure to install all the [optional dependencies](README.md#optional-dependencies) listed in the README.

### Hello World

To test out the `hello` example on-chain, first startup a local node with Docker:

```bash
make docker
```

Then in a new terminal window:

```bash
make wallet
make accounts
make examples
make say_hi
```

If all goes well you should see "Hello, contributor" in the console.
