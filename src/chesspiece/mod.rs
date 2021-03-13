pub mod chesspiece {

    pub use crate::piece::piece::PieceType;
    pub use crate::player::player::Player;

    pub struct ChessPiece {
        pub kind: PieceType,
        pub player: Player
    }

    impl ChessPiece {

        /*
        fn get_name(&self) -> String {
            match self.kind {
             PieceType::Pawn => 'Pawn',
             PieceType::Bishop => '♝',
             PieceType::Knight => '♞',
             PieceType::Rook => '♜',
             PieceType::Queen => '♛',
             PieceType::King => '♚',
             PieceType::Empty => ' '
           }
       }*/

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
