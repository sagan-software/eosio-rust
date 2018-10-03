#![feature(proc_macro_non_items)]

extern crate eosio;

use eosio::prelude::*;

const BOARD_WIDTH: u16 = 3;
const BOARD_HEIGHT: u16 = 3;
const BOARD_AREA: usize = (BOARD_WIDTH * BOARD_HEIGHT) as usize;

enum CreateError {
    EqualChallengerAndHost = 10,
    GameAlreadyExists,
    GameWriteError,
}

#[eosio_action]
fn create(challenger: AccountName, host: AccountName) {
    require_auth(host);
    eosio_assert_code(
        challenger != host,
        CreateError::EqualChallengerAndHost as u64,
    );

    let table = Game::table(host);

    eosio_assert_code(
        !table.exists(challenger),
        CreateError::GameAlreadyExists as u64,
    );

    let game = Game {
        challenger,
        host,
        turn: host,
        winner: n!(none).into(),
        board: [0; BOARD_AREA],
    };

    let write_result = table.emplace(host, game);
    eosio_assert_code(write_result.is_ok(), CreateError::GameWriteError as u64);
}

enum RestartError {
    GameNotFound = 20,
    GameReadError,
    NotYourGame,
    GameWriteError,
}

#[eosio_action]
fn restart(challenger: AccountName, host: AccountName, by: AccountName) {
    require_auth(by);

    let table = Game::table(host);
    let itr = table.find(challenger);
    eosio_assert_code(!table.is_end(&itr), RestartError::GameNotFound as u64);

    let mut game = match itr.get() {
        Ok(game) => game,
        Err(_) => {
            eosio_assert_code(false, RestartError::GameReadError as u64);
            return;
        }
    };

    eosio_assert_code(
        by == game.host || by == game.challenger,
        RestartError::NotYourGame as u64,
    );

    game.board = [0; BOARD_AREA];
    game.turn = host;
    game.winner = n!(none).into();

    let write_result = table.modify(&itr, host, game);
    eosio_assert_code(write_result.is_ok(), RestartError::GameWriteError as u64);
}

enum CloseError {
    GameNotFound = 30,
}

#[eosio_action]
fn close(challenger: AccountName, host: AccountName) {
    require_auth(host);

    let table = Game::table(host);
    let itr = table.find(challenger);
    eosio_assert_code(!table.is_end(&itr), CloseError::GameNotFound as u64);

    itr.erase();
}

enum MoveError {
    GameNotFound = 40,
    GameReadError,
    NotYourGame,
    GameEnded,
    NotYourTurn,
    InvalidMove,
    GameWriteError,
}

#[eosio_action]
fn makemove(challenger: AccountName, host: AccountName, by: AccountName, row: u16, col: u16) {
    require_auth(by);

    let table = Game::table(host);
    let itr = table.find(challenger);
    eosio_assert_code(!table.is_end(&itr), MoveError::GameNotFound as u64);

    let mut game = match itr.get() {
        Ok(game) => game,
        Err(_) => {
            eosio_assert_code(false, MoveError::GameReadError as u64);
            return;
        }
    };

    eosio_assert_code(
        by == game.host || by == game.challenger,
        MoveError::NotYourGame as u64,
    );
    eosio_assert_code(game.winner == n!(none).into(), MoveError::GameEnded as u64);
    eosio_assert_code(by == game.turn, MoveError::NotYourTurn as u64);
    eosio_assert_code(
        is_valid_move(row, col, &game.board),
        MoveError::InvalidMove as u64,
    );

    let loc = movement_location(row, col);

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

    let write_result = table.modify(&itr, host, game);
    eosio_assert_code(write_result.is_ok(), MoveError::GameWriteError as u64);
}

eosio_abi!(create, restart, close, makemove);

#[derive(TableRow, Read, Write)]
struct Game {
    #[primary]
    challenger: AccountName,
    host: AccountName,
    turn: AccountName,
    winner: AccountName,
    board: [u8; BOARD_AREA],
}

impl Game {
    fn table(host: AccountName) -> Table<Game> {
        let code = current_receiver();
        Table::new(code, host, n!(games))
    }

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
