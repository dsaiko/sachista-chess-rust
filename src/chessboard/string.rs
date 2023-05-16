use regex::Regex;
use std::fmt;
use strum::IntoEnumIterator;

use crate::bitboard::BitBoard;
use crate::chessboard::{ChessBoard, Color, Piece};

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const HEADER: &str = "  a b c d e f g h\n";
        write!(f, "{}", HEADER)?;

        let mut pieces = Vec::new();

        for c in Color::iter() {
            for p in Piece::iter() {
                pieces.push((
                    self.pieces[c][p].mirrored_vertically(),
                    p.to_colored_string(c),
                ))
            }
        }

        for i in 0..BitBoard::FIELDS.len() {
            if (i % 8) == 0 {
                if i > 0 {
                    writeln!(f, "{}", 9 - (i / 8))?;
                }

                write!(f, "{} ", 8 - (i / 8))?;
            }

            if let Some((_, c)) = pieces.iter().find(|p| (p.0 .0 & (1 << i)) != 0) {
                write!(f, "{} ", c)?;
            } else {
                write!(f, "- ")?;
            }
        }

        write!(f, "1\n{}", HEADER)?;

        Ok(())
    }
}

impl ChessBoard {
    /// Creates new board form a string.
    pub fn from_string(str: &str) -> Option<ChessBoard> {
        let re = Regex::new(r"a b c d e f g h|[0-9 \n]").unwrap();
        let mut pieces = str.to_string();

        pieces = re.replace_all(&pieces, "").to_string();
        if pieces.len() != BitBoard::FIELDS.len() {
            return None;
        }
        pieces = pieces.replace('-', "1");

        pieces += " w KQkq - 0 1";

        ChessBoard::from_fen(&pieces)
    }
}
