use regex::Regex;
use std::fmt;

use crate::bitboard::{BitBoard, Index};

impl BitBoard {
    /// Creates new board form a string.
    pub fn from_string(str: &str) -> Option<BitBoard> {
        let mut b = BitBoard::EMPTY;

        let re = Regex::new(r"[abcdefgh0-9 \n]").unwrap();
        let mut pieces = str.to_string().to_ascii_lowercase();

        pieces = re.replace_all(&pieces, "").to_string();
        if pieces.len() != BitBoard::FIELDS.len() {
            return None;
        }

        for (i, c) in pieces.chars().enumerate() {
            match c {
                'x' => b |= Index(i).as_bitboard(),
                '-' => {}
                _ => return None,
            }
        }

        Some(b.mirrored_vertically())
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const HEADER: &str = "  a b c d e f g h\n";

        let bitboard = self.mirrored_vertically();

        write!(f, "{}", HEADER)?;

        for i in 0..BitBoard::FIELDS.len() {
            if i % 8 == 0 {
                if i > 0 {
                    // print right column digit
                    writeln!(f, "{}", 9 - (i / 8))?;
                }

                // print left column digit
                write!(f, "{} ", 8 - (i / 8))?;
            }

            if bitboard & (1 << i) != BitBoard::EMPTY {
                write!(f, "x ")?;
            } else {
                write!(f, "- ")?;
            }
        }

        write!(f, "1\n{}", HEADER)?;

        Ok(())
    }
}
