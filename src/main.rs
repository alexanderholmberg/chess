use alholmbe_chess::Game;

fn main() {
  let game = Game::new();
  // for row in game.board.iter() {
  //   println!("{:?}", row);
  // }
  // println!("{:?}", game.get_game_state())
  let moves = game.get_possible_moves(String::from("d2"));
  println!("moves = {:?}", moves)
}

//
