mod generator_king;
mod generator_knight;
mod generator_pawn;

#[cfg(test)]
mod generator_king_tests;
#[cfg(test)]
mod generator_knight_tests;
#[cfg(test)]
mod generator_pawn_tests;

use lazy_static::lazy_static;

use crate::bitboard::BitBoard;
use crate::chessboard::{ChessBoard, Color, Move};
use crate::generator::generator_king::GeneratorKing;
use crate::generator::generator_knight::GeneratorKnight;
use crate::generator::generator_pawn::GeneratorPawn;

lazy_static! {
    pub static ref G: Generator = Generator::new();
}

pub struct Generator {
    generator_king: GeneratorKing,
    generator_pawn: GeneratorPawn,
    generator_knight: GeneratorKnight,
}

impl Generator {
    fn new() -> Self {
        Generator {
            generator_king: GeneratorKing::new(),
            generator_pawn: GeneratorPawn::new(),
            generator_knight: GeneratorKnight::new(),
        }
    }

    pub fn generate_moves(&self, board: &ChessBoard, m: &mut impl FnMut(Move)) {
        self.generator_king.generate_moves(self, board, m);
        self.generator_knight.generate_moves(board, m);
        self.generator_pawn.generate_moves(board, m);
    }

    pub fn generate_attacks(&self, board: &ChessBoard, color: Color) -> BitBoard {
        self.generator_king.generate_attacks(board, color)
            | self.generator_knight.generate_attacks(board, color)
            | self.generator_pawn.generate_attacks(board, color)
    }

    fn is_bitmask_under_attack(&self, board: &ChessBoard, color: Color, b: BitBoard) -> bool {
        self.generator_king.generate_attacks(board, color) & b != BitBoard::EMPTY
            || self.generator_knight.generate_attacks(board, color) & b != BitBoard::EMPTY
            || self.generator_pawn.generate_attacks(board, color) & b != BitBoard::EMPTY
    }
}
