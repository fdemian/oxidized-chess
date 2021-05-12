pub mod chesspiece {

    pub use crate::piece::piece::PieceType;
    pub use crate::player::player::Player;
    pub use crate::coordinates::coordinates::Coordinates;

    #[derive(Copy, Clone)]
    pub struct ChessPiece {
        pub kind: PieceType,
        pub player: Player
    }

    impl ChessPiece {

        /*
         * Get moves for every piece, given its current coordinates.
        */
        /*
          -- TODO
          - BISHOP
          - KNIGHT
          - ROOK
          - QUEEN
        */
        pub fn get_moves(&self, coordinates:&Vec<u32>) -> Vec<Coordinates> {
            match self.kind {
             PieceType::Pawn => {
                 vec![
                   Coordinates {
                    row: (coordinates[0]+1),
                    col: coordinates[1]
                  }
                 ]
             },
             PieceType::Bishop => {
                 vec![
                   Coordinates {
                    row: (coordinates[0]+1),
                    col: coordinates[1]
                  }
                 ]
             },
             PieceType::Knight => {
                 vec![
                   Coordinates {
                    row: (coordinates[0]+1),
                    col: coordinates[1]
                  }
                 ]
             },
             PieceType::Rook => {
                 vec![
                   Coordinates {
                    row: (coordinates[0]+1),
                    col: coordinates[1]
                  }
                 ]
             },
             PieceType::Queen => {
                 vec![
                   Coordinates {
                    row: (coordinates[0]+1),
                    col: coordinates[1]
                  }
                 ]
             },
             PieceType::King => {
                 vec![
                   Coordinates {
                    row: (coordinates[0]+1),
                    col: coordinates[1]
                   },
                   Coordinates {
                    row: (coordinates[0]-1),
                    col: (coordinates[1])
                   },
                   Coordinates {
                    row: (coordinates[0]),
                    col: (coordinates[1]+1)
                   },
                   Coordinates {
                    row: (coordinates[0]),
                    col: (coordinates[1]-1)
                   },
                   Coordinates {
                    row: (coordinates[0]+1),
                    col: (coordinates[1]+1)
                  },
                  Coordinates {
                    row: (coordinates[0]+1),
                    col: (coordinates[1]-1)
                  },
                  Coordinates {
                    row: (coordinates[0]-1),
                    col: (coordinates[1]+1)
                  },
                  Coordinates {
                    row: (coordinates[0]-1),
                    col: (coordinates[1]-1)
                  }
                ]
             },
             PieceType::Empty => {
                 vec![]
             }
           }

        }

        pub fn get_text_repr(&self) -> char {
           match self.kind {
            PieceType::Pawn => '♟',
            PieceType::Bishop => '♝',
            PieceType::Knight => '♞',
            PieceType::Rook => '♜',
            PieceType::Queen => '♛',
            PieceType::King => '♚',
            PieceType::Empty => ' '
          }
       }

   }
}
