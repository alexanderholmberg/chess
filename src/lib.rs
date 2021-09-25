mod tests;

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
    let standard_starting_board =
      String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
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
    let moving_piece = moving_piece.expect("can't move a None piece");
    let all_moves = Game::get_all_moves(self, position, moving_piece);

    let mut str_moves = vec![]; // vec![String::from("c1"), String::from("d1")];
    for mv in all_moves {
      str_moves.push(Game::parse_coordinates(mv));
    }

    Some(str_moves)
  }

  // memoize this function
  // should I filter moves here or not? probably
  fn get_all_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    let moves = match piece {
          Piece::Pawn(_) => self.get_pawn_moves(position, piece),
          Piece::Rook(_) => self.get_rook_moves(position, piece),
          Piece::Knight(_) => self.get_knight_moves(position, piece),
          Piece::Bishop(Colour::White) => self.get_bishop_moves(position, piece),
          _ => vec![]
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

  fn get_pawn_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    let mut moves = vec![];

    match piece.get_colour() {
      Colour::White => {
        // 2 forward
        if position.0 == 1 {
          if self.board[3][position.1].is_none() && self.board[2][position.1].is_none() {
            moves.push(Position(3, position.1));
          }
        }

        // 1 forward
        if position.0 < 7 {
          if self.board[position.0 + 1][position.1].is_none() {
            moves.push(Position(position.0 + 1, position.1));
          }
        }

        // 1 diagonal to the right
        if position.1 < 7 {
          match self.board[position.0 + 1][position.1 + 1] {
            Some(attacked_piece) => {
              if attacked_piece.get_colour() == Colour::Black {
                moves.push(Position(position.0 + 1, position.1 + 1));
              }
            }
            None => {}
          }
        }

        // 1 diagonal to the left
        if position.1 > 0 {
          match self.board[position.0 + 1][position.1 - 1] {
            Some(attacked_piece) => {
              if attacked_piece.get_colour() == Colour::Black {
                moves.push(Position(position.0 + 1, position.1 - 1));
              }
            }
            None => {}
          }
        }
      }
      Colour::Black => {
        // 2fw
        if position.0 == 6 {
          if self.board[4][position.1].is_none() && self.board[5][position.1].is_none() {
            moves.push(Position(4, position.1));
          }
        }

        // 1 forward
        if position.0 > 0 {
          if self.board[position.0 - 1][position.1].is_none() {
            moves.push(Position(position.0 - 1, position.1));
          }
        }

        // 1 diagonal to the right
        if position.1 < 7 {
          match self.board[position.0 - 1][position.1 + 1] {
            Some(attacked_piece) => {
              if attacked_piece.get_colour() == Colour::White {
                moves.push(Position(position.0 - 1, position.1 + 1));
              }
            }
            None => {}
          }
        }

        // 1 diagonal to the left
        if position.1 > 0 {
          match self.board[position.0 - 1][position.1 - 1] {
            Some(attacked_piece) => {
              if attacked_piece.get_colour() == Colour::White {
                moves.push(Position(position.0 - 1, position.1 - 1));
              }
            }
            None => {}
          }
        }
      }
    }

    moves
  }

  fn get_rook_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    let mut moves = vec![];
    // every tile forward (whites perspective)
    if position.0 < 7 {
      for i in (position.0 + 1)..=7 {
        match self.board[i][position.1] {
          Some(attacked_piece) => {
            if attacked_piece.get_colour() != piece.get_colour() {
              moves.push(Position(i, position.1));
            }
            break;
          }
          None => moves.push(Position(i, position.1)),
        }
      }
    }
    // every tile backward
    if position.0 > 0 {
      for i in (0..=position.0 - 1).rev() {
        // if hit -> check color
        match self.board[i][position.1] {
          Some(attacked_piece) => {
            if attacked_piece.get_colour() != piece.get_colour() {
              moves.push(Position(i, position.1));
            }

            break;
          }
          None => moves.push(Position(i, position.1)),
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
            if attacked_piece.get_colour() != piece.get_colour() {
              moves.push(Position(position.0, i));
            }

            break;
          }
          None => moves.push(Position(position.0, i)),
        }
      }
    }
    // every tile to left
    if position.1 > 0 {
      for i in (0..=(position.1 - 1)).rev() {
        //moves.push(Position(position.0, i));

        match self.board[position.0][i] {
          Some(attacked_piece) => {
            // safe to unwrap piece here because of previous match expr
            if attacked_piece.get_colour() != piece.get_colour() {
              moves.push(Position(position.0, i));
            }

            break;
          }
          None => moves.push(Position(position.0, i)),
        }
      }
    }
    moves
  }

  fn get_knight_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    let mut moves = vec![];
    // maximalt 8 moves
    // check all 8
    // for every check, check bounds, and attacked piece

    // top left fw move
    if position.1 > 0 && position.0 < 6 {
      if Game::is_legal(self.board[position.0 + 2][position.1 - 1], piece) {
        moves.push(Position(position.0 + 2, position.1 - 1));
      }
    }

    // top right fw move
    if position.1 < 7 && position.0 < 6 {
      if Game::is_legal(self.board[position.0 + 2][position.1 + 1], piece) {
        moves.push(Position(position.0 + 2, position.1 + 1));
      }
    }

    // left side top move
    if position.1 > 1 && position.0 < 7 {
      if Game::is_legal(self.board[position.0 + 1][position.1 - 2], piece) {
        moves.push(Position(position.0 + 1, position.1 - 2));
      }
    }

    // left side down move
    if position.1 > 1 && position.0 > 0 {
      if Game::is_legal(self.board[position.0 - 1][position.1 - 2], piece) {
        moves.push(Position(position.0 - 1, position.1 - 2));
      }
    }

    // down left move
    if position.1 > 0 && position.0 > 1 {
      if Game::is_legal(self.board[position.0 - 2][position.1 - 1], piece) {
        moves.push(Position(position.0 - 2, position.1 - 1));
      }
    }

    // down right move
    if position.1 < 7 && position.0 > 1 {
      if Game::is_legal(self.board[position.0 - 2][position.1 + 1], piece) {
        moves.push(Position(position.0 - 2, position.1 + 1));
      }
    }

    // right down move
    if position.1 < 6 && position.0 > 0 {
      if Game::is_legal(self.board[position.0 - 1][position.1 + 2], piece) {
        moves.push(Position(position.0 - 1, position.1 + 2));
      }
    }

    // right up move
    if position.1 < 6 && position.0 < 7 {
      if Game::is_legal(self.board[position.0 + 1][position.1 + 2], piece) {
        moves.push(Position(position.0 + 1, position.1 + 2));
      }
    }

    moves
  }

  fn get_bishop_moves(&self, position: Position, piece: Piece) -> Vec<Position> {
    let mut moves = vec![];
    moves
  }

  // checks if a position is occupied
  fn is_legal(tile: Option<Piece>, piece: Piece) -> bool {
    match tile {
      Some(attacked_piece) => {
        if attacked_piece.get_colour() != piece.get_colour() {
          true
        } else {
          false
        }
      }
      None => true,
    }
  }
}
