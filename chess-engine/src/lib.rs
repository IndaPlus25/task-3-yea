/***
 * Example template for a chess engine.
 * 
 * Course litterature.
 * Course: DD1337 Programming
 * KTH Royal Institute of Technology
 * 
 * Author: Viola Söderlund <violaso@kth.se>
 * License: MIT
 * Latest change: 2025-09-12
 */

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    board: Vec<u8>,
    white_turn: bool,
    //...
}

// these aliases are used to make the code easier to read
// Setting aliases for an empty tile
const EMPTY: u8 = 0;

// Setting aliases for tiles with white pieces
const WHITE_PAWN: u8 = 1;
const WHITE_ROOK: u8 = 2;
const WHITE_KNIGHT: u8 = 3;
const WHITE_BISHOP: u8 = 4;
const WHITE_QUEEN: u8 = 5;
const WHITE_KING: u8 = 6;

// Setting aliases for tiles with black pieces
const BLACK_PAWN: u8 = 11;
const BLACK_ROOK: u8 = 12;
const BLACK_KNIGHT: u8 = 13;
const BLACK_BISHOP: u8 = 14;
const BLACK_QUEEN: u8 = 15;
const BLACK_KING: u8 = 16;

impl Game {
    /// Initialises a new board with pieces. It is a 64 long vector with unsigned integers to represent each tile and piece.
    pub fn new() -> Game {
        let mut board = vec![0; 64];

        for i in 8..16 {board[i] = WHITE_PAWN;}
        for i in 48..56 {board[i] = BLACK_PAWN;}

        board[0] = WHITE_ROOK;
        board[7] = WHITE_ROOK;
        board[1] = WHITE_KNIGHT;
        board[6] = WHITE_KNIGHT;
        board[2] = WHITE_BISHOP;
        board[5] = WHITE_BISHOP;
        board[3] = WHITE_QUEEN;
        board[4] = WHITE_KING;

        board[56] = BLACK_ROOK;
        board[63] = BLACK_ROOK;
        board[57] = BLACK_KNIGHT;
        board[62] = BLACK_KNIGHT;
        board[58] = BLACK_BISHOP;
        board[61] = BLACK_BISHOP;
        board[59] = BLACK_QUEEN;
        board[60] = BLACK_KING;

        Game {
            state: GameState::InProgress,
            board,
            white_turn: true,
        }
    }

    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        None
    }

    /// Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>> {
        if _position.len() != 2 {return None};

        let index = Game::position_to_int(_position).unwrap();
        let piece = self.board[index];

        let mut possible_moves: Vec<String> = Vec::new();

        match piece % 10 {
                0 => return None,
                //1 => ,
                2 => possible_moves = self.rook_moves(index, piece),
                //3 => ,
                //4 => ,
                //5 => ,
                //6 => ,
                _ => return None
        }
        Some(possible_moves)
    }

    fn rook_moves(&self, index: usize, piece: u8) -> Vec<String> {

        let mut possible_moves: Vec<String> = Vec::new();

        let mut blocked_up = false;
        let mut blocked_down = false;
        let mut blocked_left = false;
        let mut blocked_right = false;

        for n in 1..8 {
            let target_up = index as isize - 8*n;
            
            if target_up >= 0 && !blocked_up{
                let target_up = target_up as usize;
                let tile = self.board[target_up];

                if tile == 0 {
                    possible_moves.push(Game::int_to_position(target_up));                               
                } else if tile / 2 != piece / 2 {
                    possible_moves.push(Game::int_to_position(target_up));
                    blocked_up = true;
                } else {
                    blocked_up = true;
                }
            }
            
            let target_down = index + 8*n as usize;
            if target_down <= 63 && !blocked_down {
                let tile2 = self.board[target_down];

                if tile2 == 0{
                    possible_moves.push(Game::int_to_position(target_down));                               
                } else if tile2 / 2 != piece / 2 {
                    possible_moves.push(Game::int_to_position(target_down));
                    blocked_down = true;
                } else {
                    blocked_down = true;
                }
            }

            if blocked_up && blocked_down {
                break;
            }
        }
        possible_moves
    }

    fn position_to_int(position: &str) -> Option<usize> {
        let mut characters = position.chars();

        let rank = characters.next()?;
        let file = characters.next()?.to_ascii_lowercase();

        if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
            return None; 
        }

        Some(file as usize * 8 + rank as usize - 1)
    }

    fn int_to_position(index: usize) -> String {

        let file = (b'a' + (index % 8) as u8) as char;
        let rank = (b'1' + (index / 8) as u8) as char;

        format!("{}{}", file, rank)
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        writeln!(f, "")?;
        writeln!(f, "  |:-----------------------:|")?;

        let mut piece: String = String::new();

        for rank in (0..8).rev() {
            write!(f, "{} | ", rank + 1)?;

            for file in 0..8 {
                match self.board[rank * 8 + file] {
                    0 => piece = "*".to_string(),
                    1 => piece = "P".to_string(),
                    2 => piece = "R".to_string(),
                    3 => piece = "Kn".to_string(),
                    4 => piece = "B".to_string(),
                    5 => piece = "Q".to_string(),
                    6 => piece = "K".to_string(),
                    11 => piece = "p".to_string(),
                    12 => piece = "r".to_string(),
                    13 => piece = "kn".to_string(),
                    14 => piece = "b".to_string(),
                    15 => piece = "q".to_string(),
                    16 => piece = "k".to_string(),
                    _ => piece = "?".to_string(),
                }

                write!(f, "{:3}", piece)?;
            }

            writeln!(f, "|")?;

        }

        writeln!(f, "  |:-----------------------:|");
        write!(f, "    ")?;
        for i in 0..8 {
            write!(f, "{:3}", (b'A' + i as u8) as char)?;
        }
        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}