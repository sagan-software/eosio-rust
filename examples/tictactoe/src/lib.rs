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

    // Check if game already exists
    let end = unsafe { ::eosio::sys::db::db_end_i64(n!(tictactoe), host.as_u64(), n!(games)) };
    let itr = unsafe {
        ::eosio::sys::db::db_find_i64(n!(tictactoe), host.as_u64(), n!(games), challenger.as_u64())
    };
    eosio_assert!(itr == end, "game already existss");

    let game = Game {
        challenger,
        host: host.clone(),
        turn: host,
        winner: AccountName::new(n!(none)),
        board: Vec::new(),
    };
    game.save();
}

#[eosio_action]
fn restart(challenger: AccountName, host: AccountName, by: AccountName) {
    require_auth(&by);

    // Check if game exists
    let end = unsafe { ::eosio::sys::db::db_end_i64(n!(tictactoe), host.as_u64(), n!(games)) };
    let itr = unsafe {
        ::eosio::sys::db::db_find_i64(n!(tictactoe), host.as_u64(), n!(games), challenger.as_u64())
    };
    eosio_assert!(itr != end, "game doesn't exists");

    let mut bytes = [0u8; 1000];
    let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
    unsafe {
        ::eosio::sys::db::db_get_i64(itr, ptr, 1000);
    }

    let (mut game, _) = Game::read(&bytes).unwrap();

    eosio_assert!(
        by == game.host || by == game.challenger,
        "this is not your game!"
    );

    game.board = vec![0; BOARD_AREA as usize];
    game.turn = host;
    game.winner = AccountName::new(n!(none));
    game.save();
}

#[eosio_action]
fn close(challenger: AccountName, host: AccountName) {
    require_auth(&host);

    // Check if game exists
    let end = unsafe { ::eosio::sys::db::db_end_i64(n!(tictactoe), host.as_u64(), n!(games)) };
    let itr = unsafe {
        ::eosio::sys::db::db_find_i64(n!(tictactoe), host.as_u64(), n!(games), challenger.as_u64())
    };
    eosio_assert!(itr != end, "game doesn't exists");

    unsafe {
        ::eosio::sys::db::db_remove_i64(itr);
    }
}

#[eosio_action]
fn makemove(challenger: AccountName, host: AccountName, by: AccountName, row: u16, col: u16) {
    require_auth(&by);

    // Check if game exists
    let end = unsafe { ::eosio::sys::db::db_end_i64(n!(tictactoe), host.as_u64(), n!(games)) };
    let itr = unsafe {
        ::eosio::sys::db::db_find_i64(n!(tictactoe), host.as_u64(), n!(games), challenger.as_u64())
    };
    eosio_assert!(itr != end, "game doesn't exists");

    let mut bytes = vec![0u8; 1000];
    let ptr: *mut c_void = &mut bytes[..] as *mut _ as *mut c_void;
    unsafe {
        ::eosio::sys::db::db_get_i64(itr, ptr, 1000);
    }

    let (mut game, _) = Game::read(&bytes).unwrap();
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
    game.save();
}

eosio_abi!(create, restart, close, makemove);

const BOARD_WIDTH: u16 = 3;
const BOARD_HEIGHT: u16 = 3;
const BOARD_AREA: u16 = BOARD_WIDTH * BOARD_HEIGHT;

// #[eosio_table]
struct Game {
    challenger: AccountName,
    host: AccountName,
    turn: AccountName,
    winner: AccountName,
    board: Vec<u8>,
}

impl Game {
    fn save(&self) {
        let mut bytes = [0u8; 1000];
        let pos = self.write(&mut bytes).unwrap();
        let ptr: *const c_void = &bytes[..] as *const _ as *const c_void;

        let end =
            unsafe { ::eosio::sys::db::db_end_i64(n!(tictactoe), self.host.as_u64(), n!(games)) };
        let itr = unsafe {
            ::eosio::sys::db::db_find_i64(
                n!(tictactoe),
                self.host.as_u64(),
                n!(games),
                self.challenger.as_u64(),
            )
        };

        if itr != end {
            unsafe {
                ::eosio::sys::db::db_update_i64(itr, self.host.as_u64(), ptr, pos as u32);
            }
        } else {
            unsafe {
                ::eosio::sys::db::db_store_i64(
                    self.host.as_u64(),
                    n!(games),
                    self.host.as_u64(),
                    self.challenger.as_u64(),
                    ptr,
                    pos as u32,
                );
            }
        }
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
            self.winner = AccountName::new(n!(draw));
            return;
        }
        self.winner = AccountName::new(n!(none));
    }
}

impl Readable for Game {
    fn read(bytes: &[u8]) -> Result<(Game, usize), ReadError> {
        let mut pos = 0;
        let (challenger, p) = AccountName::read(&bytes[pos..])?;
        pos += p;
        let (host, p) = AccountName::read(&bytes[pos..])?;
        pos += p;
        let (turn, p) = AccountName::read(&bytes[pos..])?;
        pos += p;
        let (winner, p) = AccountName::read(&bytes[pos..])?;
        pos += p;
        let (board, p) = Vec::<u8>::read(&bytes[pos..])?;
        let game = Game {
            challenger,
            host,
            turn,
            winner,
            board,
        };
        Ok((game, pos))
    }
}

impl Writeable for Game {
    fn write(&self, bytes: &mut [u8]) -> Result<usize, WriteError> {
        let mut pos = 0;
        pos += self.challenger.write(&mut bytes[pos..])?;
        pos += self.host.write(&mut bytes[pos..])?;
        pos += self.turn.write(&mut bytes[pos..])?;
        pos += self.winner.write(&mut bytes[pos..])?;
        pos += self.board.write(&mut bytes[pos..])?;
        Ok(pos)
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
