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
        let possible_moves = self.get_possible_moves(_from)?;
        let from_index = Game::position_to_int(_from).expect("The first position was not input correctly");
        let to_index = Game::position_to_int(_to).expect("The second position was not input correctly");

        if possible_moves.contains(&_to.to_ascii_uppercase()) {
            println!("From and to: {}, {}", from_index, to_index);
            println!("The possible moves from {}: {:?}", from_index, possible_moves);
            self.board[to_index] = self.board[from_index];
            self.board[from_index] = 0;
        }
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
        //Check if the position given is 2 characters long and then convert it to it's index number for the board vector
        if _position.len() != 2 {return None};

        let index = Game::position_to_int(_position).expect("Not a valid position on the board");
        let piece = self.board[index];

        let mut possible_moves: Vec<String> = Vec::new();

        //println!("This is what piece % 10 is: {}", piece % 10);

        //A match case to call the correct function for all the possible moves depending on what piece is on the given tile
        match piece % 10 {
                0 => return None,
                1 => possible_moves = self.pawn_moves(index, piece),
                2 => possible_moves = self.sweeping_moves(index, piece, &[-8, 8, -1, 1]),
                3 => possible_moves = self.singular_moves(index, piece, &[-17, 17, -15, 15, -10, 10, -6, 6]),
                4 => possible_moves = self.sweeping_moves(index, piece, &[-9, 9, -7, 7]),
                5 => possible_moves = self.sweeping_moves(index, piece, &[-8, 8, -1, 1, -9, 9, -7, 7]),
                6 => possible_moves = self.singular_moves(index, piece, &[-8, 8, -1, 1, -9, 9, -7, 7]),
                _ => return None
        }
        Some(possible_moves)
    }

    // add function for checking if king is in danger if you move the piece
    // if !king_is_in_danger, check if moving piece leads to king_is_in_danger, else check if piece can stop danger, otherwise return nothing

    // Function to calculate which "directions" the pawn piece should be able to move, depending on if it's a black or white pawn,
    // and check if it can move to take another piece. Afterwards it runs the singular_moves function which is also for the king and knight pieces
    fn pawn_moves(&self, index: usize, piece: u8) -> Vec<String> {
        
        let mut directions: Vec<isize> = Vec::new();

        if piece == 11{
            directions.push(-8);
            if index / 8 == 6 {
                directions.push(-16);
            } 
            if index as isize - 9 >= 0 {
                println!("This is what -9 has: {}", self.board[index - 9]);
                if self.board[index - 9] != 0 && self.board[index - 9] < 11 {
                    println!("-9 is added to directions");
                    directions.push(-9);
                }
            }
            if index as isize - 7 >= 0 {
                println!("This is what -7 has: {}", self.board[index - 7]);
                if self.board[index - 7] != 0 && self.board[index - 7] < 11 {
                    directions.push(-7);
                }
            }
        } 
        else {
            directions.push(8);
            if index / 8 == 1 {
                directions.push(16);
            } 
            if index + 9 <= 63 {
                if self.board[index + 9] > 10 {
                    directions.push(9);
                }
            }
            if index + 7 <= 63 {
                if self.board[index + 7] > 10 {
                    directions.push(7);
                }
            }
        }

        self.singular_moves(index, piece, &directions)
    }

    // Function to calculate if a move would be out of bounds or not, and then send it to possble_move finally determine if it's a possible move
    fn singular_moves(&self, index: usize, piece: u8, directions: &[isize]) -> Vec<String>{
        let mut possible_moves: Vec<String> = Vec::new();

        //Loops around for every move/direction a piece could go and calls possible_move
        for &direction in directions {
            let target = index as isize + direction;

            if target >= 0 && target <= 63 {
                let target = target as usize;
                println!("The move is within range of the board, target is: {}, {}", target, Game::int_to_position(target));

                if !((index as isize % 8  - target as isize % 8).abs() > 2) {
                    self.possible_move(&mut possible_moves, target, piece);
                } 
            }
        }
        possible_moves
    }

    //Function to calculate if a move would be out of bounds or not, and then send it to possble_move finally determine if it's a possible move
    fn sweeping_moves(&self, index: usize, piece: u8, directions: &[isize]) -> Vec<String> {
        let mut possible_moves: Vec<String> = Vec::new();

        //Loops around for every direction a piece could go and keeps looping until that direction gets blocked (by a chess piece or the board edge)
        for &direction in directions {
            for n in 1..8 {
                let target = index as isize + direction*n;

                if target < 0 || target > 63 {
                    break;
                }

                let target = target as usize;

                if (direction == -1 || direction == -9 || direction == 7) && target % 8 == 7 {
                    break;
                }
                if (direction == 1 || direction == 9 || direction == -7) && target % 8 == 0 {
                    break;
                }

                if self.possible_move(&mut possible_moves, target, piece) {
                    break;
                }
            }
        }

        possible_moves
    }

    // Function to check if a move is possible and add it to the final vector of usable moves or if the target destination has an ally piece on it
    fn possible_move(&self, possible_moves: &mut Vec<String>, target: usize, piece: u8) -> bool {
        let tile = self.board[target];

        //println!("This is checking if the move is possible, here is the target piece and using piece: {}, {}", tile / 10, (piece) / 10);
        if tile == 0{
            //println!("The move is possible");
            possible_moves.push(Game::int_to_position(target));                               
        } else if tile / 10 != piece / 10 {
            //println!("The move is possible and takes a piece");
            possible_moves.push(Game::int_to_position(target));
            return true;
        } else {
            //println!("The move is not possible, teammate is here");
            return true;
        }
        false
    }

    // Function to convert a chess position like a1 to the index number of the game chess board vector
    fn position_to_int(position: &str) -> Option<usize> {
        let mut characters = position.chars();

        let file = characters.next()?.to_ascii_lowercase();
        let rank = characters.next()?;

        if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
            return None; 
        }

        let file_index = (file as u8 - b'a') as usize;
        let rank_index = (rank as u8 - b'1') as usize;

        Some(rank_index * 8 + file_index)
    }

    // Function to convert the index number of the game chess board vector to a chess position like a1
    fn int_to_position(index: usize) -> String {

        let file = (b'A' + (index % 8) as u8) as char;
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

        // Loops through the game chess board vector in reverse order of ranks in order to simulate looking from the white sides perspective
        // Every number in the vector is an id that corresponds to an empty tile or a chess piece, which gets printed out as the standard
        // abbreviations for each chess piece and a star for empty tiles
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
        writeln!(f, "")?;
        // ?;
        writeln!(f, "{:?}", self.get_possible_moves("e7"))
        // writeln!(f, "{:?}", Game::int_to_position(6))?;
        // writeln!(f, "{:?}", Game::position_to_int("g1"))?;
        // write!(f, "{:?}", self.board[6])
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

        let mut game = Game::new();

        println!("{:?}", game);
        // game.make_move("a2", "a4");
        // game.make_move("B7", "b5");
        // game.make_move("B5", "A4");
        // game.make_move("c7", "C5");

        // println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}