# todo

- check
- promotion

- castling
- en passant
- checkmate
- stalemate

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

<FEN> ::= <Piece Placement>
' ' <Side to move>
' ' <Castling ability>
' ' <En passant target square>
' ' <Halfmove clock>
' ' <Fullmove counter>

<Piece Placement> ::= <rank8>'/'<rank7>'/'<rank6>'/'<rank5>'/'<rank4>'/'<rank3>'/'<rank2>'/'<rank1>
<ranki> ::= [<digit17>]<piece> {[<digit17>]<piece>} [<digit17>] | '8'
<piece> ::= <white Piece> | <black Piece>
<digit17> ::= '1' | '2' | '3' | '4' | '5' | '6' | '7'
<white Piece> ::= 'P' | 'N' | 'B' | 'R' | 'Q' | 'K'
<black Piece> ::= 'p' | 'n' | 'b' | 'r' | 'q' | 'k'

<Side to move> ::= {'w' | 'b'}

<Castling ability> ::= '-' | ['K'] ['Q'] ['k'] ['q'] (1..4)

<En passant target square> ::= '-' | <epsquare>
<epsquare> ::= <fileLetter> <eprank>
<fileLetter> ::= 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h'
<eprank> ::= '3' | '6'

<Halfmove Clock> ::= <digit> {<digit>}
<digit> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'

<Fullmove counter> ::= <digit19> {<digit>}
<digit19> ::= '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
<digit> ::= '0' | <digit19>

rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
