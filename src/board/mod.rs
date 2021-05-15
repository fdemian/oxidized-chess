pub mod board {

    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use rand::Rng;

    const BOARD_LIMIT:usize = 8;

    pub use crate::piece::piece::PieceType;
    pub use crate::player::player::Player;
    pub use crate::chesspiece::chesspiece::ChessPiece;
    pub use crate::coordinates::coordinates::Coordinates;

    pub struct Board {
     pub positions: Vec<Vec<ChessPiece>>,
     pub computer_pieces: Vec<Coordinates>,
    }

    impl Board {

        pub fn get_char_from_number(&self, character:&u32) -> char {
            let column:char = match character {
              0 => 'A',
              1 => 'B',
              2 => 'C',
              3 => 'D',
              4 => 'E',
              5 => 'F',
              6 => 'G',
              7 => 'H',
              _  => '_'
            };

            return column;
        }

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

            // Initial board positions and computer pices coordinates.
            self.positions = INITIAL_STATE;
            self.computer_pieces= vec![
                Coordinates{ row: 0, col: 0},
                Coordinates{ row: 0, col: 1},
                Coordinates{ row: 0, col: 2},
                Coordinates{ row: 0, col: 3},
                Coordinates{ row: 0, col: 4},
                Coordinates{ row: 0, col: 5},
                Coordinates{ row: 0, col: 6},
                Coordinates{ row: 0, col: 7},
                Coordinates{ row: 1, col: 0},
                Coordinates{ row: 1, col: 1},
                Coordinates{ row: 1, col: 2},
                Coordinates{ row: 1, col: 3},
                Coordinates{ row: 1, col: 4},
                Coordinates{ row: 1, col: 5},
                Coordinates{ row: 1, col: 6},
                Coordinates{ row: 1, col: 7},
            ];
        }

        pub fn draw(&mut self) {

          // Iterate rows
          for r in 0..BOARD_LIMIT {
             for c in 0..BOARD_LIMIT {
                let piece = &self.positions[r][c];
                if c == 0 {
                 let positon_fmt:char = self.get_char_from_number(&(c as u32));
                 print!("{0} [{1}]", r+1, piece.get_text_repr());
                }
                else {
                  print!(" [{0}]", piece.get_text_repr());
                }
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
            //if the destination coordintes coincide with any element of the path.
            // if they do , return true, else return false.

           // get_moves(origin);

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


        // Get a random computer piece and move it.
        pub fn computer_move(&mut self) {

            let mut rand_piece:ChessPiece = ChessPiece {
                kind: PieceType::Empty,
                player: Player::None
            };
            let mut rand_coord:Coordinates = Coordinates { row: 0, col: 0};
            let mut rand_index = 0;

            //Get a random (non empty) computer piece.
            while rand_piece.kind == PieceType::Empty {
              let len = self.computer_pieces.len();
              let mut rng = rand::thread_rng();

              rand_index = rng.gen_range(0..len-1);
              rand_coord = self.computer_pieces[rand_index];
              rand_piece = self.positions[(rand_coord.row as usize)][(rand_coord.col as usize)];

              println!("{0}", rand_piece.kind == PieceType::Empty);
              println!("{0}", rand_piece.get_text_repr());

            }

            // Get all posible moves for that piece.
            let coordinate_moves:Vec<u32> = vec![rand_coord.row, rand_coord.col];
            let moves = rand_piece.get_moves(&coordinate_moves);

            // Pick a random move.
            let mut rng2 = rand::thread_rng();
            let move_len = moves.len();
            let mut rand_index2 = 0;

            if move_len != 1 {
               rand_index2 = rng2.gen_range(0..move_len-1);
            }

            let move_coord:Coordinates = moves[rand_index2];
            let comp_move_dest:Vec<u32> = vec![move_coord.col, move_coord.row];

            let mut c0 = coordinate_moves[0];
            let mut c1 = coordinate_moves[1];
            println!("[{0}, {1}]", c0, c1);
            println!("{0}", self.positions[c0 as usize][c1 as usize].get_text_repr());
            println!(":::::::");
            c0 = comp_move_dest[0];
            c1 =  comp_move_dest[1];
            println!("[{0}, {1}]", c0, c1);
            println!("{0}", self.positions[c0 as usize][c1 as usize].get_text_repr());
            println!(":::::::");

            // Perform the move.
            self.perform_move(coordinate_moves ,comp_move_dest);
            self.computer_pieces[rand_index] = move_coord;
        }

    }
}
