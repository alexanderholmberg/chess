#[cfg(test)]
mod tests {
  use super::*;
  mod init {
    use super::*;

    #[test]
    fn initializes_correctly() {
      //let game = Game::new();
      let game = Game::new_from_fen(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
      assert_eq!(game.board[0][0].unwrap(), Piece::Rook(Colour::White));
      assert_eq!(game.board[0][1].unwrap(), Piece::Knight(Colour::White));
      assert_eq!(game.board[0][2].unwrap(), Piece::Bishop(Colour::White));
      assert_eq!(game.board[0][3].unwrap(), Piece::Queen(Colour::White));
      assert_eq!(game.board[0][4].unwrap(), Piece::King(Colour::White));
      assert_eq!(game.board[0][5].unwrap(), Piece::Bishop(Colour::White));
      assert_eq!(game.board[0][6].unwrap(), Piece::Knight(Colour::White));
      assert_eq!(game.board[0][7].unwrap(), Piece::Rook(Colour::White));

      for i in 0..=7 {
        assert_eq!(game.board[1][i].unwrap(), Piece::Pawn(Colour::White));
      }

      for i in 0..=7 {
        assert_eq!(game.board[2][i].is_none(), true);
      }

      for i in 0..=7 {
        assert_eq!(game.board[6][i].unwrap(), Piece::Pawn(Colour::Black));
      }

      assert_eq!(game.board[7][0].unwrap(), Piece::Rook(Colour::Black));
      assert_eq!(game.board[7][1].unwrap(), Piece::Knight(Colour::Black));
      assert_eq!(game.board[7][2].unwrap(), Piece::Bishop(Colour::Black));
      assert_eq!(game.board[7][3].unwrap(), Piece::Queen(Colour::Black));
      assert_eq!(game.board[7][4].unwrap(), Piece::King(Colour::Black));
      assert_eq!(game.board[7][5].unwrap(), Piece::Bishop(Colour::Black));
      assert_eq!(game.board[7][6].unwrap(), Piece::Knight(Colour::Black));
      assert_eq!(game.board[7][7].unwrap(), Piece::Rook(Colour::Black));
      
    }

    #[test]
    fn random_fen_inits() {
      let game = Game::new_from_fen(String::from("4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1"));

      assert_eq!(game.board[0][0].unwrap(), Piece::Rook(Colour::White));
      assert_eq!(game.board[1][3].unwrap(), Piece::Rook(Colour::White));
      assert_eq!(game.board[6][6].unwrap(), Piece::Rook(Colour::Black));
      assert_eq!(game.board[7][7].unwrap(), Piece::Rook(Colour::Black));
      assert_eq!(game.board[7][6].is_none(), true);
      assert_eq!(game.board[0][4].unwrap(), Piece::King(Colour::White));
      assert_eq!(game.board[7][4].unwrap(), Piece::King(Colour::Black));
      assert_eq!(game.board[0][5].is_none(), true);

      let game = Game::new_from_fen(String::from("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50"));

      assert_eq!(game.board[2][0].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[4][1].unwrap(), Piece::Pawn(Colour::Black));
      assert_eq!(game.board[3][1].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[3][0].unwrap(), Piece::Pawn(Colour::Black));
      assert_eq!(game.board[5][3].unwrap(), Piece::Pawn(Colour::Black));
      assert_eq!(game.board[4][4].unwrap(), Piece::Pawn(Colour::Black));
      assert_eq!(game.board[3][5].unwrap(), Piece::Pawn(Colour::Black));
      assert_eq!(game.board[4][3].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[3][4].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[2][5].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[3][7].unwrap(), Piece::Pawn(Colour::White));
      assert_eq!(game.board[4][7].unwrap(), Piece::Pawn(Colour::Black));

      assert_eq!(game.board[2][7].unwrap(), Piece::King(Colour::White));
      assert_eq!(game.board[6][5].unwrap(), Piece::King(Colour::Black));

      assert_eq!(game.board[0][0].is_none(), true);
      assert_eq!(game.board[7][7].is_none(), true);
      assert_eq!(game.board[5][7].is_none(), true);
    }

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
      assert_eq!(Game::parse_string(String::from("a5")), Position(4, 0));
      assert_eq!(Game::parse_string(String::from("d7")), Position(6, 3));
      assert_eq!(Game::parse_string(String::from("h7")), Position(6, 7));
    }

