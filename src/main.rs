mod board;
mod piece;
mod chesspiece;
mod player;
mod input;
mod coordinates;

use player::player::Player;

fn main() {

    /*

    MIENTRAS (NO JAQUE MATE)
  - Tomar input del jugador (E.G A1, B5). (X)
  - Mover las piezas como quiere el jugador.
  - Verificar si el programa esta en jaque mate.
  - Dibujar tabla.
  - Mover piezas de la computadora.
  - Verificar si el programa esta en jaque mate.
  FIN


        MOVER LAS PIEZAS: (TABLERO)
            - Verificar que haya una pieza en la posición buscada.
            - Verificar que esa pieza pueda moverse a la posición deseada (este en su rango de movimientos , la posición deseada exista y no haya otra pieza en esa posición).
               - DE LO CONTRARIO PEDIR INPUT, ESPECIFICANDO EL ERROR.
           - REALIZAR MOVIDA


        REALIZAR MOVIDA (TABLERO):
          - Verificar si la pieza “come” a otra pieza.
              - Si es asi, reemplazar esa pieza por un espacio vacío.
          - Reemplazar el espacio vacío al que muevo por la pieza.
          - Reemplazar el lugar donde solía estar la pieza por un espacio vacío.
*/

    // TODO:
    // Instantiate and initialize chessboard.
    let mut check_mate_player:Player = Player::None;
    let mut check_mate:bool = false;
    let mut chess_board = board::board::Board{ positions: vec![vec![]], computer_pieces: vec![] };
    chess_board.reset();
    chess_board.draw();

    while !check_mate {

      // Get input from player.
      let input = input::input::get_move_from_user();
      let split:Vec<&str> = input.split(",").collect();
      let origin:String = split[0].to_string();
      let destination:String = split[1].to_string();

      //Move piece if posible.
      chess_board.move_piece(origin, destination);

      // Draw board.
      chess_board.draw();

      // Check if program is in check mate.
      check_mate_player = chess_board.check_mate_player();
      check_mate = check_mate_player != Player::None;

      if !check_mate {
        chess_board.computer_move();
      }

    }

    if check_mate_player == Player::User {
        println!("You win");
    }
    else {
        println!("You lose! (Computer wins!)");
    }

    // Get user input.
}
