#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::*;

const BOARD_WIDTH: u16 = 3;
const BOARD_HEIGHT: u16 = 3;
const BOARD_AREA: usize = (BOARD_WIDTH * BOARD_HEIGHT) as usize;

#[eosio_action]
fn create(challenger: AccountName, host: AccountName) {
    require_auth(host);
    eosio_assert!(
        challenger != host,
        "challenger shouldn't be the same as host"
    );

    let code = current_receiver();
    let table = Game::table(code, host);

    eosio_assert!(!table.exists(challenger), "game already exists");

    let game = Game {
        challenger,
        host,
        turn: host,
        winner: n!(none).into(),
        board: [0; BOARD_AREA],
    };

    table.emplace(host, game).assert("write");
}

#[eosio_action]
fn restart(challenger: AccountName, host: AccountName, by: AccountName) {
    require_auth(by);

    let code = current_receiver();
    let table = Game::table(code, host);
    let itr = table.find(challenger).assert("game doesn't exist");
    let mut game = itr.get().assert("read");

    eosio_assert!(
        by == game.host || by == game.challenger,
        "this is not your game"
    );

    game.board = [0; BOARD_AREA];
    game.turn = host;
    game.winner = n!(none).into();

    itr.modify(host, game).assert("write");
}

#[eosio_action]
fn close(challenger: AccountName, host: AccountName) {
    require_auth(host);

    let code = current_receiver();
    let table = Game::table(code, host);
    let itr = table.find(challenger).assert("game doesn't exist");

    itr.erase().assert("read");
}

#[eosio_action]
fn makemove(challenger: AccountName, host: AccountName, by: AccountName, row: u16, col: u16) {
    require_auth(by);

    // Check if game exists

    let code = current_receiver();
    let table = Game::table(code, host);
    let itr = table.find(challenger).assert("game doesn't exist");

    let mut game = itr.get().assert("read");

    // Check if this game hasn't ended yet
    eosio_assert!(game.winner == n!(none).into(), "the game has ended!");
    // Check if this game belongs to the action sender
    eosio_assert!(
        by == game.host || by == game.challenger,
        "this is not your game"
    );
    // Check if this is the  action sender's turn
    eosio_assert!(by == game.turn, "it's not your turn yet!");

    // Check if user makes a valid movement
    eosio_assert!(
        is_valid_move(row, col, &game.board),
        "not a valid movement!"
    );

    let loc = movement_location(row, col);

    // Fill the cell, 1 for host, 2 for challenger
    for (i, cell) in game.board.iter_mut().enumerate() {
        if i == loc {
            if game.turn == host {
                *cell = 1;
                game.turn = challenger;
            } else {
                *cell = 2;
                game.turn = host;
            }
            break;
        }
    }
    game.winner = game.get_winner();
    itr.modify(host, game).assert("write");
}

eosio_abi!(create, restart, close, makemove);

#[eosio_table(games)]
struct Game {
    #[primary]
    challenger: AccountName,
    host: AccountName,
    turn: AccountName,
    winner: AccountName,
    board: [u8; BOARD_AREA],
}

impl Game {
    fn get_winner(&self) -> AccountName {
        let wins = [
            // rows
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            // cols
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            // diagonals
            [0, 4, 8],
            [2, 4, 6],
        ];
        for tiles in wins.iter() {
            let first = tiles.get(0).and_then(|&i| self.board.get(i as usize));
            let second = tiles.get(1).and_then(|&i| self.board.get(i as usize));
            let third = tiles.get(2).and_then(|&i| self.board.get(i as usize));
            match (first, second, third) {
                (Some(1), Some(1), Some(1)) => {
                    return self.host;
                }
                (Some(2), Some(2), Some(2)) => {
                    return self.challenger;
                }
                _ => (),
            }
        }
        let mut taken_tiles = 0;
        for tile in self.board.iter() {
            if *tile != 0 {
                taken_tiles += 1;
            }
        }
        if taken_tiles == BOARD_AREA {
            n!(draw).into()
        } else {
            n!(none).into()
        }
    }
}

fn movement_location(row: u16, col: u16) -> usize {
    (row * BOARD_WIDTH + col) as usize
}

fn is_empty_cell(cell: u8) -> bool {
    cell == 0
}

fn is_valid_move(row: u16, col: u16, board: &[u8]) -> bool {
    let loc = movement_location(row, col);
    match board.get(loc) {
        Some(&cell) => col < BOARD_WIDTH && row < BOARD_HEIGHT && is_empty_cell(cell),
        None => false,
    }
}
