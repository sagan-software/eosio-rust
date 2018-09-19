#![no_std]
#![feature(alloc, proc_macro_non_items)]

#[macro_use]
extern crate alloc;
extern crate eosio;

use alloc::vec::Vec;
use eosio::prelude::*;

#[eosio_action]
fn create(challenger: AccountName, host: AccountName) {
    require_auth(&host);
    eosio_assert!(
        challenger != host,
        "challenger shouldn't be the same as host"
    );

    let table = Game::table(host.clone());

    eosio_assert!(!table.exists(challenger.as_u64()), "game already exists");

    let game = Game {
        challenger,
        host: host.clone(),
        turn: host.clone(),
        winner: n!(none).into(),
        board: Vec::new(),
    };
    table.emplace(host, game);
}

#[eosio_action]
fn restart(challenger: AccountName, host: AccountName, by: AccountName) {
    require_auth(&by);

    let table = Game::table(host.clone());
    let itr = table.find(challenger.as_u64());
    eosio_assert!(!table.is_end(itr), "game doesn't exists");

    let mut game = table.get(itr).unwrap();

    eosio_assert!(
        by == game.host || by == game.challenger,
        "this is not your game!"
    );

    game.board = vec![0; BOARD_AREA as usize];
    game.turn = host.clone();
    game.winner = n!(none).into();

    table.modify(itr, host, game);
}

#[eosio_action]
fn close(challenger: AccountName, host: AccountName) {
    require_auth(&host);

    let table = Game::table(host.clone());
    let itr = table.find(challenger.as_u64());
    eosio_assert!(!table.is_end(itr), "game doesn't exists");

    table.erase(itr);
}

#[eosio_action]
fn makemove(challenger: AccountName, host: AccountName, by: AccountName, row: u16, col: u16) {
    require_auth(&by);

    let table = Game::table(host.clone());
    let itr = table.find(challenger.as_u64());
    eosio_assert!(!table.is_end(itr), "game doesn't exists");

    let mut game = table.get(itr).unwrap();
    game.board.resize(BOARD_AREA as usize, 0);

    // Check if this game hasn't ended yet
    eosio_assert!(
        game.winner == AccountName::new(n!(none)),
        "the game has ended!"
    );
    // Check if this game belongs to the action sender
    eosio_assert!(
        by == game.host || by == game.challenger,
        "this is not your game!"
    );
    // Check if this is the  action sender's turn
    eosio_assert!(by == game.turn, "it's not your turn yet!");

    // Check if user makes a valid movement
    eosio_assert!(
        is_valid_movement(row, col, &game.board),
        "not a valid movement!"
    );

    let (cell_value, turn) = if game.turn == host {
        (1, game.challenger.clone())
    } else {
        (2, game.host.clone())
    };

    let loc = movement_location(row, col);
    game.board[loc] = cell_value;
    game.turn = turn;
    game.update_winner();

    table.modify(itr, host, game);
}

eosio_abi!(create, restart, close, makemove);

const BOARD_WIDTH: u16 = 3;
const BOARD_HEIGHT: u16 = 3;
const BOARD_AREA: u16 = BOARD_WIDTH * BOARD_HEIGHT;

#[derive(TableRow, Readable, Writeable)]
struct Game {
    #[primary_key]
    challenger: AccountName,
    host: AccountName,
    turn: AccountName,
    winner: AccountName,
    board: Vec<u8>,
}

impl Game {
    fn table(host: AccountName) -> Table<Game> {
        let code = current_receiver();
        Table::new(code, host, n!(games).into())
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
                    self.host.clone()
                } else {
                    self.challenger.clone()
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
            self.winner = n!(draw).into();
            return;
        }
        self.winner = n!(none).into();
    }
}

fn movement_location(row: u16, col: u16) -> usize {
    (row * BOARD_WIDTH + col) as usize
}

fn is_empty_cell(cell: u8) -> bool {
    cell == 0
}

fn is_valid_movement(row: u16, col: u16, board: &[u8]) -> bool {
    let loc = movement_location(row, col);
    col < BOARD_WIDTH && row < BOARD_HEIGHT && is_empty_cell(board[loc])
}
