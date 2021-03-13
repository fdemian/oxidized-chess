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
                ChessPiece { kind: PieceType::Rook, player: Player::Black },
                ChessPiece { kind: PieceType::Knight, player :Player::Black },
                ChessPiece { kind: PieceType::Bishop, player: Player::Black },
                ChessPiece { kind: PieceType::Queen, player: Player::Black },
                ChessPiece { kind: PieceType::King, player: Player::Black },
                ChessPiece { kind: PieceType::Bishop, player: Player::Black },
                ChessPiece { kind: PieceType::Knight, player: Player::Black },
                ChessPiece { kind: PieceType::Rook, player: Player::Black }
              ],
              [
                ChessPiece { kind: PieceType::Pawn, player: Player::Black },
                ChessPiece { kind: PieceType::Pawn, player: Player::Black },
                ChessPiece { kind: PieceType::Pawn, player: Player::Black },
                ChessPiece { kind: PieceType::Pawn, player: Player::Black },
                ChessPiece { kind: PieceType::Pawn, player: Player::Black },
                ChessPiece { kind: PieceType::Pawn, player: Player::Black },
                ChessPiece { kind: PieceType::Pawn, player: Player::Black },
                ChessPiece { kind: PieceType::Pawn, player: Player::Black }
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
                ChessPiece { kind: PieceType::Pawn, player: Player::White },
                ChessPiece { kind: PieceType::Pawn, player: Player::White },
                ChessPiece { kind: PieceType::Pawn, player: Player::White },
                ChessPiece { kind: PieceType::Pawn, player: Player::White },
                ChessPiece { kind: PieceType::Pawn, player: Player::White },
                ChessPiece { kind: PieceType::Pawn, player: Player::White },
                ChessPiece { kind: PieceType::Pawn, player: Player::White },
                ChessPiece { kind: PieceType::Pawn, player: Player::White }
              ],
              [
                ChessPiece { kind: PieceType::Rook, player: Player::White },
                ChessPiece { kind: PieceType::Knight, player: Player::White },
                ChessPiece { kind: PieceType::Bishop, player: Player::White },
                ChessPiece { kind: PieceType::Queen, player: Player::White },
                ChessPiece { kind: PieceType::King, player: Player::White },
                ChessPiece { kind: PieceType::Bishop, player: Player::White },
                ChessPiece { kind: PieceType::Knight, player: Player::White },
                ChessPiece { kind: PieceType::Rook, player: Player::White }
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

        pub fn move_piece(&mut self, moveset:String) {

        }

        pub fn check_mate_player(){

        }

    }
}
