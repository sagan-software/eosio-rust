#![no_std]
#![feature(alloc, proc_macro_non_items)]

#[macro_use]
extern crate alloc;
extern crate eosio;
extern crate eosio_bytes;

use alloc::vec::Vec;
use eosio::prelude::*;

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

    let table = Game::table(host);

    eosio_assert!(!table.exists(challenger), "game already exists");

    let game = Game {
        challenger,
        host,
        turn: host,
        winner: None,
        board: vec![0; BOARD_AREA],
    };
    table.emplace(host, game);
}

#[eosio_action]
fn restart(challenger: AccountName, host: AccountName, by: AccountName) {
    require_auth(by);

    let table = Game::table(host);
    let itr = table.find(challenger);
    eosio_assert!(!table.is_end(itr), "game doesn't exists");

    let mut game = table.get(itr).unwrap();

    eosio_assert!(
        by == game.host || by == game.challenger,
        "this is not your game!"
    );

    game.board = vec![0; BOARD_AREA];
    game.turn = host;
    game.winner = None;

    table.modify(itr, host, game);
}

#[eosio_action]
fn close(challenger: AccountName, host: AccountName) {
    require_auth(host);

    let table = Game::table(host);
    let itr = table.find(challenger);
    eosio_assert!(!table.is_end(itr), "game doesn't exists");

    table.erase(itr);
}

#[eosio_action]
fn makemove(challenger: AccountName, host: AccountName, by: AccountName, row: u16, col: u16) {
    require_auth(by);

    let table = Game::table(host);
    let itr = table.find(challenger);
    eosio_assert!(!table.is_end(itr), "game doesn't exists");

    let mut game = table.get(itr).unwrap();

    eosio_assert!(
        by == game.host || by == game.challenger,
        "this is not your game!"
    );
    eosio_assert!(game.winner.is_none(), "the game has ended!");
    eosio_assert!(by == game.turn, "it's not your turn yet!");
    eosio_assert!(game.is_valid_move(row, col), "not a valid movement!");

    let loc = movement_location(row, col);
    if game.turn == host {
        game.board[loc] = 1;
        game.turn = challenger;
    } else {
        game.board[loc] = 2;
        game.turn = host;
    };

    game.update_winner();

    table.modify(itr, host, game);
}

eosio_abi!(create, restart, close, makemove);

#[derive(TableRow, Readable, Writeable)]
struct Game {
    #[primary]
    challenger: AccountName,
    host: AccountName,
    turn: AccountName,
    winner: Option<AccountName>,
    board: Vec<u8>,
}

impl Game {
    fn table(host: AccountName) -> Table<Game> {
        let code = current_receiver();
        Table::new(code, host, n!(games))
    }

    fn update_winner(&mut self) {
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
            let first = self.board[tiles[0]];
            let second = self.board[tiles[1]];
            let third = self.board[tiles[2]];
            if first != 0 && first == second && second == third {
                self.winner = if first == 1 {
                    Some(self.host)
                } else {
                    Some(self.challenger)
                };
                return;
            }
        }
        let mut taken_tiles = 0;
        for tile in self.board.iter() {
            if *tile != 0 {
                taken_tiles += 1;
            }
        }
        if taken_tiles == 9 {
            self.winner = Some(n!(draw).into());
            return;
        }
        self.winner = None;
    }

    fn is_valid_move(&self, row: u16, col: u16) -> bool {
        let loc = movement_location(row, col);
        col < BOARD_WIDTH && row < BOARD_HEIGHT && is_empty_cell(self.board[loc])
    }
}

fn movement_location(row: u16, col: u16) -> usize {
    (row * BOARD_WIDTH + col) as usize
}

fn is_empty_cell(cell: u8) -> bool {
    cell == 0
}
