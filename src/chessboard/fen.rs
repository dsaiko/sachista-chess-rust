use std::fmt::Write;
use std::str;
use strum::IntoEnumIterator;

use crate::bitboard::{BitBoard, Index};
use crate::chessboard::{ChessBoard, Color, Piece};

impl ChessBoard {
    /// Returns FEN for this board.
    pub fn to_fen(self) -> String {
        let mut pieces = Vec::new();

        for c in Color::iter() {
            for p in Piece::iter() {
                pieces.push((
                    self.pieces[c][p].mirrored_vertically(),
                    p.to_colored_string(c),
                ))
            }
        }

        let mut fen = String::new();
        let mut spaces: usize = 0;

        macro_rules! output_spaces {
            () => {
                if spaces > 0 {
                    write!(fen, "{}", spaces).unwrap();
                    spaces = 0;
                }
            };
        }

        for i in 0..BitBoard::FIELDS.len() {
            if i > 0 && (i % 8) == 0 {
                output_spaces!();
                write!(fen, "/").unwrap();
            }

            if let Some((_, c)) = pieces.iter().find(|p| (p.0 .0 & (1 << i)) != 0) {
                output_spaces!();
                write!(fen, "{}", c).unwrap();
            } else {
                spaces += 1;
            }
        }
        output_spaces!();

        // next move color
        write!(fen, " {} ", self.next_move).unwrap();

        // Castling
        let mut some_castling = false;
        for c in Color::iter() {
            if self.castling_options[c].king_side {
                write!(fen, "{}", Piece::King.to_colored_string(c)).unwrap();
                some_castling = true;
            }
            if self.castling_options[c].queen_side {
                write!(fen, "{}", Piece::Queen.to_colored_string(c)).unwrap();
                some_castling = true;
            }
        }
        if !some_castling {
            write!(fen, "-").unwrap();
        }
        write!(fen, " ").unwrap();

        // enPassant
        if let Some(target) = self.en_passant_target {
            write!(fen, "{} ", target).unwrap();
        } else {
            write!(fen, "- ").unwrap();
        }

        // clock + move number
        write!(fen, "{} {}", self.half_move_clock, self.full_move_number).unwrap();

        fen
    }

    /// Creates ChessBoard from FEN string.
    pub fn from_fen(fen: &str) -> Option<ChessBoard> {
        //"8/8/8/8/8/8/8/8"
        if fen.len() < 15 {
            return None;
        }

        let mut i: usize = 0;
        let mut b = ChessBoard::new();

        let chars = fen.as_bytes();

        macro_rules! shift_board {
            ($a:expr) => {
                for c in Color::iter() {
                    for p in Piece::iter() {
                        b.pieces[c][p] <<= $a;
                    }
                }
            };
        }

        // pieces
        while i < chars.len() {
            let c = chars[i];
            i += 1;

            if c == b' ' {
                break;
            }

            if c == b'/' {
                // nothing
                continue;
            }

            if c.is_ascii_digit() {
                // shift by number of empty fields
                shift_board!(c - b'0');
            } else {
                // shift all pieces by 1
                shift_board!(1);

                let (c, p) = Piece::from_char(c as char)?;
                b.pieces[c][p] |= 1
            }
        }

        // need to mirror the boards
        for c in Color::iter() {
            for p in Piece::iter() {
                b.pieces[c][p] = b.pieces[c][p].mirrored_horizontally();
            }
        }

        // next move
        if i < chars.len() {
            match chars[i] {
                b'w' => b.next_move = Color::White,
                b'b' => b.next_move = Color::Black,
                _ => return None,
            }
            i += 1;
        }

        // castling
        i += 1;
        while i < chars.len() {
            let c = chars[i];
            i += 1;

            if c == b' ' {
                break;
            }

            if let Some((color, piece)) = Piece::from_char(c as char) {
                match piece {
                    Piece::King => b.castling_options[color].king_side = true,
                    Piece::Queen => b.castling_options[color].queen_side = true,
                    _ => return None,
                }
            }
        }

        // enPassant
        let mut notation = String::new();
        while i < chars.len() {
            let c = chars[i];
            i += 1;

            if c == b' ' {
                break;
            }

            if c != b'-' && notation.len() < 2 {
                notation.push(c as char);
            }
            if notation.len() == 2 {
                b.en_passant_target = Index::from_notation(&notation);
            }
        }

        // half move clock
        let mut n: usize = 0;
        while i < chars.len() {
            let c = chars[i];
            i += 1;

            if c == b' ' {
                break;
            }

            if c.is_ascii_digit() {
                n = n * 10 + (c - b'0') as usize;
            }
        }
        if n > 0 {
            b.half_move_clock = n;
        }

        // full move number
        n = 0;
        while i < chars.len() {
            let c = chars[i];
            i += 1;

            if c == b' ' {
                break;
            }

            if c.is_ascii_digit() {
                n = n * 10 + (c - b'0') as usize;
            }
        }
        if n > 0 {
            b.full_move_number = n;
        }

        // fix castling - rooks
        if (b.pieces[Color::White][Piece::Rook] & BitBoard::A1) == BitBoard::EMPTY {
            b.castling_options[Color::White].queen_side = false;
        }
        if (b.pieces[Color::White][Piece::Rook] & BitBoard::H1) == BitBoard::EMPTY {
            b.castling_options[Color::White].king_side = false;
        }
        if (b.pieces[Color::Black][Piece::Rook] & BitBoard::A8) == BitBoard::EMPTY {
            b.castling_options[Color::Black].queen_side = false;
        }
        if (b.pieces[Color::Black][Piece::Rook] & BitBoard::H8) == BitBoard::EMPTY {
            b.castling_options[Color::Black].king_side = false;
        }

        // fix castling - kings
        if (b.pieces[Color::White][Piece::King] & BitBoard::E1) == BitBoard::EMPTY {
            b.castling_options[Color::White].king_side = false;
            b.castling_options[Color::White].queen_side = false;
        }
        if (b.pieces[Color::Black][Piece::King] & BitBoard::E8) == BitBoard::EMPTY {
            b.castling_options[Color::Black].king_side = false;
            b.castling_options[Color::Black].queen_side = false;
        }

        b.update_hash();

        Some(b)
    }
}
