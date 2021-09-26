use alholmbe_chess::Game;

fn main() {
  let mut game = Game::new();
  game.print_board();
  game.make_move(String::from("a2"), String::from("a4"));
  game.print_board();
  game.make_move(String::from("a1"), String::from("a3"));
  game.print_board();
  // println!("{:?}", game.get_game_state())
  //let moves = game.get_possible_moves(String::from("d2"));
  //println!("moves = {:?}", moves)
}
