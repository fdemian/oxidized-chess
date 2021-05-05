pub mod chesspiece {

    pub use crate::piece::piece::PieceType;
    pub use crate::player::player::Player;

    #[derive(Copy, Clone)]
    pub struct ChessPiece {
        pub kind: PieceType,
        pub player: Player
    }

    impl ChessPiece {

        /*
        pub fn get_moves(&self, coordinates:Vec<Vec<u32>>) -> Vec<>
        */

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
