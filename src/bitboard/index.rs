use std::fmt;
use std::fmt::Formatter;

use crate::bitboard::BitBoard;

/// Chess field index.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub struct Index(pub usize);

impl Index {
    /// Returns index object from notation, or None if notation is outside of
    /// chess notation range "A1".."H8".
    pub fn from_notation(notation: &str) -> Option<Self> {
        // validate
        if notation.len() != 2 {
            return None;
        }

        let lower = notation.to_ascii_lowercase();

        let n = lower.as_bytes();
        let c1 = n[0];
        let c2 = n[1];

        // validate first char
        if !(b'a'..=b'h').contains(&c1) {
            return None;
        }

        // validate second char
        if !(b'1'..=b'8').contains(&c2) {
            return None;
        }

        let i = (c1 - b'a') + ((c2 - b'1') << 3);

        Some(Index(i as usize))
    }

    /// File of the chess field represented by this index.
    #[inline(always)]
    pub fn file(self) -> usize {
        self.0 % 8
    }

    /// Rank of the chess field represented by this index.
    #[inline(always)]
    pub fn rank(self) -> usize {
        self.0 / 8
    }

    /// Index as BitBoard.
    #[inline(always)]
    pub fn as_bitboard(self) -> BitBoard {
        BitBoard(1 << self.0)
    }

    /// Abs distance to other index.
    #[inline(always)]
    pub fn distance_to(self, other: Index) -> usize {
        if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        }
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (b'a' + (self.file() as u8)) as char,
            1 + self.rank()
        )?;
        Ok(())
    }
}

impl std::ops::Sub<usize> for Index {
    type Output = Index;

    fn sub(self, rhs: usize) -> Self::Output {
        Index(self.0 - rhs)
    }
}

impl std::ops::Add<usize> for Index {
    type Output = Index;

    fn add(self, rhs: usize) -> Self::Output {
        Index(self.0 + rhs)
    }
}