    #[test]
    fn pawn_moves() {
      let game = Game::new();

      let mut m = game.get_all_moves(Position(1, 0), Some(Piece::Pawn(Colour::White)));
      let mut m2 = vec![Position(2, 0), Position(3, 0), Position(2, 1)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m3 = game.get_all_moves(Position(1, 6), Some(Piece::Pawn(Colour::White)));
      let mut m4 = vec![Position(2, 5), Position(2, 6), Position(2, 7), Position(3, 6)];
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
      let game = Game::new();
      let m = game.get_all_moves(Position(0, 0), Some(Piece::Rook(Colour::White)));
      let m2 = vec![];
      assert_eq!(m, m2);

      let mut m = game.get_all_moves(Position(5, 5), Some(Piece::Rook(Colour::White)));
      let mut m2 = vec![];
      for i in 6..=7 {
        m2.push(Position(i, 5))
      }
      for i in 0..=4 {
        m2.push(Position(i, 5));
      }
      for i in 6..=7 {
        m2.push(Position(5, i));
      }
      for i in 0..=4 {
        m2.push(Position(5, i));
      }
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m = game.get_all_moves(Position(7, 7), Some(Piece::Rook(Colour::Black)));
      let mut m2 = vec![];
      for i in 0..=6 {
        m2.push(Position(i, 7));
        m2.push(Position(7, i));
      }
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }

    //#[test]
    // fn filtered_rook_moves() {
    //   let game = Game::new();
    //   let all_moves = Game::get_all_moves(Position(0, 0), Some(Piece::Rook(Colour::White)));
    //   let moves = game.filter_moves(Some(Piece::Rook(Colour::White)), all_moves);

    //   assert_eq!(moves, vec![]);
    // }
  }
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Position(usize, usize);

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

    pub fn get_colour(&self) -> Colour {
      // figure out how to pattern match the piece kind
      // because this is uuuuuugly
      match self {
        Piece::Pawn(Colour::White) => Colour::White,
        Piece::Pawn(Colour::Black) => Colour::Black,
        Piece::Rook(Colour::White) => Colour::White,
        Piece::Rook(Colour::Black) => Colour::Black,
        Piece::Knight(Colour::White) => Colour::White,
        Piece::Knight(Colour::Black) => Colour::Black,
        Piece::Bishop(Colour::White) => Colour::White,
        Piece::Bishop(Colour::Black) => Colour::Black,
        Piece::Queen(Colour::White) => Colour::White,
        Piece::Queen(Colour::Black) => Colour::Black,
        Piece::King(Colour::White) => Colour::White,
        Piece::King(Colour::Black) => Colour::Black,
      }
    }
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

