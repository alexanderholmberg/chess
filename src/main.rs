use alholmbe_chess::Game;
use std::io;

fn main() {
  let mut game = Game::new();

  println!("Guess the number!");

  loop {
    if game.whites_turn {
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
    if !check_input(from.clone(), to.clone()) {
      println!("illegal input!");
      continue;
    }

    println!("from: {}, to: {}", from, to);

    // this needs to return an err or something if the move is not made
    game.make_move(from, to);
    game.print_board();
  }
}

fn check_input(from: String, to: String) -> bool {
  let every_tile = Game::get_all_tiles();
  let mut legit: (bool, bool) = (false, false);
  for tile in every_tile {
    if tile == from {
      legit.0 = true;
    }
    if tile == to {
      legit.1 = true;
    }
  }
  if legit == (true, true) {
    true
  } else {
    false
  }
}
