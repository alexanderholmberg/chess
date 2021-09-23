#[cfg(test)]
mod tests {
  use super::*;
  mod init {
    use super::*;
    #[test]
    fn kings_are_in_place() {
      let game = Game::new();
      assert_eq!(
        game.board[7][4].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::King(Colour::Black)
      );
      assert_eq!(
        game.board[0][4].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::King(Colour::White)
      );
    }
    #[test]
    fn queens_are_in_place() {
      let game = Game::new();
      assert_eq!(
        game.board[7][3].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::Queen(Colour::Black)
      );
      assert_eq!(
        game.board[0][3].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::Queen(Colour::White)
      );
    }

    #[test]
    fn rooks_are_in_place() {
      let game = Game::new();
      assert_eq!(
        game.board[0][0].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::Rook(Colour::White)
      );
      assert_eq!(
        game.board[7][7].unwrap_or(Piece::Pawn(Colour::Black)),
        Piece::Rook(Colour::Black)
      );
    }

    #[test]
    fn nones_in_place() {
      let game = Game::new();
      assert_eq!(game.board[3][5].is_none(), true);
      assert_eq!(game.board[4][2].is_none(), true);
    }

    fn gets_game_state() {
      let mut game = Game::new();
      assert_eq!(*game.get_game_state(), GameState::InProgress);
      game.state = GameState::GameOver;
      assert_eq!(*game.get_game_state(), GameState::GameOver);
    }
  }

  mod movement {
    use super::*;
    #[test]
    fn parsing_positions() {
      //let game = Game::new();
      assert_eq!(Game::parse_string(String::from("a5")), (4, 0));
      assert_eq!(Game::parse_string(String::from("d7")), (6, 3));
      assert_eq!(Game::parse_string(String::from("h7")), (6, 7));
    }

