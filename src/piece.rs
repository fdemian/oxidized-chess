pub mod piece {
  #[derive(PartialEq)]
  #[derive(Copy, Clone)]
  pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
    Empty
 }
}
