#[cfg(test)]
mod tests {
  mod init {
    use crate::Colour;
    use crate::Game;
    use crate::GameState;
    use crate::Piece;

    #[test]
    fn initializes_correctly() {
      //let game = Game::new();
      let game = Game::new_from_fen(String::from(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
      ));
      assert_eq!(game.turn, Colour::White);
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

      let game = Game::new_from_fen(String::from("4k2r/6r1/8/8/8/8/3R4/R3K3 b Qk - 0 1"));
      assert_eq!(game.turn, Colour::Black);
    }

    #[test]
    fn random_fen_inits() {
      let game = Game::new_from_fen(String::from("4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1"));
      assert_eq!(game.turn, Colour::White);

      assert_eq!(game.board[0][0].unwrap(), Piece::Rook(Colour::White));
      assert_eq!(game.board[1][3].unwrap(), Piece::Rook(Colour::White));
      assert_eq!(game.board[6][6].unwrap(), Piece::Rook(Colour::Black));
      assert_eq!(game.board[7][7].unwrap(), Piece::Rook(Colour::Black));
      assert_eq!(game.board[7][6].is_none(), true);
      assert_eq!(game.board[0][4].unwrap(), Piece::King(Colour::White));
      assert_eq!(game.board[7][4].unwrap(), Piece::King(Colour::Black));
      assert_eq!(game.board[0][5].is_none(), true);

      let game = Game::new_from_fen(String::from(
        "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50",
      ));
      assert_eq!(game.turn, Colour::Black);

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
    use crate::Colour;
    use crate::Game;
    use crate::Piece;
    use crate::Position;
    #[test]
    fn parsing_positions() {
      //let game = Game::new();
      assert_eq!(Game::parse_string(&String::from("a5")), Position(4, 0));
      assert_eq!(Game::parse_string(&String::from("d7")), Position(6, 3));
      assert_eq!(Game::parse_string(&String::from("h7")), Position(6, 7));
    }

    #[test]
    fn random_moves() {}

    #[test]
    fn getting_possible_moves() {
      let game = Game::new();
      assert_eq!(game.get_possible_moves(String::from("a2")).is_some(), true);
      let mut m = game.get_possible_moves(String::from("a2")).unwrap();
      let mut m2 = vec![String::from("a3"), String::from("a4")];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
      assert_eq!(game.get_possible_moves(String::from("c3")).is_none(), true);
      assert_eq!(game.get_possible_moves(String::from("c6")).is_none(), true);
      assert_eq!(game.get_possible_moves(String::from("d5")).is_none(), true);
      assert_eq!(game.get_possible_moves(String::from("e4")).is_none(), true);
    }
    #[test]
    fn move_twice_in_row() {
      let mut game = Game::new();
      // cant move white twice in a row
      assert_eq!(
        game
          .make_move(String::from("a2"), String::from("a4"))
          .is_some(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("b2"), String::from("b4"))
          .is_some(),
        false
      );

      assert_eq!(
        game
          .make_move(String::from("a7"), String::from("a5"))
          .is_some(),
        true
      );
      assert_eq!(
        game
          .make_move(String::from("f7"), String::from("f5"))
          .is_some(),
        false
      );
    }

    #[test]
    fn moving_none() {
      let mut game = Game::new();
      // cant move white twice in a row
      assert_eq!(
        game
          .make_move(String::from("a3"), String::from("a4"))
          .is_some(),
        false
      );
      assert_eq!(
        game
          .make_move(String::from("b4"), String::from("b5"))
          .is_some(),
        false
      );
      assert_eq!(
        game
          .make_move(String::from("b6"), String::from("b7"))
          .is_some(),
        false
      );
    }

    #[test]
    fn validate_input() {
      assert_eq!(
        Game::check_input(String::from("daggbfdlgh"), String::from("bfdnlgbvfdhb")),
        false
      );
      assert_eq!(
        Game::check_input(String::from("f2"), String::from("f9")),
        false
      );
      assert_eq!(
        Game::check_input(String::from("f0"), String::from("e0")),
        false
      );
      assert_eq!(
        Game::check_input(String::from("a2"), String::from("a4")),
        true
      );
      assert_eq!(
        Game::check_input(String::from("b7"), String::from("b5")),
        true
      );
      assert_eq!(
        Game::check_input(String::from("g2"), String::from("g4")),
        true
      );
    }

    #[test]
    fn pawn_moves() {
      let game = Game::new();

      let mut m = game.get_all_moves(Position(1, 0), Piece::Pawn(Colour::White));
      let mut m2 = vec![Position(2, 0), Position(3, 0)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m = game.get_all_moves(Position(1, 6), Piece::Pawn(Colour::White));
      let mut m2 = vec![Position(2, 6), Position(3, 6)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game2 = Game::new_from_fen(String::from(
        "8/5k2/4p3/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50",
      ));
      let mut m = game2.get_all_moves(Position(4, 3), Piece::Pawn(Colour::White));
      let mut m2 = vec![Position(5, 3), Position(5, 4)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
      let mut m = game2.get_all_moves(Position(3, 0), Piece::Pawn(Colour::Black));
      let mut m2 = vec![];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }

    #[test]
    fn rook_moves() {
      let game = Game::new();
      let m = game.get_all_moves(Position(0, 0), Piece::Rook(Colour::White));
      let m2 = vec![];
      assert_eq!(m, m2);

      let m = game.get_all_moves(Position(7, 7), Piece::Rook(Colour::Black));
      let m2 = vec![];
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "8/3q1k2/3r4/1p1Pp2p/pP2Pp1P/P4P2/7K/8 b - - 99 50",
      ));
      let mut m = game.get_all_moves(Position(5, 3), Piece::Rook(Colour::Black));
      let mut m2 = vec![
        Position(4, 3),
        Position(5, 4),
        Position(5, 5),
        Position(5, 6),
        Position(5, 7),
        Position(5, 2),
        Position(5, 1),
        Position(5, 0),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      // let mut m = game.get_possible_moves(String::from("d6")).unwrap();
      // let mut m2 = vec![
      //   String::from("a6"),
      //   String::from("b6"),
      //   String::from("c6"),
      //   String::from("e6"),
      //   String::from("f6"),
      //   String::from("g6"),
      //   String::from("h6"),
      //   String::from("d5"),
      // ];
      // m.sort();
      // m2.sort();
      // assert_eq!(m, m2);
    }

    #[test]
    fn knight_moves() {
      let game = Game::new();
      let mut m = game.get_all_moves(Position(3, 3), Piece::Knight(Colour::White));
      let mut m2 = vec![
        Position(5, 2),
        Position(5, 4),
        Position(4, 1),
        Position(2, 1),
        Position(2, 5),
        Position(4, 5),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
      let mut m = game.get_all_moves(Position(7, 6), Piece::Knight(Colour::Black));
      let mut m2 = vec![Position(5, 7), Position(5, 5)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/1n1B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(4, 3), Piece::Knight(Colour::White));
      let mut m2 = vec![
        Position(5, 1),
        Position(3, 1),
        Position(6, 2),
        Position(6, 4),
        Position(5, 5),
        Position(3, 5),
        Position(2, 4),
        Position(2, 2),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m = game.get_all_moves(Position(5, 1), Piece::Knight(Colour::Black));
      let mut m2 = vec![Position(3, 0), Position(3, 2), Position(4, 3)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/3B4/1p1NP2P/6P1/3P1Q2/PnP1K3/q5b1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(1, 1), Piece::Knight(Colour::Black));
      let mut m2 = vec![
        Position(3, 0),
        Position(3, 2),
        Position(2, 3),
        Position(0, 3),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }

    #[test]
    fn bishop_moves() {
      let game = Game::new();
      let mut m = game.get_all_moves(Position(0, 2), Piece::Bishop(Colour::White));
      let mut m2 = vec![];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(5, 3), Piece::Bishop(Colour::White));
      let mut m2 = vec![
        Position(4, 2),
        Position(3, 1),
        Position(2, 0),
        Position(6, 2),
        Position(7, 1),
        Position(6, 4),
        Position(7, 5),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m = game.get_all_moves(Position(0, 6), Piece::Bishop(Colour::Black));
      let mut m2 = vec![
        Position(1, 5),
        Position(2, 4),
        Position(3, 3),
        Position(4, 2),
        Position(5, 1),
        Position(1, 7),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3PbQ2/P1P1K3/q7 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(2, 4), Piece::Bishop(Colour::Black));
      let mut m2 = vec![
        Position(1, 5),
        Position(0, 6),
        Position(3, 3),
        Position(4, 2),
        Position(5, 1),
        Position(3, 5),
        Position(4, 6),
        Position(5, 7),
        Position(1, 3),
        Position(0, 2),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/b5q1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(0, 0), Piece::Bishop(Colour::Black));
      let mut m2 = [
        Position(1, 1),
        Position(2, 2),
        Position(3, 3),
        Position(4, 4),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }
    #[test]
    fn queen_moves() {
      let game = Game::new();
      let mut m = game.get_all_moves(Position(0, 3), Piece::Queen(Colour::White));
      let mut m2 = vec![];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(2, 5), Piece::Queen(Colour::White));
      let mut m2 = vec![
        Position(2, 4),
        Position(3, 4),
        Position(3, 5),
        Position(4, 5),
        Position(5, 5),
        Position(6, 5),
        Position(2, 6),
        Position(2, 7),
        Position(1, 6),
        Position(0, 7),
        Position(1, 5),
        Position(0, 5),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let mut m = game.get_all_moves(Position(0, 0), Piece::Queen(Colour::Black));
      let mut m2 = vec![
        Position(1, 0),
        Position(1, 1),
        Position(2, 2),
        Position(3, 3),
        Position(4, 4),
        Position(0, 1),
        Position(0, 2),
        Position(0, 3),
        Position(0, 4),
        Position(0, 5),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }
    #[test]
    fn king_moves() {
      let game = Game::new();
      let mut m = game.get_all_moves(Position(0, 4), Piece::King(Colour::White));
      let mut m2 = vec![];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      let mut m = game.get_all_moves(Position(1, 4), Piece::King(Colour::White));
      let mut m2 = vec![
        Position(1, 3),
        Position(2, 4),
        Position(1, 5),
        Position(0, 5),
        Position(0, 4),
        Position(0, 3),
      ];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
      let mut m = game.get_all_moves(Position(7, 4), Piece::King(Colour::Black));
      let mut m2 = vec![Position(7, 3), Position(7, 5), Position(6, 4)];
      m.sort();
      m2.sort();
      assert_eq!(m, m2);
    }

    #[test]
    fn getting_king() {
      let game = Game::new();
      assert_eq!(game.get_king(String::from("white")), Position(0, 4));
      assert_eq!(game.get_king(String::from("black")), Position(7, 4));
      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      assert_eq!(game.get_king(String::from("white")), Position(1, 4));
      assert_eq!(game.get_king(String::from("black")), Position(7, 4));
      let game = Game::new_from_fen(String::from(
        "8/3q1k2/3r4/1p1Pp2p/pP2Pp1P/P4P2/7K/8 b - - 99 50",
      ));
      assert_eq!(game.get_king(String::from("white")), Position(1, 7));
      assert_eq!(game.get_king(String::from("black")), Position(6, 5));
    }
  }

  mod special_rules {
    use crate::Castling;
    use crate::Colour;
    use crate::Game;
    use crate::GameState;
    use crate::Piece;
    use crate::Position;
    #[test]
    fn castling() {
      let game = Game::new();
      let castling = Castling {
        white_queen: true,
        white_king: true,
        black_queen: true,
        black_king: true,
      };
      assert_eq!(game.castling, castling);

      let game = Game::new_from_fen(String::from(
        "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1",
      ));
      let castling = Castling {
        white_queen: false,
        white_king: false,
        black_queen: false,
        black_king: false,
      };
      assert_eq!(game.castling, castling);
      let game = Game::new_from_fen(String::from("4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1"));
      let castling = Castling {
        white_queen: true,
        white_king: false,
        black_queen: false,
        black_king: true,
      };
      assert_eq!(game.castling, castling);
    }

    #[test]
    fn check() {
      let game = Game::new();
      assert_eq!(game.state, GameState::InProgress);
      //game.check_for_check(attacking_piece: Piece, origin: Position, target: Position)

      let mut game = Game::new_from_fen(String::from(
        "rnbqkbnr/ppp2ppp/8/3pp3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 3",
      ));
      game.make_move(String::from("f1"), String::from("b5"));
      assert_eq!(game.state, GameState::Check);
      let mut game = Game::new_from_fen(String::from(
        "rnbq1bnr/ppp2ppp/3k4/1B1P4/5p2/8/PPPPQ1PP/RNB1K1NR w KQ - 2 6",
      ));
      game.make_move(String::from("e2"), String::from("e6"));
      assert_eq!(game.state, GameState::Check);
      game.make_move(String::from("d6"), String::from("c5"));
      assert_eq!(game.state, GameState::InProgress);
    }
  }
}
