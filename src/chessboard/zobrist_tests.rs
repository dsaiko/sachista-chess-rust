use std::collections::HashSet;

use crate::bitboard::BitBoard;
use crate::chessboard::{zobrist, ChessBoard, Color, Piece};

#[test]
fn rnd() {
    let mut values = HashSet::new();

    let mut count = 0;
    let side = zobrist::Z.side;
    values.insert(side);
    assert_ne!(side, 0);
    count += 1;

    for i1 in 0..zobrist::Z.pieces.len() {
        for i2 in 0..zobrist::Z.pieces[i1].len() {
            for i3 in 0..zobrist::Z.pieces[i1][i2].len() {
                let v = zobrist::Z.pieces[i1][i2][i3];
                values.insert(v);
                assert_ne!(v, 0);
                count += 1;
            }
        }
    }

    for i in 0..zobrist::Z.en_passant.len() {
        let v = zobrist::Z.en_passant[i];
        values.insert(v);
        assert_ne!(v, 0);
        count += 1;
    }

    for i1 in 0..zobrist::Z.castling.len() {
        for i2 in 0..zobrist::Z.castling[i1].len() {
            let v = zobrist::Z.castling[i1][i2];
            values.insert(v);
            assert_ne!(v, 0);
            count += 1;
        }
    }

    assert_eq!(count, values.len());
}

#[test]
fn zobrist() {
    let mut board = ChessBoard::new_standard_board();
    assert_eq!(board.hash, 1573147779821007145);

    board.pieces[Color::White][Piece::Pawn] ^= BitBoard::A3;
    board.pieces[Color::White][Piece::Pawn] ^= BitBoard::A2;
    board.update_hash();

    assert_eq!(board.hash, 7254632465593326825);
}
