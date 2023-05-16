pub use self::index::Index;

mod bitboard_constants;
mod bitboard_operators;
mod index;
mod index_constants;
mod string;

#[cfg(test)]
mod bitboard_constants_tests;
#[cfg(test)]
mod bitboard_tests;
#[cfg(test)]
mod index_tests;

/// Chessboard bit representation.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    /// Creates new board form list of Indices.
    pub fn from_indices(indices: Vec<Index>) -> BitBoard {
        let mut b = BitBoard::EMPTY;

        for i in indices {
            b |= i.as_bitboard();
        }
        b
    }

    /// Returns number of set fields in the board.
    #[inline(always)]
    pub fn popcnt(self) -> usize {
        self.0.count_ones() as usize
    }

    /// Returns next index of non-empty bit.
    #[inline(always)]
    pub fn bitscan(self) -> Option<Index> {
        if self == BitBoard::EMPTY {
            return None;
        }
        Some(Index(self.0.trailing_zeros() as usize))
    }

    /// Returns index of first set bit and resets this bit in the returned board.
    #[inline(always)]
    pub fn bitpop(self) -> (Option<Index>, BitBoard) {
        if self == BitBoard::EMPTY {
            return (None, BitBoard::EMPTY);
        }

        let b = self.0;
        (self.bitscan(), BitBoard(b & (b - 1)))
    }

    /// Shifts all existing Board pieces by one.
    #[inline(always)]
    pub fn shifted_north(self) -> BitBoard {
        self << 8
    }

    /// Shifts all existing Board pieces by one.
    #[inline(always)]
    pub fn shifted_south(self) -> BitBoard {
        self >> 8
    }

    /// Shifts all existing Board pieces by one.
    #[inline(always)]
    pub fn shifted_east(self) -> BitBoard {
        (self << 1) & !BitBoard::FILE_A
    }

    /// Shifts all existing Board pieces by one.
    #[inline(always)]
    pub fn shifted_northeast(self) -> BitBoard {
        (self << 9) & !BitBoard::FILE_A
    }

    /// Shifts all existing Board pieces by one.
    #[inline(always)]
    pub fn shifted_southeast(self) -> BitBoard {
        (self >> 7) & !BitBoard::FILE_A
    }

    /// Shifts all existing Board pieces by one.
    #[inline(always)]
    pub fn shifted_west(self) -> BitBoard {
        (self >> 1) & !BitBoard::FILE_H
    }

    /// Shifts all existing Board pieces by one.
    #[inline(always)]
    pub fn shifted_southwest(self) -> BitBoard {
        (self >> 9) & !BitBoard::FILE_H
    }

    /// Shifts all existing Board pieces by one.
    #[inline(always)]
    pub fn shifted_northwest(self) -> BitBoard {
        (self << 7) & !BitBoard::FILE_H
    }

    /// Shifts all existing Board pieces by multiple steps.
    pub fn shifted(self, dx: isize, dy: isize) -> BitBoard {
        let mut b = self.0;

        // dy = up/down
        if dy > 0 {
            b <<= dy * 8;
        }
        if dy < 0 {
            b >>= (-dy) * 8;
        }

        // dx = left / right
        if dx > 0 {
            for _ in 0..dx {
                b = (b << 1) & !BitBoard::FILE_A.0;
            }
        }
        if dx < 0 {
            for _ in 0..(-dx) {
                b = (b >> 1) & !BitBoard::FILE_H.0;
            }
        }
        BitBoard(b)
    }

    /// Returns this board mirrored vertically.
    pub fn mirrored_vertically(self) -> BitBoard {
        let mut result = 0u64;
        let b = self.0;

        result |= (b >> 56) & BitBoard::RANK_1.0;
        result |= ((b >> 48) & BitBoard::RANK_1.0) << 8;
        result |= ((b >> 40) & BitBoard::RANK_1.0) << 16;
        result |= ((b >> 32) & BitBoard::RANK_1.0) << 24;
        result |= ((b >> 24) & BitBoard::RANK_1.0) << 32;
        result |= ((b >> 16) & BitBoard::RANK_1.0) << 40;
        result |= ((b >> 8) & BitBoard::RANK_1.0) << 48;
        result |= (b & BitBoard::RANK_1.0) << 56;

        BitBoard(result)
    }

    /// Returns this board mirrored horizontally.
    pub fn mirrored_horizontally(self) -> BitBoard {
        const K1: u64 = 0x5555555555555555u64;
        const K2: u64 = 0x3333333333333333u64;
        const K4: u64 = 0x0f0f0f0f0f0f0f0fu64;

        let mut b = self.0;

        b = ((b >> 1) & K1) | ((b & K1) << 1);
        b = ((b >> 2) & K2) | ((b & K2) << 2);
        b = ((b >> 4) & K4) | ((b & K4) << 4);

        BitBoard(b)
    }

    /// Returns bitboard flipped around A1H8 diagonal.
    pub fn mirrored_a1h8(self) -> BitBoard {
        const K1: u64 = 0x5500550055005500u64;
        const K2: u64 = 0x3333000033330000u64;
        const K4: u64 = 0x0f0f0f0f00000000u64;

        let mut b = self.0;

        let mut t = K4 & (b ^ (b << 28));

        b ^= t ^ (t >> 28);
        t = K2 & (b ^ (b << 14));
        b ^= t ^ (t >> 14);
        t = K1 & (b ^ (b << 7));
        b ^= t ^ (t >> 7);

        BitBoard(b)
    }

    /// Returns bitboard flipped around A8H1 diagonal.
    pub fn mirrored_a8h1(self) -> BitBoard {
        const K1: u64 = 0xaa00aa00aa00aa00u64;
        const K2: u64 = 0xcccc0000cccc0000u64;
        const K4: u64 = 0xf0f0f0f00f0f0f0fu64;

        let mut b = self.0;
        let mut t = b ^ (b << 36);

        b ^= K4 & (t ^ (b >> 36));
        t = K2 & (b ^ (b << 18));
        b ^= t ^ (t >> 18);
        t = K1 & (b ^ (b << 9));
        b ^= t ^ (t >> 9);

        BitBoard(b)
    }

    /// Converts this board to list of Indices.
    pub fn indices(self) -> Vec<Index> {
        let mut v: Vec<Index> = Vec::with_capacity(self.popcnt());

        let mut b = self;

        while let (Some(i), next) = b.bitpop() {
            b = next;
            v.push(i);
        }
        v
    }
}
