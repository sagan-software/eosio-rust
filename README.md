# rust-eos

> ***Libraries for building EOS smart contracts in Rust***


🚧 _**UNDER CONSTRUCTION**_ 🚧

Please excuse the mess, this project is under active development. Documentation, goals, and roadmap coming soon.

## Contributor Quickstart

Want to help? Great! Here's how you can get started:

```bash
git clone git@github.com:sagan-software/rust-eos.git
cd rust-eos
make
```

That will clone the repo, install Rust, build crates, and run unit tests.

### Optional Dependencies

To test out smart contracts on a local node you will need to have these dependencies installed:

1. **Docker**: used to start test nodes and deploy smart contracts.
    - [Docker Community Edition for Mac](https://store.docker.com/editions/community/docker-ce-desktop-mac)
2. **Binaryen**: used to optimize `.wasm` files and generate `.wat` files.
    - For [Homebrew](https://brew.sh/) users: `brew install binaryen`

### Hello World

To test out the `hello` example on-chain, first startup a local node with Docker:

```bash
make docker
```

Then in a new terminal window:

```bash
make wallet
make hello_account
make hello_example
make say_hi
```

If all goes well you should see "Hello, contributor" in the console.