    #[test]
    fn pawn_moves() {
      // let game = Game::new();

      let mut m = Game::get_available_moves((1, 0), Some(Piece::Pawn(Colour::White)));
      let mut m2 = vec![(2, 0), (3, 0), (2, 1)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m3 = Game::get_available_moves((1, 6), Some(Piece::Pawn(Colour::White)));
      let mut m4 = vec![(2, 5), (2, 6), (2, 7), (3, 6)];
      m3.sort();
      m4.sort();
      assert_eq!(m3, m4);

      // let moves = match game.get_possible_moves(String::from("b1")) {
      //     Some(v) => v,
      //     _ => panic!("eerrrrrrr!"),
      // };
      // assert_eq!(moves, vec![String::from("c1"), String::from("d1")]);
    }

    #[test]
    fn rook_moves() {
      let mut m = Game::get_available_moves((0, 0), Some(Piece::Rook(Colour::White)));
      let mut m2 = vec![];
      for i in 1..=7 {
        m2.push((i, 0));
        m2.push((0, i));
      }
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m = Game::get_available_moves((5, 5), Some(Piece::Rook(Colour::White)));
      let mut m2 = vec![];
      for i in 6..=7 {
        m2.push((i, 5))
      }
      for i in 0..=4 {
        m2.push((i, 5));
      }
      for i in 6..=7 {
        m2.push((5, i));
      }
      for i in 0..=4 {
        m2.push((5, i));
      }
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colour {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    King(Colour),
    Rook(Colour),
    Bishop(Colour),
    Queen(Colour),
    Knight(Colour),
    Pawn(Colour),
}

impl Piece {
    // fn take_turn(&self, /*...*/) -> /*...*/ {
    //     match self {
    //         Piece::King(_colour) => /*...*/,
    //         // ...
    //     }
    // }

    // ...
}

#[derive(Debug)]
pub struct Game {
    pub state: GameState,
    pub name: String,
    pub board: [[Option<Piece>; 8]; 8],
}

impl Game {
    // creates a new game
    pub fn new() -> Game {
        Game {
            name: String::from("yoo"),
            state: GameState::InProgress,
            board: Game::initialize_board(),
        }
    }

    fn get_starting_piece_at_position(row: usize, col: usize) -> Piece {
        match (row, col) {
            (1, _) => Piece::Pawn(Colour::White),
            (0, 0) => Piece::Rook(Colour::White),
            (0, 1) => Piece::Knight(Colour::White),
            (0, 2) => Piece::Bishop(Colour::White),
            (0, 3) => Piece::Queen(Colour::White),
            (0, 4) => Piece::King(Colour::White),
            (0, 5) => Piece::Bishop(Colour::White),
            (0, 6) => Piece::Knight(Colour::White),
            (0, 7) => Piece::Rook(Colour::White),
            (6, _) => Piece::Pawn(Colour::Black),
            (7, 0) => Piece::Rook(Colour::Black),
            (7, 1) => Piece::Knight(Colour::Black),
            (7, 2) => Piece::Bishop(Colour::Black),
            (7, 3) => Piece::Queen(Colour::Black),
            (7, 4) => Piece::King(Colour::Black),
            (7, 5) => Piece::Bishop(Colour::Black),
            (7, 6) => Piece::Knight(Colour::Black),
            (7, 7) => Piece::Rook(Colour::Black),
            _ => Piece::Pawn(Colour::White), // handle error here
        }
    }

    fn initialize_board() -> [[Option<Piece>; 8]; 8] {
        let mut b: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
        for row in 0..=1 {
            for col in 0..=7 {
                b[row][col] = Some(Game::get_starting_piece_at_position(row, col));
            }
        }
        for row in 6..=7 {
            for col in 0..=7 {
                b[row][col] = Some(Game::get_starting_piece_at_position(row, col));
            }
        }

        b
    }

    // if illegal -> return Err
    // if legal and InProgress is true -> return the current state of the game
    // example position = "a5"
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        // parse position to a row and col
        // check if it's a legal move
        // make the move by updating the board
        // return the current state

        //let moves = Game::get_possible_moves(self, _from);
        let from = Game::parse_string(_from);

        let to = Game::parse_string(_to);

        //let possible_moves = get_possible_moves(self, from);
        Some(GameState::InProgress)
    }

    fn parse_string(position: String) -> (usize, usize) {
        // turn string into two chars
        // match the first one with a num
        // parse the other one - 1 to a int
        let mut chars = vec![];
        for ch in position.chars() {
            chars.push(ch);
        }

        let col = match chars[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => 8, // handle error
        };
        let row = chars[1].to_digit(10).unwrap() - 1; // handle error

        return (row as usize, col);
    }

    fn parse_coordinates(position: (usize, usize)) -> String {
        let col = match position.1 {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => "x", // handle error
        };

        let row = (position.0 + 1).to_string();
        let fin = String::from(col) + &row;
        fin
    }

    // // promote a peasant
    // pub fn set_promotion(&mut self, _piece: String) -> () {

    // }
    // // get current game state
    // the api return GameState, not a reference
    pub fn get_game_state(&self) -> &GameState {
        &self.state
    }

    

    // // given position, returns all possible moves for the piece
    pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>> {
        // hitta alla moves
        // filtrera ifall nagot ar i vagen
        let position = Game::parse_string(_position);
        let piece = self.board[position.0][position.1];
        println!("{:?}", piece);

        let moves = Game::get_available_moves(position, piece);

        // filter the moves here

        let mut str_moves = vec![]; // vec![String::from("c1"), String::from("d1")];

        for mv in moves {
            str_moves.push(Game::parse_coordinates(mv));
        }

        Some(str_moves)
        //Some(vec![String::from("yo")])
    }

    fn get_available_moves(position: (usize, usize), piece: Option<Piece>) -> Vec<(usize, usize)> {
        let moves = match piece {
            Some(Piece::Pawn(Colour::White)) => {
                let mut moves = vec![];
                // 2 forward
                match position {
                    (1, col) => {
                        moves.push((3, col));
                    }
                    (6, col) => {
                        moves.push((4, col));
                    }
                    _ => {}
                }

                // 1 forward
                moves.push((position.0 + 1, position.1));

                // 1 diagonal to the right
                if position.1 < 7 {
                    moves.push((position.0 + 1, position.1 + 1));
                }
                
                // 1 diagonal to the left
                if position.1 > 0 {
                    moves.push((position.0 + 1, position.1 - 1));
                }

                moves
            },
            Some(Piece::Rook(Colour::White)) => {
              let mut moves = vec![];
              // every tile forward
              if position.0 < 7 {
                for i in (position.0 + 1)..=7 {
                  moves.push((i, position.1)); 
                }
              }
              // every tile backward
              if position.0 > 0 {
                for i in ((0..=position.0 - 1)).rev() {
                  moves.push((i, position.1));
                }
              }
              // every tile to right
              if position.1 < 7 {
                for i in (position.1 + 1)..=7 {
                  moves.push((position.0, i));
                }
              }
              // every tile to left
              if position.1 > 0 {
                for i in (0..=(position.1 - 1)).rev() {
                  moves.push((position.0, i));
                }
              }
              moves
            },
            _ => vec![]
            // Some(Piece::Knight(Colour::White)) => {},
            // Some(Piece::Bishop(Colour::White)) => {},
            // Some(Piece::Queen(Colour::White)) => {},
            // Some(Piece::King(Colour::White)) => {},
            // Some(Piece::Pawn(Colour::Black)) => {},
            // Some(Piece::Rook(Colour::Black)) => {},
            // Some(Piece::Knight(Colour::Black)) => {},
            // Some(Piece::Bishop(Colour::Black)) => {},
            // Some(Piece::Queen(Colour::Black)) => {},
            // Some(Piece::King(Colour::Black)) => {},
            // None => {},
        };

        moves
    }
}
