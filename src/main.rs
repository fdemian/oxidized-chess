mod board;
mod piece;
mod chesspiece;
mod player;
mod input;

extern crate ndarray;
use ndarray::array;

fn main() {

    /*

        - Mostrar estado inicial.

        MIENTRAS (NO JAQUE MATE)
        - Tomar input del jugador (E.G A1, B5).
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
    let mut chess_board = board::board::Board{ positions: array![[]] };
    chess_board.reset();
    chess_board.draw();

    // Get user input.
    let input = input::input::get_move_from_user();
    chess_board.move_piece(input);

    chess_board.draw();
}
