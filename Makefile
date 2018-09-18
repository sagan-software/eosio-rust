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

accounts: hello_account tictactoe_account alice_account bob_account

%_gc.wasm: %.wasm
	wasm-gc $*.wasm $*_gc.wasm

%_gc_opt.wasm: %_gc.wasm
	wasm-opt --fuzz-exec --output $*_gc_opt.wasm -Oz $*_gc.wasm

%_gc_opt.wat: %_gc_opt.wasm
	wasm2wat $*_gc_opt.wasm -o $*_gc_opt.wat --generate-names

%_gc_opt_wat.wasm: %_gc_opt.wat
	wat2wasm $*_gc_opt.wat -o $*_gc_opt_wat.wasm

%_example: target/wasm32-unknown-unknown/release/%_gc_opt_wat.wasm
	$(CLEOS) set abi $* /mnt/dev/examples/$*/$*.abi
	$(CLEOS) set code $* /mnt/dev/release/$*_gc_opt_wat.wasm

say_hi:
	$(CLEOS) push action hello hi '["contributor","tester"]' -p 'hello@active'

create_game:
	$(CLEOS) push action tictactoe create '["bob","alice"]' -p 'alice@active'

restart_game:
	$(CLEOS) push action tictactoe restart '["bob","alice","alice"]' -p 'alice@active'

close_game:
	$(CLEOS) push action tictactoe close '["bob","alice"]' -p 'alice@active'

make_moves: make_moves_alice make_moves_bob
	$(CLEOS) push action tictactoe makemove '["bob","alice","alice",0,1]' -p 'alice@active'
	$(CLEOS) push action tictactoe makemove '["bob","alice","bob",1,1]' -p 'bob@active'
	$(CLEOS) push action tictactoe makemove '["bob","alice","alice",0,2]' -p 'alice@active'

make_moves_alice:
	$(CLEOS) push action tictactoe makemove '["bob","alice","alice",0,0]' -p 'alice@active'

make_moves_bob:
	$(CLEOS) push action tictactoe makemove '["bob","alice","bob",1,0]' -p 'bob@active'

get_games_%:
	$(CLEOS) get table tictactoe $* games

.PHONY: install build test clean docker wallet accounts hello
.SECONDARY: