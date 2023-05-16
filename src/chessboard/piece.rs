use std::fmt;
use strum_macros::{EnumCount, EnumIter};

use crate::chessboard::Color;

/// Chess piece.
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumCount, EnumIter)]
pub enum Piece {
    King = 0,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}
impl Piece {
    /// Creates piece form string.
    pub fn from_char(c: char) -> Option<(Color, Piece)> {
        match c {
            'k' => Some((Color::Black, Piece::King)),
            'q' => Some((Color::Black, Piece::Queen)),
            'b' => Some((Color::Black, Piece::Bishop)),
            'n' => Some((Color::Black, Piece::Knight)),
            'r' => Some((Color::Black, Piece::Rook)),
            'p' => Some((Color::Black, Piece::Pawn)),
            'K' => Some((Color::White, Piece::King)),
            'Q' => Some((Color::White, Piece::Queen)),
            'B' => Some((Color::White, Piece::Bishop)),
            'N' => Some((Color::White, Piece::Knight)),
            'R' => Some((Color::White, Piece::Rook)),
            'P' => Some((Color::White, Piece::Pawn)),
            _ => None,
        }
    }

    /// Returns piece representation with respect of color
    pub fn to_colored_string(self, color: Color) -> String {
        let c = self.to_string();
        if color == Color::White {
            c.to_uppercase()
        } else {
            c
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::King => write!(f, "k"),
            Piece::Queen => write!(f, "q"),
            Piece::Bishop => write!(f, "b"),
            Piece::Knight => write!(f, "n"),
            Piece::Rook => write!(f, "r"),
            Piece::Pawn => write!(f, "p"),
        }
    }
}
