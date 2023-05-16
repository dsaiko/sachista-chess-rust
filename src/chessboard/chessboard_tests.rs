use crate::bitboard::Index;
use crate::chessboard::{ChessBoard, Color};

#[test]
fn pieces() {
    let board = ChessBoard::new_standard_board();
    let white = board.pieces(Color::White);
    let black = board.pieces(Color::Black);

    assert_eq!(
        white.to_string(),
        "  a b c d e f g h
8 - - - - - - - - 8
7 - - - - - - - - 7
6 - - - - - - - - 6
5 - - - - - - - - 5
4 - - - - - - - - 4
3 - - - - - - - - 3
2 x x x x x x x x 2
1 x x x x x x x x 1
  a b c d e f g h
"
    );

    assert_eq!(
        black.to_string(),
        "  a b c d e f g h
8 x x x x x x x x 8
7 x x x x x x x x 7
6 - - - - - - - - 6
5 - - - - - - - - 5
4 - - - - - - - - 4
3 - - - - - - - - 3
2 - - - - - - - - 2
1 - - - - - - - - 1
  a b c d e f g h
"
    );
}

#[test]
fn standard_board() {
    let board = ChessBoard::new_standard_board();

    assert_eq!(32, board.all_pieces().popcnt());
    assert_eq!(16, board.my_pieces().popcnt());
    assert_eq!(16, board.opponent_pieces().popcnt());
    assert_eq!(64 - 16, board.board_to_attack().popcnt());

    assert_eq!(Index::E1, board.my_king().unwrap());
    assert_eq!(Index::E8, board.opponent_king().unwrap());
}
