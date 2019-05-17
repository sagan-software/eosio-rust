# Installation

## Installing Rust

Install Rust with `rustup` per the [official instructions](https://www.rust-lang.org/en-US/install.html):

```sh
curl https://sh.rustup.rs -sSf | sh
```

This project requires nightly Rust and the `wasm32-unknown-unknown` target to be available, which can be installed with `rustup`:

```sh
rustup install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup default nightly
```

## Installing EOS

An EOS node is required to deploy and test smart contracts. The easiest way to setup a node is to use Docker. See the [official Docker quickstart guide](https://developers.eos.io/eosio-nodeos/docs/docker-quickstart) for instructions.

We recommend using `docker-compose` to manage `nodeos` and `keosd` containers. You can download the official [`docker-compose-latest.yml`](https://raw.githubusercontent.com/EOSIO/eos/master/Docker/docker-compose-latest.yml) file and start the containers using these commands:

```sh
wget https://raw.githubusercontent.com/EOSIO/eos/master/Docker/docker-compose-latest.yml
docker volume create --name=nodeos-data-volume
docker volume create --name=keosd-data-volume
docker-compose -f docker-compose-latest.yml up
```

**Note #1!** If you are using `cleos` within a Docker container, you need to mount your project directory as a volume so that `cleos` can deploy your files. If you're using Docker Compose, add your project directory to the `volumes` section of the `keosd` container like so (abbreviated):

```yaml
services:
  keosd:
    volumes:
      - ./:mnt/dev/project:ro
```

**Note #2!** If you are expecting to see console output from `nodeos` then be sure to add `--contracts-console` to the end of the `nodeosd` command like so (abbreviated):

```yaml
services:
  nodeosd:
    command: /opt/eosio/bin/nodeosd.sh ... --contracts-console
```

## Optional Dependencies

### wasm-gc

[wasm-gc](https://github.com/alexcrichton/wasm-gc) is a command-line tool that removes unused code in WASM files. It can be installed with Cargo:

```sh
cargo install wasm-gc
```

### Binaryen

[Binaryen](https://github.com/WebAssembly/binaryen) comes with a command-line tool called `wasm-opt` that optimizes WASM file sizes. Binaryen can be installed with most system package managers.

### WebAssembly Binary Toolkit (WABT)

[WABT](https://github.com/WebAssembly/wabt) comes with a command-line tool `wasm2wat` that can be used to create textual representations of WASM files, which can be useful for debugging. WABT can be installed with most system package managers.
