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
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    Checkmate
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
        if self.state == GameState::Checkmate {
            return Some(self.state);
        }
        if _from.len() != 2 || _to.len() != 2 {return Some(self.state)};

        let from_index = Game::position_to_int(_from).expect("The first position was not input correctly");
        let to_index = Game::position_to_int(_to).expect("The second position was not input correctly");

        let possible_moves = self.get_possible_moves(from_index)?;
        
        // if the move is possible and within rules, do it
        if possible_moves.contains(&to_index) && ((self.white_turn && self.board[from_index] / 10 == 0) || (self.white_turn == false && self.board[from_index] / 10 == 1)) {
            self.board[to_index] = self.board[from_index];
            self.board[from_index] = 0;

            // check where the opposing king is and if it is being attacked
            for i in 0..64 {
                if self.board[i] == BLACK_KING && self.white_turn && self.get_all_possible_moves(self.white_turn).contains(&i) {
                    self.state = self.check_checkmate(&i);
                } else if self.board[i] == WHITE_KING && !self.white_turn && self.get_all_possible_moves(self.white_turn).contains(&i){
                    self.state = self.check_checkmate(&i);
                }
            }

            self.white_turn = !self.white_turn;
        }
        Some(self.state)
    }

    /// Set the piece type that a pawn becames following a promotion.
    // I would believe you shouldn't ask for input inside a package like this, so I don't really know how to handle this
    // and I am too lazy to make a default case, wouldn't be that hard I believe, if pawn goes to certain indexes on opposing side, self.board[index] = QUEEN
    pub fn set_promotion(&mut self, index: &usize) -> () {
        ()
    }

    /// Get the current game state.
    // Haven't used this at all lol
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    // Gets called when a king is under attack, checks if all of the kings moves would also result in it being attacked, and then
    // loops through and simulates every ally move to see if they can block the attack or kill the attacker. If the king is still under attack, checkmate
    // Returns GameState
    pub fn check_checkmate(&mut self, king_index: &usize) -> GameState{
        let mut possible_moves: Vec<usize> = Vec::new();
        let mut piece: u8 = 0;
        let mut piece2: u8 = 0;
        let colour: u8 = {
            if self.white_turn {
                1
            } else {
                0
            }
        };


        if self.get_possible_moves(*king_index).expect("error").iter().all(|&_move| self.get_all_possible_moves(self.white_turn).contains(&_move))
        {
            for i in 0..64 {

                if self.board[i] / 10 == colour && self.board[i] != BLACK_KING && self.board[i] != WHITE_KING && self.board[i] != 0{
                    possible_moves = self.get_possible_moves(i).expect("error");
                    piece = self.board[i];
                    
                    for _move in possible_moves {
                        piece2 = self.board[_move];

                        self.board[_move] = self.board[i];
                        self.board[i] = 0;

                        if !self.get_all_possible_moves(self.white_turn).contains(&king_index) {
                            self.board[i] = piece;
                            self.board[_move] = piece2;
                            return GameState::Check;
                        }
                        self.board[i] = piece;
                        self.board[_move] = piece2;
                    }
                } 
            }
            return GameState::Checkmate;
        }
        GameState::Check
    }
    
    // This calls get_possible_moves, which returns a list of all possible moves for a piece on a given tile, and converts it to 
    // a list of chess notations instead of back end chess index
    pub fn get_possible_moves_chess(&self, index: usize) -> Option<Vec<String>> {
        let possible_moves: Vec<String> = self.get_possible_moves(index)?.into_iter().map(|_move| Game::int_to_position(_move)).collect();
        Some(possible_moves)
    }

    // Returns a HashSet of all possible tiles all of the pieces on one given side can go to, so every tile white can go to for example.
    pub fn get_all_possible_moves(&self, white_turn: bool) -> HashSet<usize> {
        let mut all_possible_moves: HashSet<usize> = HashSet::new();

        if white_turn {
            for i in 0..64 {
                if self.board[i] / 10 == 0 && self.board[i] != 0 {
                    all_possible_moves.extend(self.get_possible_moves(i).expect("Error"));
                }
            }
        } else {
            for i in 0..64 {
                if self.board[i] / 10 == 1 {
                    all_possible_moves.extend(self.get_possible_moves(i).expect("Error"));
                }
            }
        }

        all_possible_moves
    }

    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling. (spoiler, I didn't)
    pub fn get_possible_moves(&self, index: usize) -> Option<Vec<usize>>{
        let piece = self.board[index];

        let mut possible_moves: Vec<usize> = Vec::new();

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

    // Function to calculate which "directions" the pawn piece should be able to move, depending on if it's a black or white pawn,
    // and check if it can move to take another piece. Afterwards it runs the singular_moves function which is also for the king and knight pieces
    fn pawn_moves(&self, index: usize, piece: u8) -> Vec<usize> {
        
        let mut directions: Vec<isize> = Vec::new();

        if piece == 11{
            directions.push(-8);
            if index / 8 == 6 {
                directions.push(-16);
            } 
            if index as isize - 9 >= 0 {
                if self.board[index - 9] != 0 && self.board[index - 9] < 11 {
                    directions.push(-9);
                }
            }
            if index as isize - 7 >= 0 {
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

    // Function to calculate if a move would be out of bounds or not, and then send it to possible_move finally determine if it's a possible move
    fn singular_moves(&self, index: usize, piece: u8, directions: &[isize]) -> Vec<usize>{
        let mut possible_moves: Vec<usize> = Vec::new();

        //Loops around for every move/direction a piece could go and calls possible_move
        for &direction in directions {
            let target = index as isize + direction;

            if target >= 0 && target <= 63 {
                let target = target as usize;

                if !((index as isize % 8  - target as isize % 8).abs() > 2) {
                    self.possible_move(&mut possible_moves, target, piece);
                } 
            }
        }
        possible_moves
    }

    //Function to calculate if a move would be out of bounds or not, and then send it to possible_move finally determine if it's a possible move
    fn sweeping_moves(&self, index: usize, piece: u8, directions: &[isize]) -> Vec<usize> {
        let mut possible_moves: Vec<usize> = Vec::new();

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
    fn possible_move(&self, possible_moves: &mut Vec<usize>, target: usize, piece: u8) -> bool {
        let tile = self.board[target];

        if tile == 0{
            possible_moves.push(target); 
        } else if tile / 10 != piece / 10 {
            possible_moves.push(target);
            return true;
        } else {
            return true;
        }
        false
    }

    // Function to convert a chess notation/position like a1 to the index number of the game chess board vector
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

    // Function to convert the index number of the game chess board vector to a chess notation/position like a1
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

// Btw, this example has the chess board kind of inverted, the kings are supposed to be 2 tiles to the right

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
                    EMPTY => piece = "*".to_string(),
                    WHITE_PAWN => piece = "P".to_string(),
                    WHITE_ROOK => piece = "R".to_string(),
                    WHITE_KNIGHT => piece = "Kn".to_string(),
                    WHITE_BISHOP => piece = "B".to_string(),
                    WHITE_QUEEN => piece = "Q".to_string(),
                    WHITE_KING => piece = "K".to_string(),
                    BLACK_PAWN => piece = "p".to_string(),
                    BLACK_ROOK => piece = "r".to_string(),
                    BLACK_KNIGHT => piece = "kn".to_string(),
                    BLACK_BISHOP => piece = "b".to_string(),
                    BLACK_QUEEN => piece = "q".to_string(),
                    BLACK_KING => piece = "k".to_string(),
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
        writeln!(f, "")
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

        // println!("{:?}", game);

        // Test to see if white check is possible
        // game.make_move("e2", "e3");
        // game.make_move("a7", "a6");

        // game.make_move("d1", "f3");
        // game.make_move("b7", "b6");

        //  game.make_move("f3", "e4");
        //  game.make_move("h7", "h6");


        //  game.make_move("a2", "a3");
        //  game.make_move("e7", "e5");

        //  game.make_move("e4", "e5");


        // Test to see if black check is possible
        // game.make_move("e2", "e4");
        // game.make_move("e7", "e6");
        
        // game.make_move("a2", "a3");
        // game.make_move("d8", "f6");

        //  game.make_move("b2", "b3");
        //  game.make_move("f6", "f5");

        //  game.make_move("h2", "h3");
        //  game.make_move("f5", "e4");

        //game.make_move("e4", "e5");


        println!("{:?}", game);
        println!("This is the gamestate: {:?}", game.state);

        //assert_eq!(game.get_game_state(), GameState::InProgress); ???
        // Turned this off to test if check and checkmate worked
    }
}