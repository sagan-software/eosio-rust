use eosio::*;
use eosio_cdt::*;

const BOARD_WIDTH: u16 = 3;
const BOARD_HEIGHT: u16 = 3;
const BOARD_AREA: usize = (BOARD_WIDTH * BOARD_HEIGHT) as usize;

const EMPTY: u8 = 0;
const HOST: u8 = 1;
const CHALLENGER: u8 = 2;
const DRAW: u8 = 3;

#[eosio::table("game")]
struct Game {
    host: AccountName,
    #[eosio(primary_key)]
    challenger: AccountName,
    turn: u8,
    winner: u8,
    board: [u8; BOARD_AREA],
}

#[eosio::action]
fn create(host: AccountName, challenger: AccountName) {
    require_auth(host);
    assert!(
        challenger != host,
        "challenger shouldn't be the same as host",
    );
    assert!(is_account(challenger), "challenger account doesn't exist");

    let code = current_receiver();
    let table = Game::table(code, host);

    assert!(!table.exists(challenger), "game already exists");

    let game = Game::new(host, challenger);

    table.emplace(host, game).expect("write");
}

#[eosio::action]
fn restart(host: AccountName, challenger: AccountName, by: u8) {
    assert!(
        by == HOST || by == CHALLENGER,
        "by must be either 1 (HOST) or 2 (CHALLENGER)",
    );
    require_auth(if by == HOST { host } else { challenger });

    let code = current_receiver();
    let table = Game::table(code, host);
    let cursor = table.find(challenger).expect("game doesn't exist");
    let mut game = cursor.get().expect("read");

    game.restart();

    cursor.modify(Payer::Same, game).expect("write");
}

#[eosio::action]
fn close(host: AccountName, challenger: AccountName) {
    require_auth(host);

    let code = current_receiver();
    let table = Game::table(code, host);
    let cursor = table.find(challenger).expect("game doesn't exist");

    cursor.erase().expect("read");
}

#[eosio::action("makemove")]
fn make_move(
    host: AccountName,
    challenger: AccountName,
    by: u8,
    row: u16,
    col: u16,
) {
    assert!(
        by == HOST || by == CHALLENGER,
        "by must be either 1 (HOST) or 2 (CHALLENGER)",
    );
    require_auth(if by == HOST { host } else { challenger });

    // Check if game exists
    let code = current_receiver();
    let table = Game::table(code, host);
    let cursor = table.find(challenger).expect("game doesn't exist");

    let mut game = cursor.get().expect("failed to read game");

    assert!(game.winner == EMPTY, "the game has ended!");
    assert!(
        (by == HOST && game.turn == HOST)
            || (by == CHALLENGER && game.turn == CHALLENGER),
        "it's not your turn yet!",
    );

    assert!(game.is_valid_move(row, col), "not a valid movement!");

    game.make_move(row, col);
    cursor.modify(Payer::Same, game).expect("write");
}

eosio::abi!(create, restart, close, make_move);

impl Game {
    fn new(host: AccountName, challenger: AccountName) -> Self {
        Self {
            host,
            challenger,
            turn: HOST,
            winner: EMPTY,
            board: [EMPTY; BOARD_AREA],
        }
    }

    fn restart(&mut self) {
        self.board = [EMPTY; BOARD_AREA];
        self.turn = HOST;
        self.winner = EMPTY;
    }

    fn make_move(&mut self, row: u16, col: u16) {
        let loc = Self::movement_location(row, col);
        // Fill the cell, 1 for host, 2 for challenger
        for (i, cell) in self.board.iter_mut().enumerate() {
            if i == loc {
                if self.turn == HOST {
                    *cell = HOST;
                    self.turn = CHALLENGER;
                } else {
                    *cell = CHALLENGER;
                    self.turn = HOST;
                }
                break;
            }
        }
        self.winner = self.get_winner();
    }

    fn get_winner(&self) -> u8 {
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
                (Some(&HOST), Some(&HOST), Some(&HOST)) => {
                    return HOST;
                }
                (Some(&CHALLENGER), Some(&CHALLENGER), Some(&CHALLENGER)) => {
                    return CHALLENGER;
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
            DRAW
        } else {
            EMPTY
        }
    }

    fn is_valid_move(&self, row: u16, col: u16) -> bool {
        let loc = Self::movement_location(row, col);
        match self.board.get(loc) {
            Some(&cell) => {
                col < BOARD_WIDTH && row < BOARD_HEIGHT && cell == EMPTY
            }
            None => false,
        }
    }

    fn movement_location(row: u16, col: u16) -> usize {
        (row * BOARD_WIDTH + col) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_move() {
        let mut game = Game::new(n!("player1").into(), n!("player2").into());
        assert_eq!(game.is_valid_move(0, 0), true);
        assert_eq!(game.is_valid_move(1, 1), true);
        assert_eq!(game.is_valid_move(2, 2), true);
        assert_eq!(game.is_valid_move(3, 3), false);
        game.make_move(0, 0);
        assert_eq!(game.is_valid_move(0, 0), false);
    }

    #[test]
    fn test_make_move() {
        let mut game = Game::new(n!("player1").into(), n!("player2").into());
        assert_eq!(game.board[0], EMPTY);
        game.make_move(0, 0);
        assert_eq!(game.board[0], HOST);
        assert_eq!(game.board[1], EMPTY);
        game.make_move(0, 1);
        assert_eq!(game.board[1], CHALLENGER);
    }

    #[test]
    fn test_get_winner() {
        let mut game = Game::new(n!("player1").into(), n!("player2").into());
        assert_eq!(game.winner, EMPTY);
        game.make_move(0, 0);
        assert_eq!(game.winner, EMPTY);
        game.make_move(1, 0);
        assert_eq!(game.winner, EMPTY);
        game.make_move(0, 1);
        assert_eq!(game.winner, EMPTY);
        game.make_move(1, 1);
        assert_eq!(game.winner, EMPTY);
        game.make_move(0, 2);
        assert_eq!(game.winner, HOST);
    }

    #[test]
    fn test_restart() {
        let mut game = Game::new(n!("player1").into(), n!("player2").into());
        assert_eq!(game.winner, EMPTY);
        game.make_move(0, 0);
        assert_eq!(game.winner, EMPTY);
        game.make_move(1, 0);
        assert_eq!(game.winner, EMPTY);
        game.make_move(0, 1);
        assert_eq!(game.winner, EMPTY);
        game.make_move(1, 1);
        assert_eq!(game.winner, EMPTY);
        game.make_move(0, 2);
        assert_eq!(game.winner, HOST);
        assert_eq!(game.turn, CHALLENGER);
        game.restart();
        assert_eq!(game.winner, EMPTY);
        assert_eq!(game.turn, HOST);
        assert_eq!(game.board, [0u8; BOARD_AREA]);
    }
}
