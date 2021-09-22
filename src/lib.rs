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
    }

    mod movement {
        use super::*;
        #[test]
        fn gets_game_state() {
            let mut game = Game::new();
            assert_eq!(*game.get_game_state(), GameState::InProgress);
            game.state = GameState::GameOver;
            assert_eq!(*game.get_game_state(), GameState::GameOver);
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
    // pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {

    // }
    // // promote a peasant
    // pub fn set_promotion(&mut self, _piece: String) -> () {

    // }
    // // get current game state
    pub fn get_game_state(&self) -> &GameState {
        &self.state
    }
    // // given position, returns all possible moves for the piece
    // pub fn get_possible_moves(&self, _position: String) -> Optional<Vec<String>> {

    // }
}
