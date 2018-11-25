all: install build test

install:
	git submodule update --init --recursive
	curl https://sh.rustup.rs -sSf | sh
	rustup target add wasm32-unknown-unknown
	rustup install nightly
	rustup default nightly
	cargo install --force wasm-gc
	cargo install --force bindgen
	cargo install --force wasm-bindgen
	cd examples/tictactoe_ui && yarn install

build:
	cargo build --release --target=wasm32-unknown-unknown -vv

build_ui: build target/wasm32-unknown-unknown/release/tictactoe_ui_gc_opt_wat.wasm
	mkdir -p docs/tictactoe
	wasm-bindgen \
		target/wasm32-unknown-unknown/release/tictactoe_ui.wasm \
		--out-dir docs/tictactoe
	yarn --cwd examples/tictactoe_ui build

serve_ui: build_ui
	yarn --cwd examples/tictactoe_ui serve

test:
	cargo test

docs:
	rm -Rf target/doc
	cargo doc \
		--all \
		--exclude addressbook \
		--exclude eosio_token \
		--exclude hello \
		--exclude hello_bare \
		--exclude tictactoe \
		--exclude tictactoe_ui \
		--no-deps
	cp -rf target/doc/* docs/

lint:
	touch eosio/src/lib.rs
	touch eosio_sys/src/lib.rs
	touch eosio_macros/src/lib.rs
	cargo clippy

clean:
	rm -Rf target

docker-down:
	docker-compose down
	docker volume rm -f nodeos-data-volume
	docker volume rm -f keosd-data-volume

docker: docker-down
	docker volume create --name=nodeos-data-volume
	docker volume create --name=keosd-data-volume
	docker-compose up

publish:
	cd eosio_sys && cargo publish
	cd eosio_macros && cargo publish
	cd eosio && cargo publish

CLEOS := docker-compose exec keosd cleos --url http://nodeosd:8888 --wallet-url http://127.0.0.1:8900
PUBKEY := EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY := 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3

wallet:
	$(CLEOS) wallet create --to-console
	$(CLEOS) wallet import --private-key $(PRIVKEY)

%_account:
	$(CLEOS) create account eosio $* $(PUBKEY) $(PUBKEY)

accounts: hello_account tictactoe_account alice_account bob_account carol_account dan_account addressbook_account eosiotkncpp_account hellobare_account hellocpp_account eosio.token_account

%_permissions:
	$(CLEOS) set account permission $* active \
		'{"threshold": 1,"keys": [{"key": "'$(PUBKEY)'","weight": 1}],"accounts": [{"permission":{"actor":"$*","permission":"eosio.code"},"weight":1}]}' owner

%_gc.wasm: %.wasm
	wasm-gc $*.wasm $*_gc.wasm

%_gc_opt.wasm: %_gc.wasm
	wasm-opt --fuzz-exec --output $*_gc_opt.wasm -Oz $*_gc.wasm

%_gc_opt.wat: %_gc_opt.wasm
	wasm2wat $*_gc_opt.wasm -o $*_gc_opt.wat --generate-names

%_gc_opt_wat.wasm: %_gc_opt.wat
	wat2wasm $*_gc_opt.wat -o $*_gc_opt_wat.wasm

%_example: target/wasm32-unknown-unknown/release/%_gc_opt_wat.wasm
	$(CLEOS) set abi $(subst _,,$*) /mnt/dev/examples/$*/$*.abi.json
	$(CLEOS) set code $(subst _,,$*) /mnt/dev/release/$*_gc_opt.wasm

eosio_token: target/wasm32-unknown-unknown/release/eosio_token_gc_opt_wat.wasm
	$(CLEOS) set abi eosio.token /mnt/dev/project/crates/eosio_token/eosio_token.abi.json
	$(CLEOS) set code eosio.token /mnt/dev/release/eosio_token_gc_opt.wasm

examples: addressbook_example eosio_token hello_example hello_bare_example tictactoe_example

say_hi:
	$(CLEOS) push action hello hi '["contributor"]' -p 'hello@active'

say_hi_bare:
	$(CLEOS) push action hellobare hi '["contributor"]' -p 'hello@active'

hello_cpp: examples/hello/cpp/hello_gc_opt_wat.wasm
	$(CLEOS) set abi hellocpp /mnt/dev/examples/hello/hello.abi.json
	$(CLEOS) set code hellocpp /mnt/dev/examples/hello/cpp/hello_gc_opt.wasm

