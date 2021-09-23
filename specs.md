# mandatory

- whose turn it is
- how pieces can move
- check
- promotion

Positions are given as strings with the format "<file><rank>"

following funcs should be public:

```rs
pub struct Game {
  // creates a new game
  pub fn new() -> Game

  // if illegal -> return Err
  // if legal and InProgress is true -> return the current state of the game
  pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState>

  // promote a peasant
  pub fn set_promotion(&mut self, _piece: String) -> ()

  // get current game state
  pub fn get_game_state(&self) -> GameState

  // given position, returns all possible moves for the piece
  pub fn get_possible_moves(&self, _position: String) -> Optional<Vec<String>>


}
```

Program should also export this enum:

```rs
pub enum GameState {
  InProgress bool,
  Check bool,
  GameOver bool,
}
```

tankar:

- läsa in FEN strings för att smälla upp ett bräde
  - hade hjälp enormt vid testande

få input -> parsea till row,col -> generera alla lagliga moves
-> kolla om move är lagligt -> move
ex:
"a2", "a4" -> Pawn till (0, 3) -> [(0, 2), (0, 3)] -> är lagligt
-> förflytta

todo:

- load board from FEN string