    pub fn new_from_fen(fen_string: String) -> Game {
        Game {
            name: String::from("yoo"),
            state: GameState::InProgress,
            board: Game::initialize_board_from_fen(fen_string),
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
      let standard_starting_board = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
      Game::initialize_board_from_fen(standard_starting_board)
    }

    fn initialize_board_from_fen(fen_string: String) -> [[Option<Piece>; 8]; 8] {
      let mut board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
      let lines: Vec<&str> = fen_string.split(' ').collect();
      let positions: Vec<&str> = lines[0].split('/').collect();
      
      let mut i = 7;
      for row in 0..=7 {
        let mut col = 0;
        for ch in positions[i].chars() {
          board[row][col] = match ch {
            'r' => Some(Piece::Rook(Colour::Black)),
            'n' => Some(Piece::Knight(Colour::Black)),
            'b' => Some(Piece::Bishop(Colour::Black)),
            'q' => Some(Piece::Queen(Colour::Black)),
            'k' => Some(Piece::King(Colour::Black)),
            'p' => Some(Piece::Pawn(Colour::Black)),
            'R' => Some(Piece::Rook(Colour::White)),
            'N' => Some(Piece::Knight(Colour::White)),
            'B' => Some(Piece::Bishop(Colour::White)),
            'Q' => Some(Piece::Queen(Colour::White)),
            'K' => Some(Piece::King(Colour::White)),
            'P' => Some(Piece::Pawn(Colour::White)),
            _ => {
              col += ch.to_digit(10).unwrap() as usize - 1;
              None
            }
          };
          col += 1;
        }
        
        if i != 0 {
          i -= 1;    
        }
      }

      board
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

    fn parse_string(position: String) -> Position {
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

        return Position(row as usize, col);
    }

    fn parse_coordinates(position: Position) -> String {
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
        // find ALL moves for a piece
        let position = Game::parse_string(_position);
        let moving_piece = self.board[position.0][position.1];
        let all_moves = Game::get_all_moves(self, position, moving_piece);

        // filter the moves here
        // let moves = Game::filter_moves(self, moving_piece, all_moves);

        let mut str_moves = vec![]; // vec![String::from("c1"), String::from("d1")];
        for mv in all_moves {
            str_moves.push(Game::parse_coordinates(mv));
        }

        Some(str_moves)
    }

    // memoize this function
    // should I filter moves here or not? probably
    fn get_all_moves(&self, position: Position, piece: Option<Piece>) -> Vec<Position> {
      let moves = match piece {
          Some(Piece::Pawn(Colour::White)) => {
              let mut moves = vec![];
              // 2 forward
              match position {
                  Position(1, col) => {
                      moves.push(Position(3, col));
                  }
                  Position(6, col) => {
                      moves.push(Position(4, col));
                  }
                  _ => {}
              }

              // 1 forward
              moves.push(Position(position.0 + 1, position.1));

              // 1 diagonal to the right
              if position.1 < 7 {
                  moves.push(Position(position.0 + 1, position.1 + 1));
              }
              
              // 1 diagonal to the left
              if position.1 > 0 {
                  moves.push(Position(position.0 + 1, position.1 - 1));
              }

              moves
          },
          Some(Piece::Rook(Colour::White)) | Some(Piece::Rook(Colour::Black)) => {
            let mut moves = vec![];
            // every tile forward
            if position.0 < 7 {
              for i in (position.0 + 1)..=7 {
                // if hit -> check color
                match self.board[i][position.1] {
                  Some(attacked_piece) => {
                    // safe to unwrap piece here because of previous match expr
                    if attacked_piece.get_colour() != piece.unwrap().get_colour() {
                      moves.push(Position(i, position.1));
                    }

                    break;
                  }
                  None => moves.push(Position(i, position.1))
                } 
              }
            }
            // every tile backward
            if position.0 > 0 {
              for i in ((0..=position.0 - 1)).rev() {
                // if hit -> check color
                match self.board[i][position.1] {
                  Some(attacked_piece) => {
                    // safe to unwrap piece here because of previous match expr
                    if attacked_piece.get_colour() != piece.unwrap().get_colour() {
                      moves.push(Position(i, position.1));
                    }

                    break;
                  }
                  None => moves.push(Position(i, position.1))
                } 
              }
            }
            // every tile to right
            if position.1 < 7 {
              for i in (position.1 + 1)..=7 {
                // if hit -> check color
                match self.board[position.0][i] {
                  Some(attacked_piece) => {
                    // safe to unwrap piece here because of previous match expr
                    if attacked_piece.get_colour() != piece.unwrap().get_colour() {
                      moves.push(Position(position.0, i));
                    }

                    break;
                  }
                  None => moves.push(Position(position.0, i))
                } 

                
              }
            }
            // every tile to left
            if position.1 > 0 {
              for i in (0..=(position.1 - 1)).rev() {
                moves.push(Position(position.0, i));

                match self.board[position.0][i] {
                  Some(attacked_piece) => {
                    // safe to unwrap piece here because of previous match expr
                    if attacked_piece.get_colour() != piece.unwrap().get_colour() {
                      moves.push(Position(position.0, i));
                    }

                    break;
                  }
                  None => moves.push(Position(position.0, i))
                } 
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
          // Some(Piece::Knight(Colour::Black)) => {},
          // Some(Piece::Bishop(Colour::Black)) => {},
          // Some(Piece::Queen(Colour::Black)) => {},
          // Some(Piece::King(Colour::Black)) => {},
          // None => {},
      };

      moves
  }
}
