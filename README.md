# DD1337 Week 3-4

## Chess Project

Following public functions are found in the public struct `Game`:

| **Function**                                                                   | **Description**                                                                                                                                                          |
| ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `pub fn new() -> Game`                                                         | Initialises a new board with pieces.                                                                                                                                     |
| `pub fn new_from_fen(fen_string: String) -> Game`                              | Initialises a new board with pieces from the provided FEN-string.                                                                                                        |
| `pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState>` | If the current game state is `InProgress` and the move is legal, it moves a piece and returns the resulting state of the game. If the move is illegal, None is returned. |
| `pub fn set_promotion(&mut self, _piece: String) -> ()`                        | Set the piece type that a peasant becames following a promotion.                                                                                                         |
| `pub fn get_game_state(&self) -> &GameState`                                   | Gets the current game state.                                                                                                                                             |
| `pub fn get_possible_moves(&self, _position: String) -> Optional<Vec<String>>` | If a piece is standing on the given tile, it returns all possible new positions for that piece. If there is no piece at the given tile, it returns None.                 |
| `pub fn play()`                                                                | Starts a game of chess in the terminal.                                                                                                                                  |

Positions are given as strings with the format `"<file><rank>"`. For example, `"a4"` or `"d6"`

For the terminal chess, to move a piece you type in the previous position followed by the new position. For example, if you want to go from a2 to a4, the input should be `a2a4`.

The program also exports an enumerable `GameState` with the values:

- `InProgress`,
- `Check`,
- `GameOver`,
- _(optional)_ `Checkmate` and
- _(optional)_ `DeadPosition`.
