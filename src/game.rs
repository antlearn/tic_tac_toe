//! This a module for setting up a game of Tic-Tac-Toe.
//!
//! It contains an aliased type for the game board, an enum for the game turn, and a struct for the
//! game itself.

/// The game board as an aliased type.
type Board = Vec<Vec<String>>;

/// A turn in the game as an Enum.
#[derive(Debug)]
enum Turn {
    /// The player's turn.
    Player,
    /// The bot's turn.
    Bot,
}

/// The game represented as a struct.
#[derive(Debug)]
pub struct Game {
    /// The game board.
    board: Board,
    /// The current turn of the game.
    current_turn: Turn,
}

impl Game {
    /// Constructs a `Game` object.
    ///
    /// The board will default to a vector of chars indicating the available moves, and the first
    /// turn will default to the player. For fun, a user could randomize the starting player.
    ///
    /// # Example
    ///
    /// ```
    /// use game::Game;
    ///
    /// let game = Game::new();
    /// ```
    pub fn new() -> Game {
        Game {
            board: vec![
                vec![String::from("1"), String::from("2"), String::from("3")],
                vec![String::from("4"), String::from("5"), String::from("6")],
                vec![String::from("7"), String::from("8"), String::from("9")],
            ],
            current_turn: Turn::Player,
        }
    }

    /// Plays the game.
    pub fn play_game(&mut self) {
        let mut finished = false;

        while !finished {
            self.play_turn();

            if self.game_is_won() {
                self.reset();
                finished = Self::player_is_finished();
            }
        }
    }

    /// Plays a turn of the game, getting moves from the player or bot.
    fn play_turn(&mut self) {
        self.print_board();

        let (valid_token, valid_move) = match self.current_turn {
            Turn::Player => (String::from("X"), self.get_player_move()),
            Turn::Bot => (String::from("O"), self.get_bot_move()),
        };

        let (row, col) = Self::move_to_board_location(valid_move);

        self.board[row][col] = valid_token;
        self.current_turn = self.get_next_turn();
    }

    /// Prints the game board
    ///
    /// # Example
    ///
    /// ```
    /// use game::Game;
    ///
    /// let game = Game::new();
    ///
    /// game.print_board()
    ///
    /// // You should see (including blank lines):
    /// //
    /// // +---+---+---+
    /// // | 1 | 2 | 3 |
    /// // +---+---+---+
    /// // | 4 | 5 | 6 |
    /// // +---+---+---+
    /// // | 7 | 8 | 9 |
    /// // +---+---+---+
    /// //
    /// ```
    fn print_board(&self) {
        let seperator = "+---+---+---+";

        println!("\n{}", seperator);

        for row in &self.board {
            println!("| {} |\n{}", row.join(" | "), seperator);
        }

        print!("\n");
    }

    /// Gets move from player.
    fn get_player_move(&self) -> u32 {
        unimplemented!();
    }

    /// Gets move from bot.
    fn get_bot_move(&self) -> u32 {
        unimplemented!();
    }

    /// Determins if move is valid.
    fn is_valid_move(&self, unchecked_move: u32) -> bool {
        unimplemented!();
    }

    /// Turns a move integer into the respective row and column board location.
    fn move_to_board_location(game_move: u32) -> (usize, usize) {
        let row = (game_move - 1) / 3;
        let col = (game_move - 1) % 3;

        (row as usize, col as usize)
    }

    /// Get the next turn, either the player or bot.
    fn get_next_turn(&self) -> Turn {
        match self.current_turn {
            Turn::Player => Turn::Bot,
            Turn::Bot => Turn::Player,
        }
    }

    /// Determines if game is won.
    fn game_is_won(&self) -> bool {
        unimplemented!();
    }

    /// Determines if player wants to play again.
    fn player_is_finished() -> bool {
        unimplemented!();
    }

    /// Resets the game.
    fn reset(&mut self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
