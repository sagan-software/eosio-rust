all: install build test

install:
	git submodule update --init --recursive
	curl https://sh.rustup.rs -sSf | sh
	rustup target add wasm32-unknown-unknown
	rustup install nightly
	rustup default nightly
	cargo install --force wasm-gc
	cargo install --force bindgen

build:
	cargo build --release --target=wasm32-unknown-unknown

test:
	cargo test

clean:
	rm -Rf target

docker:
	docker-compose down
	docker volume rm -f nodeos-data-volume
	docker volume rm -f keosd-data-volume
	docker volume create --name=nodeos-data-volume
	docker volume create --name=keosd-data-volume
	docker-compose up

CLEOS := docker-compose exec keosd cleos --url http://nodeosd:8888 --wallet-url http://127.0.0.1:8900
PUBKEY := EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY := 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3

wallet:
	$(CLEOS) wallet create --to-console
	$(CLEOS) wallet import --private-key $(PRIVKEY)

%_account:
	$(CLEOS) create account eosio $* $(PUBKEY) $(PUBKEY)

accounts: hello_account

%_gc.wasm: %.wasm
	wasm-gc $*.wasm $*_gc.wasm

%_gc_opt.wasm: %_gc.wasm
	wasm-opt --output $*_gc_opt.wasm -Oz $*_gc.wasm

%_gc_opt.wat: %_gc_opt.wasm
	wasm2wat $*_gc_opt.wasm -o $*_gc_opt.wat --generate-names

%_gc_opt_wat.wasm: %_gc_opt.wat
	wat2wasm $*_gc_opt.wat -o $*_gc_opt_wat.wasm

%_example: target/wasm32-unknown-unknown/release/%_gc_opt_wat.wasm
	$(CLEOS) set abi $* /mnt/dev/examples/$*/$*.abi
	$(CLEOS) set code $* /mnt/dev/release/$*_gc_opt_wat.wasm

say_hi:
	$(CLEOS) push action hello hi '["sagan"]' -p 'hello@active'

.PHONY: install build test clean docker wallet accounts hello
.SECONDARY: