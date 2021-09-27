use alholmbe_chess::Game;
use std::io;

fn main() {
  let mut game = Game::new();

  println!("Guess the number!");
  let mut whites_turn = true;

  loop {
    if whites_turn {
      println!("move for white (from, to) EXAMPLE a2a4: ");
    } else {
      println!("move for black (from, to) EXAMPLE a7a5: ");
    }

    let mut mv = String::new();

    io::stdin().read_line(&mut mv).expect("Failed to read line");

    // skip the rest of the current iteration if we get a non number
    let actual_move: String = match mv.trim().parse() {
      Ok(pos) => pos,
      Err(_) => continue,
    };

    if actual_move == "q" {
      break;
    }

    let mut from = String::from("");
    let mut to = String::from("");
    for (i, c) in actual_move.chars().enumerate() {
      if i == 0 || i == 1 {
        from.push(c);
      } else {
        to.push(c);
      }
    }

    // kolla ifall from och to ar legit moves

    println!("from: {}, to: {}", from, to);

    game.make_move(from, to);
    game.print_board();
    whites_turn = !whites_turn;
  }

  //let moves = game.get_possible_moves(String::from("d2"));
  //println!("moves = {:?}", moves)
}