say_hi_cpp:
	$(CLEOS) push action hellocpp hi '["contributor"]' -p 'hello@active'

simulate_tictactoe: create_game make_moves restart_game make_moves close_game

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

eosio_token_cpp: examples/eosio_token/cpp/eosio.token_gc_opt_wat.wasm
	$(CLEOS) set abi eosiotkncpp /mnt/dev/examples/eosio_token/eosio_token.abi.json
	$(CLEOS) set code eosiotkncpp /mnt/dev/examples/eosio_token/cpp/eosio.token_gc_opt.wasm

create_token:
	$(CLEOS) push action eosio.token create '["alice","1000.00 TGFT"]' -p 'eosio.token@active'

create_token_cpp:
	$(CLEOS) push action eosiotkncpp create '["alice","1000.00 TGFT"]' -p 'eosiotkncpp@active'

issue_tokens:
	$(CLEOS) push action eosio.token issue '["alice","1.00 TGFT","here you go"]' -p 'alice@active'
	$(CLEOS) push action eosio.token issue '["bob","1.00 TGFT","here you go"]' -p 'alice@active'
	#$(CLEOS) push action eosio.token issue '["carol","1.00 TGFT","here you go"]' -p 'alice@active'

issue_tokens_cpp:
	$(CLEOS) push action eosiotkncpp issue '["alice","1.00 TGFT","here you go"]' -p 'alice@active'
	$(CLEOS) push action eosiotkncpp issue '["bob","1.00 TGFT","here you go"]' -p 'alice@active'
	#$(CLEOS) push action eosio.token issue '["carol","1.00 TGFT","here you go"]' -p 'alice@active'

transfer_tokens:
	$(CLEOS) push action eosio.token transfer '["alice","bob","1.00 TGFT","here you go"]' -p 'alice@active'
	$(CLEOS) push action eosio.token transfer '["bob","alice","0.05 TGFT","here you go"]' -p 'bob@active'

transfer_tokens_cpp:
	$(CLEOS) push action eosiotkncpp transfer '["alice","bob","1.00 TGFT","here you go"]' -p 'alice@active'
	$(CLEOS) push action eosiotkncpp transfer '["bob","alice","0.05 TGFT","here you go"]' -p 'bob@active'

get_currency_stats:
	$(CLEOS) get table eosio.token TGFT stat

get_token_accounts:
	$(CLEOS) get table eosio.token alice accounts
	$(CLEOS) get table eosio.token bob accounts

add_address:
	$(CLEOS) push action addressbook add '["dan","Dan","Larimer","1 EOS Way","Blacksburg","VA",24062]' -p 'dan@active'
	$(CLEOS) push action addressbook add '["alice","Alice","Doe","1 EOS Way","Blacksburg","VA",24061]' -p 'alice@active'
	$(CLEOS) push action addressbook add '["bob","Bob","Smith","1 EOS Way","Blacksburg","VA",24060]' -p 'bob@active'

update_address:
	$(CLEOS) push action addressbook update '["dan","Dan","Larimer","1 EOS Way","Blacksburg","VA",24060]' -p 'dan@active'
	$(CLEOS) push action addressbook update '["alice","Alice","Doe","1 EOS Way","Blacksburg","VA",24061]' -p 'alice@active'
	$(CLEOS) push action addressbook update '["bob","Bob","Smith","1 EOS Way","Blacksburg","VA",24062]' -p 'bob@active'

remove_address:
	$(CLEOS) push action addressbook remove '["alice"]' -p 'alice@active'
	$(CLEOS) push action addressbook remove '["bob"]' -p 'bob@active'
	$(CLEOS) push action addressbook remove '["dan"]' -p 'dan@active'

like_address:
	$(CLEOS) push action addressbook like '["dan"]' -p 'alice@active'
	$(CLEOS) push action addressbook like '["dan"]' -p 'bob@active'
	$(CLEOS) push action addressbook like '["dan"]' -p 'carol@active'

likezip_address:
	$(CLEOS) push action addressbook likezip '[24060]' -p 'alice@active'
	$(CLEOS) push action addressbook likezip '[24061]' -p 'bob@active'
	$(CLEOS) push action addressbook likezip '[24062]' -p 'carol@active'

get_addresses:
	$(CLEOS) get table addressbook addressbook address

.PHONY: install build test clean docker wallet accounts hello docs
.SECONDARY:
