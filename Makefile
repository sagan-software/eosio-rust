.PHONY: all
all: install build test

.PHONY: install
install:
	git submodule update --init --recursive
	curl https://sh.rustup.rs -sSf | sh
	rustup target add wasm32-unknown-unknown
	rustup install nightly
	rustup default nightly
	cargo install --force wasm-gc
	cargo install --force bindgen

.PHONY: build
build:
	cargo fmt --all
	RUSTFLAGS="-C link-args=-zstack-size=48000" \
	cargo +nightly-2018-11-26 build --release --target=wasm32-unknown-unknown -vv \
		-p addressbook \
		-p hello \
		-p hello_bare \
		-p tictactoe \
		-p eosio_token
	cargo build --release -p eosio_cli -vv

.PHONY: test
test:
	cargo test -p eosio -p eosio_macros -p eosio_sys

.PHONY: docs
docs:
	rm -Rf target/doc
	cargo doc \
		--all \
		--exclude addressbook \
		--exclude eosio_macros \
		--exclude eosio_macros_impl \
		--exclude hello \
		--exclude hello_bare \
		--exclude tictactoe \
		--no-deps
	git worktree remove --force ./gh-pages || exit 0
	git worktree add ./gh-pages gh-pages
	cp -rf target/doc/* gh-pages/
	echo '<meta http-equiv="refresh" content="0;url=eosio/">' > gh-pages/index.html

.PHONY: lint
lint:
	touch crates/eosio/src/lib.rs
	touch crates/eosio_abi/src/lib.rs
	touch crates/eosio_macros/src/lib.rs
	touch crates/eosio_macros_impl/src/lib.rs
	touch crates/eosio_rpc/src/lib.rs
	touch crates/eosio_sys/src/lib.rs
	touch crates/eosio_system/src/lib.rs
	touch crates/eosio_token/src/lib.rs
	cargo clippy

.PHONY: clean
clean:
	rm -Rf target

.PHONY: docker-down
docker-down:
	docker-compose down
	docker volume rm -f nodeos-data-volume
	docker volume rm -f keosd-data-volume

.PHONY: docker-up
docker-up: docker-down
	docker volume create --name=nodeos-data-volume
	docker volume create --name=keosd-data-volume
	docker-compose up

.PHONY: docker-init
docker-init: wallet accounts examples

.PHONY: publish
publish:
	cd eosio_sys && cargo publish
	cd eosio_macros && cargo publish
	cd eosio && cargo publish

CLEOS := docker-compose exec keosd cleos --url http://nodeosd:8888 --wallet-url http://127.0.0.1:8900
PUBKEY := EOS6MRyAjQq8ud7hVNYcfnVPJqcVpscN5So8BhtHuGYqET5GDW5CV
PRIVKEY := 5KQwrPbwdL6PhXujxW37FSSQZ1JiwsST4cqQzDeyXtP79zkvFD3

.PHONY: wallet
wallet:
	$(CLEOS) wallet create --to-console
	$(CLEOS) wallet import --private-key $(PRIVKEY)

%_account:
	$(CLEOS) create account eosio $* $(PUBKEY) $(PUBKEY)

.PHONY: accounts
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

.PHONY: eosio_token
eosio_token: target/wasm32-unknown-unknown/release/eosio_token_gc_opt_wat.wasm
	$(CLEOS) set abi eosio.token /mnt/dev/project/crates/eosio_token/eosio_token.abi.json
	$(CLEOS) set code eosio.token /mnt/dev/release/eosio_token_gc_opt.wasm

.PHONY: examples
examples: addressbook_example eosio_token hello_example hello_bare_example tictactoe_example eosio_token_cpp

.PHONY: say_hi
say_hi:
	$(CLEOS) push action hello hi '["contributor"]' -p 'hello@active'

.PHONY: say_hi_bare
say_hi_bare:
	$(CLEOS) push action hellobare hi '["contributor"]' -p 'hello@active'

.PHONY: hello_cpp
hello_cpp: examples/hello/cpp/hello_gc_opt_wat.wasm
	$(CLEOS) set abi hellocpp /mnt/dev/examples/hello/hello.abi.json
	$(CLEOS) set code hellocpp /mnt/dev/examples/hello/cpp/hello_gc_opt.wasm

.PHONY: say_hi_cpp
say_hi_cpp:
	$(CLEOS) push action hellocpp hi '["contributor"]' -p 'hello@active'

.PHONY: simulate_tictactoe
simulate_tictactoe: create_game make_moves restart_game make_moves close_game

.PHONY: create_game
create_game:
	$(CLEOS) push action tictactoe create '["bob","alice"]' -p 'alice@active'

.PHONY: restart_game
restart_game:
	$(CLEOS) push action tictactoe restart '["bob","alice","alice"]' -p 'alice@active'

.PHONY: close_game
close_game:
	$(CLEOS) push action tictactoe close '["bob","alice"]' -p 'alice@active'

.PHONY: make_moves
make_moves: make_moves_alice make_moves_bob
	$(CLEOS) push action tictactoe makemove '["bob","alice","alice",0,1]' -p 'alice@active'
	$(CLEOS) push action tictactoe makemove '["bob","alice","bob",1,1]' -p 'bob@active'
	$(CLEOS) push action tictactoe makemove '["bob","alice","alice",0,2]' -p 'alice@active'

.PHONY: make_moves_alice
make_moves_alice:
	$(CLEOS) push action tictactoe makemove '["bob","alice","alice",0,0]' -p 'alice@active'

.PHONY: make_moves_bob
make_moves_bob:
	$(CLEOS) push action tictactoe makemove '["bob","alice","bob",1,0]' -p 'bob@active'

get_games_%:
	$(CLEOS) get table tictactoe $* games

.PHONY: eosio_token_cpp
eosio_token_cpp: crates/eosio_token/cpp/eosio.token_gc_opt_wat.wasm
	$(CLEOS) set abi eosiotkncpp /mnt/dev/project/crates/eosio_token/eosio_token.abi.json
	$(CLEOS) set code eosiotkncpp /mnt/dev/project/crates/eosio_token/cpp/eosio.token_gc_opt.wasm

.PHONY: create_token
create_token:
	$(CLEOS) push action eosio.token create '["alice","1000.00 TGFT"]' -p 'eosio.token@active'

.PHONY: create_token_cpp
create_token_cpp:
	$(CLEOS) push action eosiotkncpp create '["alice","1000.00 TGFT"]' -p 'eosiotkncpp@active'

.PHONY: issue_tokens
issue_tokens:
	$(CLEOS) push action eosio.token issue '["alice","1.00 TGFT","here you go"]' -p 'alice@active'
	$(CLEOS) push action eosio.token issue '["bob","1.00 TGFT","here you go"]' -p 'alice@active'
	#$(CLEOS) push action eosio.token issue '["carol","1.00 TGFT","here you go"]' -p 'alice@active'

.PHONY: issue_tokens_cpp
issue_tokens_cpp:
	$(CLEOS) push action eosiotkncpp issue '["alice","1.00 TGFT","here you go"]' -p 'alice@active'
	$(CLEOS) push action eosiotkncpp issue '["bob","1.00 TGFT","here you go"]' -p 'alice@active'
	#$(CLEOS) push action eosio.token issue '["carol","1.00 TGFT","here you go"]' -p 'alice@active'

.PHONY: transfer_tokens
transfer_tokens:
	$(CLEOS) push action eosio.token transfer '["alice","bob","1.00 TGFT","here you go"]' -p 'alice@active'
	$(CLEOS) push action eosio.token transfer '["bob","alice","1.00 TGFT","here you go"]' -p 'bob@active'

.PHONY: transfer_tokens_cpp
transfer_tokens_cpp:
	$(CLEOS) push action eosiotkncpp transfer '["alice","bob","1.00 TGFT","here you go"]' -p 'alice@active'
	$(CLEOS) push action eosiotkncpp transfer '["bob","alice","1.00 TGFT","here you go"]' -p 'bob@active'

.PHONY: retire_tokens
retire_tokens:
	$(CLEOS) push action eosio.token retire '["1.00 TGFT","retire"]' -p 'alice@active'

.PHONY: retire_tokens_cpp
retire_tokens_cpp:
	$(CLEOS) push action eosiotkncpp retire '["1.00 TGFT","retire"]' -p 'alice@active'

.PHONY: close_token
close_token:
	$(CLEOS) push action eosio.token close '["alice","2,TGFT"]' -p 'alice@active'

.PHONY: close_token_cpp
close_token_cpp:
	$(CLEOS) push action eosiotkncpp close '["alice","2,TGFT"]' -p 'alice@active'

.PHONY: open_token
open_token:
	$(CLEOS) push action eosio.token open '["alice","2,TGFT","alice"]' -p 'alice@active'
	$(CLEOS) push action eosio.token open '["alice","2,TGFT","bob"]' -p 'bob@active'

.PHONY: open_token_cpp
open_token_cpp:
	$(CLEOS) push action eosiotkncpp open '["alice","2,TGFT","alice"]' -p 'alice@active'
	$(CLEOS) push action eosiotkncpp open '["alice","2,TGFT","bob"]' -p 'bob@active'


bench_%:
	@echo $*
	@make $* | grep -Po '([\d]+ us)'

.PHONY: bench_rs
bench_rs:
	make -i --quiet bench_create_token
	set -e; i=1; while [ "$$i" -le 100 ]; do \
		printf "\nITERATION $$i:\n"; \
		make --quiet bench_open_token || exit 1; \
		make --quiet bench_issue_tokens || exit 1; \
		make --quiet bench_transfer_tokens || exit 1; \
		make --quiet bench_retire_tokens || exit 1; \
		make --quiet bench_close_token || exit 1; \
		i=$$((i + 1));\
	done

.PHONY: bench_cpp
bench_cpp:
	make -i --quiet bench_create_token_cpp
	set -e; i=1; while [ "$$i" -le 100 ]; do \
		printf "\nITERATION $$i:\n"; \
		make --quiet bench_open_token_cpp || exit 1; \
		make --quiet bench_issue_tokens_cpp || exit 1; \
		make --quiet bench_transfer_tokens_cpp || exit 1; \
		make --quiet bench_retire_tokens_cpp || exit 1; \
		make --quiet bench_close_token_cpp || exit 1; \
		i=$$((i + 1));\
	done

bench:
	make --quiet bench_cpp > bench_cpp.txt
	make --quiet bench_rs > bench_rs.txt

.PHONY: get_currency_stats
get_currency_stats:
	$(CLEOS) get table eosio.token TGFT stat

.PHONY: get_currency_stats_cpp
get_currency_stats_cpp:
	$(CLEOS) get table eosiotkncpp TGFT stat

.PHONY: get_token_accounts
get_token_accounts:
	$(CLEOS) get table eosio.token alice accounts
	$(CLEOS) get table eosio.token bob accounts
	$(CLEOS) get table eosio.token eosio.token accounts

.PHONY: get_token_accounts_cpp
get_token_accounts_cpp:
	$(CLEOS) get table eosiotkncpp alice accounts
	$(CLEOS) get table eosiotkncpp bob accounts
	$(CLEOS) get table eosiotkncpp eosiotkncpp accounts

.PHONY: add_address
add_address:
	$(CLEOS) push action addressbook add '["dan","Dan","Larimer","1 EOS Way","Blacksburg","VA",24062]' -p 'dan@active'
	$(CLEOS) push action addressbook add '["alice","Alice","Doe","1 EOS Way","Blacksburg","VA",24061]' -p 'alice@active'
	$(CLEOS) push action addressbook add '["bob","Bob","Smith","1 EOS Way","Blacksburg","VA",24060]' -p 'bob@active'

.PHONY: update_address
update_address:
	$(CLEOS) push action addressbook update '["dan","Dan","Larimer","1 EOS Way","Blacksburg","VA",24060]' -p 'dan@active'
	$(CLEOS) push action addressbook update '["alice","Alice","Doe","1 EOS Way","Blacksburg","VA",24061]' -p 'alice@active'
	$(CLEOS) push action addressbook update '["bob","Bob","Smith","1 EOS Way","Blacksburg","VA",24062]' -p 'bob@active'

.PHONY: remove_address
remove_address:
	$(CLEOS) push action addressbook remove '["alice"]' -p 'alice@active'
	$(CLEOS) push action addressbook remove '["bob"]' -p 'bob@active'
	$(CLEOS) push action addressbook remove '["dan"]' -p 'dan@active'

.PHONY: like_address
like_address:
	$(CLEOS) push action addressbook like '["dan"]' -p 'alice@active'
	$(CLEOS) push action addressbook like '["dan"]' -p 'bob@active'
	$(CLEOS) push action addressbook like '["dan"]' -p 'carol@active'

.PHONY: likezip_address
likezip_address:
	$(CLEOS) push action addressbook likezip '[24060]' -p 'alice@active'
	$(CLEOS) push action addressbook likezip '[24061]' -p 'bob@active'
	$(CLEOS) push action addressbook likezip '[24062]' -p 'carol@active'

.PHONY: get_addresses
get_addresses:
	$(CLEOS) get table addressbook addressbook address

.PHONY: update_submodules
update_submodules:
	git submodule foreach git pull origin master

.SECONDARY:
