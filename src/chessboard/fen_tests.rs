use crate::bitboard::{BitBoard, Index};
use crate::chessboard::ChessBoard;
use crate::chessboard::Color;

#[test]
fn fen() {
    let b = ChessBoard::new_standard_board();
    assert_eq!(b.to_fen(), ChessBoard::STANDARD_BOARD_FEN);

    let b2 = ChessBoard::from_fen("7B/6B1/5B2/4B3/3B4/2B5/1B6/B7 w - - 0 1").unwrap();
    assert_eq!(b2.all_pieces(), BitBoard::A1H8[7]);
}

#[test]
fn from_fen() {
    let b = ChessBoard::new_standard_board();
    assert_eq!(b.to_fen(), ChessBoard::STANDARD_BOARD_FEN);
    let b2 = ChessBoard::from_fen(&b.to_fen()).unwrap();
    assert_eq!(b2.to_fen(), ChessBoard::STANDARD_BOARD_FEN);
    assert_eq!(b2.hash, b.hash);
}

#[test]
fn from_fen_with_en_passant() {
    let mut b = ChessBoard::new_standard_board();
    b.next_move = Color::Black;
    b.castling_options[0].king_side = false;
    b.castling_options[1].queen_side = false;

    b.en_passant_target = Some(Index::E3);
    b.half_move_clock = 22;
    b.full_move_number = 31;

    let b2 = ChessBoard::from_fen(&b.to_fen()).unwrap();
    assert_eq!(b.to_fen(), b2.to_fen());
}

#[test]
fn fen_misplaced_castling() {
    let b =
        ChessBoard::from_fen("rnbqkbn1/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1BNR w KQkq - 0 1").unwrap();
    assert_eq!(
        b.to_fen(),
        "rnbqkbn1/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1BNR w q - 0 1"
    );
}

#[test]
fn incomplete_fen() {
    let b = ChessBoard::from_fen("8/1K6/1Q6/8/5r2/4rk2/8/8 b - -").unwrap();
    assert_eq!(b.all_pieces().popcnt(), 5);
    assert_eq!(b.next_move, Color::Black);
    assert_eq!(b.full_move_number, 1);
    assert_eq!(b.half_move_clock, 0);
}

#[test]
fn invalid_fen() {
    let b = ChessBoard::from_fen("XXX");
    assert!(b.is_none());

    let b2 = ChessBoard::from_fen("");
    assert!(b2.is_none());
}
