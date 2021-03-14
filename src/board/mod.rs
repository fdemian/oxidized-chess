pub mod board {

    const BOARD_LIMIT:usize = 8;

    extern crate ndarray;
    use ndarray::array;
    use ndarray::Array2;

    pub use crate::piece::piece::PieceType;
    pub use crate::player::player::Player;
    pub use crate::chesspiece::chesspiece::ChessPiece;

    pub struct Board {
     pub positions: Array2<ChessPiece>
    }

    impl Board {

        pub fn reset(&mut self) {

          let mut INITIAL_STATE:Array2<ChessPiece> = array![

              [
                ChessPiece { kind: PieceType::Rook, player: Player::Computer },
                ChessPiece { kind: PieceType::Knight, player :Player::Computer },
                ChessPiece { kind: PieceType::Bishop, player: Player::Computer },
                ChessPiece { kind: PieceType::Queen, player: Player::Computer },
                ChessPiece { kind: PieceType::King, player: Player::Computer },
                ChessPiece { kind: PieceType::Bishop, player: Player::Computer },
                ChessPiece { kind: PieceType::Knight, player: Player::Computer },
                ChessPiece { kind: PieceType::Rook, player: Player::Computer }
              ],
              [
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer }
              ],
              [ // Row 3
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None }
              ],
              [ // Row 4
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None }
              ],
              [   // Row 5
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None }
              ],
              [
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None }
              ],
              [
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User }
              ],
              [
                ChessPiece { kind: PieceType::Rook, player: Player::User },
                ChessPiece { kind: PieceType::Knight, player: Player::User },
                ChessPiece { kind: PieceType::Bishop, player: Player::User },
                ChessPiece { kind: PieceType::Queen, player: Player::User },
                ChessPiece { kind: PieceType::King, player: Player::User },
                ChessPiece { kind: PieceType::Bishop, player: Player::User },
                ChessPiece { kind: PieceType::Knight, player: Player::User },
                ChessPiece { kind: PieceType::Rook, player: Player::User }
              ]
            ];

            self.positions = INITIAL_STATE;
        }

        pub fn draw(&mut self) {

          // Iterate rows
          for r in 0..BOARD_LIMIT {
             for c in 0..BOARD_LIMIT {
                let piece = &self.positions[(r,c)];
                print!("{}", piece.get_text_repr());
             }
             println!(" ");
         }

        }

        pub fn is_valid_move(&mut self, origin:String, destination:String) -> bool {
            return true;
        }

        pub fn move_piece(&mut self, origin:String, destination:String) -> bool {
            /*if !is_valid_move(origin, destination) {
                return false;
            }*/
            return true;
        }

        pub fn check_mate_player(&self) -> Player {
            return Player::None;
        }

        pub fn computer_move(&self) {

        }

    }
}
