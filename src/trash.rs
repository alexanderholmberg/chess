// fn filter_moves(&self, moving_piece: Option<Piece>, all_moves: Vec<Position>) -> Vec<Position> {
//   // for every move, check if spot is taken
//   // if taken and same colour -> not possible move
//   // if taken and diff colour -> possible move

//   // match for every piece and filter moves accordingly

//   match moving_piece {
//     Some(Piece::Rook(_)) => {
//       // check front
//       // check back
//       // check right
//       // check left
//     }
//     None => panic!("trying to move a None piece"),
//   }

//   let mut moves = vec![];
//   for mv in &all_moves {
//     let piece = self.board[mv.0][mv.1];
//     match piece {
//       None => {
//         // can always move to a None tile
//         moves.push(mv)
//       }
//       Some(attacked_piece) => match moving_piece {
//         Some(attacking_piece) => {
//           if attacked_piece.get_colour() != attacking_piece.get_colour() {
//             moves.push(mv);
//           }
//         }
//         None => panic!("trying to move a None piece"),
//       },
//     }
//   }

//   all_moves
// }
