## Chess library and command line chess in Rust

## Chess library

Following public functions are found in the public struct `Game`:

| **Function**                                                                   | **Description**                                                                                                                                                                      |
| ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `pub fn new() -> Game`                                                         | Initialises a new board with pieces.                                                                                                                                                 |
| `pub fn new_from_fen(fen_string: String) -> Game`                              | Initialises a new board with pieces from the provided FEN-string.                                                                                                                    |
| `pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState>` | If the current game state is `InProgress` and the move is legal, it moves a piece and returns the resulting state of the game. If the move is illegal, None is returned.             |
| `pub fn set_promotion(&mut self, position: String, new_piece: char) -> ()`     | Set the piece type that a pawn becomes following a promotion. If you for example want to promote the pawn at a8 to a queen, call the function with the string "a8" and the char "q". |
| `pub fn get_game_state(&self) -> &GameState`                                   | Gets the current game state.                                                                                                                                                         |
| `pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>>`   | If a piece is standing on the given position, it returns all possible moves for that piece. If there is no piece at the given tile, it returns None.                                 |
| `pub fn play()`                                                                | Starts a game of chess in the terminal.                                                                                                                                              |

The program also exports an enumerable `GameState` with the values:

- `InProgress`,
- `Check`,
- `Checkmate`
- `Stalemate`
- `GameOver`

Positions are given as strings with the format `"<file><rank>"`. For example, `"a4"` or `"d6"`

# Command Line Chess

For the command linie chess, to move a piece you type in the previous position followed by the new position. For example, if you want to go from a2 to a4, the input should be `a2a4`. To start the terminal chess, just type `cargo run`.
What it looks like in a light-themed(cursed I know) terminal:

![Chess UI](/assets/chessUIWhiteTerminal.png)

# More

The tests are divided into three modules, `init`, `movement` and `special_rules`. Type `cargo test <module>` to test a specific module, or just type `cargo test` to test them all at once.

OBS! There are some inefficient searches made in this program

# TODO

Castling and en passant are not implemented yet
