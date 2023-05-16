use strum::EnumCount;

use crate::bitboard::{BitBoard, Index};
use crate::chessboard::{CastlingOptions, Color, Piece};

impl std::ops::Index<Color> for [[BitBoard; 6]; 2] {
    type Output = [BitBoard; 6];

    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl std::ops::IndexMut<Color> for [[BitBoard; 6]; 2] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl std::ops::Index<Color> for [CastlingOptions] {
    type Output = CastlingOptions;

    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl std::ops::IndexMut<Color> for [CastlingOptions] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl std::ops::Index<Piece> for [BitBoard] {
    type Output = BitBoard;

    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

impl std::ops::IndexMut<Piece> for [BitBoard] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl std::ops::Index<Color> for [[[u64; 64]; 6]; 2] {
    type Output = [[u64; 64]; 6];

    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl std::ops::IndexMut<Color> for [[[u64; 64]; 6]; 2] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl std::ops::Index<Piece> for [[u64; 64]; 6] {
    type Output = [u64; 64];

    fn index(&self, index: Piece) -> &Self::Output {
        &self[index as usize]
    }
}

impl std::ops::IndexMut<Piece> for [[u64; 64]; 6] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl std::ops::Index<Color> for [[u64; 2]; 2] {
    type Output = [u64; 2];

    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl std::ops::IndexMut<Color> for [[u64; 2]; 2] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl std::ops::Index<Index> for [u64; 64] {
    type Output = u64;

    fn index(&self, index: Index) -> &Self::Output {
        &self[index.0]
    }
}

impl std::ops::IndexMut<Index> for [u64; 64] {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl std::ops::Index<Color> for [[BitBoard; Index::FIELDS.len()]; Color::COUNT] {
    type Output = [BitBoard; Index::FIELDS.len()];

    fn index(&self, index: Color) -> &Self::Output {
        &self[index as usize]
    }
}

impl std::ops::IndexMut<Color> for [[BitBoard; Index::FIELDS.len()]; Color::COUNT] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl std::ops::Index<Index> for [BitBoard] {
    type Output = BitBoard;

    fn index(&self, index: Index) -> &Self::Output {
        &self[index.0]
    }
}

impl std::ops::IndexMut<Index> for [BitBoard] {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self[index.0]
    }
}
