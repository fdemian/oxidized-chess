pub mod board {

    const BOARD_LIMIT:usize = 8;

    pub use crate::piece::piece::PieceType;
    pub use crate::player::player::Player;
    pub use crate::chesspiece::chesspiece::ChessPiece;

    pub struct Board {
     pub positions: Vec<Vec<ChessPiece>>
    }

    impl Board {

        pub fn reset(&mut self) {

          let INITIAL_STATE:Vec<Vec<ChessPiece>> = vec![

              vec![
                ChessPiece { kind: PieceType::Rook, player: Player::Computer },
                ChessPiece { kind: PieceType::Knight, player :Player::Computer },
                ChessPiece { kind: PieceType::Bishop, player: Player::Computer },
                ChessPiece { kind: PieceType::Queen, player: Player::Computer },
                ChessPiece { kind: PieceType::King, player: Player::Computer },
                ChessPiece { kind: PieceType::Bishop, player: Player::Computer },
                ChessPiece { kind: PieceType::Knight, player: Player::Computer },
                ChessPiece { kind: PieceType::Rook, player: Player::Computer }
              ],
              vec![
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer },
                ChessPiece { kind: PieceType::Pawn, player: Player::Computer }
              ],
              vec![ // Row 3
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None }
              ],
              vec![ // Row 4
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None }
              ],
              vec![   // Row 5
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None }
              ],
              vec![
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None },
                ChessPiece { kind: PieceType::Empty, player: Player::None }
              ],
              vec![
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User },
                ChessPiece { kind: PieceType::Pawn, player: Player::User }
              ],
              vec![
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
                let piece = &self.positions[r][c];
                print!("[{0}]", piece.get_text_repr());
             }
             println!(" ");
         }

        }

        pub fn is_valid_move(&mut self, origin:&Vec<u32>, dest:&Vec<u32>) -> bool {

            // If the player wants to move a piece outside of the board.
            // Return false.
            if origin[0] > 7 || origin[1] >7 || dest[0] > 7 || dest[1] > 7 {
                return false;
            }

            let row:usize = origin[0] as usize;
            let col:usize = origin[1] as usize;
            let current_piece = &self.positions[row][col];

            // The user wants to move a piece that does not exist.
            if current_piece.kind == PieceType::Empty {
                return false;
            }

            // Piece exists and the user wants to move the piece inside the board.

            // GET PIECE MOVES (LEGAL PATHS) and check
            //if the destination coordintes coincide with the last element of the path.
            // if they do , return true, else return false.

            return true;
        }

        // Transforms something like A1 TO [0, 0]
        pub fn get_coordinates(&mut self, positions:&String) -> Vec<u32> {
            const RADIX: u32 = 10;
            let first_char = positions.chars().nth(0).unwrap();
            let second_char = positions.chars().nth(1).unwrap();

            let column:u32 = match first_char {
              'A' => 0,
              'B' => 1,
              'C' => 2,
              'D' => 3,
              'E' => 4,
              'F' => 5,
              'G' => 6,
              'H' => 7,
               _  => 0
            };
            let row:u32 = second_char.to_digit(RADIX).unwrap();
            let coordinates = vec![(row-1), column];

            return coordinates;
        }

        pub fn move_piece(&mut self, origin:String, destination:String) -> bool {

            let origin_coords:Vec<u32> = self.get_coordinates(&origin);
            let dest_coords:Vec<u32> = self.get_coordinates(&destination);

            if !self.is_valid_move(&origin_coords, &dest_coords) {
                return false;
            }

            let row:usize = dest_coords[0] as usize;
            let col:usize = dest_coords[1] as usize;

            if self.positions[row][col].kind == PieceType::Empty {
               self.perform_move(origin_coords, dest_coords);
            }

            return true;
        }

        pub fn perform_move(&mut self, origin:Vec<u32>, dest:Vec<u32>) {

            let empty_space:ChessPiece = ChessPiece {
              kind: PieceType::Empty,
              player: Player::None
            };
            let origin_row:usize = origin[0] as usize;
            let origin_col:usize = origin[1] as usize;

            let dest_row:usize = dest[0] as usize;
            let dest_col:usize = dest[1] as usize;

            let piece:ChessPiece = self.positions[origin_row][origin_col];

            // Move the piece to its destination.
            //Fill the previous location with empty space.
            self.positions[dest_row][dest_col] = piece;
            self.positions[origin_row][origin_col] = empty_space;

        }

        pub fn check_mate_player(&self) -> Player {
            return Player::None;
        }

        pub fn computer_move(&self) {

        }

    }
}
