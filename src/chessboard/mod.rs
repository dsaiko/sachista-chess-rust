pub use self::castling_options::CastlingOptions;
pub use self::color::Color;
pub use self::moves::Move;
pub use self::moves::Specifier;
pub use self::piece::Piece;
pub use self::zobrist::Zobrist;

mod board_indices;
mod castling_options;
mod color;
mod fen;
mod moves;
mod piece;
mod string;
mod zobrist;

use strum::EnumCount;

use crate::bitboard::{BitBoard, Index};

#[cfg(test)]
mod chessboard_tests;
#[cfg(test)]
mod color_tests;
#[cfg(test)]
mod fen_tests;
#[cfg(test)]
mod moves_tests;
#[cfg(test)]
mod piece_tests;
#[cfg(test)]
mod string_tests;
#[cfg(test)]
mod zobrist_tests;

/// ChessBoard representation.
#[derive(Debug, Copy, Clone)]
pub struct ChessBoard {
    pub pieces: [[BitBoard; Piece::COUNT]; Color::COUNT],
    pub next_move: Color,
    pub castling_options: [CastlingOptions; Color::COUNT],
    pub en_passant_target: Option<Index>,
    pub half_move_clock: usize,
    pub full_move_number: usize,
    pub hash: u64,
}

impl ChessBoard {
    pub const STANDARD_BOARD_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    /// Creates a new empty chessboard.
    pub fn new() -> Self {
        ChessBoard {
            pieces: [[BitBoard::EMPTY; Piece::COUNT]; Color::COUNT],
            next_move: Color::White,
            castling_options: [CastlingOptions {
                king_side: false,
                queen_side: false,
            }; Color::COUNT],
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 1,
            hash: 0,
        }
    }

    /// Creates a new chessboard initialized to the standard setup.
    #[rustfmt::skip]
    pub fn new_standard_board() -> Self {
        let mut b = ChessBoard::new();

        b.pieces[Color::White][Piece::King] = BitBoard::E1;
        b.pieces[Color::White][Piece::Queen] = BitBoard::D1;
        b.pieces[Color::White][Piece::Bishop] = BitBoard::C1 | BitBoard::F1;
        b.pieces[Color::White][Piece::Knight] = BitBoard::B1 | BitBoard::G1;
        b.pieces[Color::White][Piece::Rook] = BitBoard::A1 | BitBoard::H1;
        b.pieces[Color::White][Piece::Pawn] = BitBoard::A2 | BitBoard::B2 | BitBoard::C2 | BitBoard::D2 | BitBoard::E2 | BitBoard::F2 | BitBoard::G2 | BitBoard::H2;

        b.pieces[Color::Black][Piece::King] = BitBoard::E8;
        b.pieces[Color::Black][Piece::Queen] = BitBoard::D8;
        b.pieces[Color::Black][Piece::Bishop] = BitBoard::C8 | BitBoard::F8;
        b.pieces[Color::Black][Piece::Knight] = BitBoard::B8 | BitBoard::G8;
        b.pieces[Color::Black][Piece::Rook] = BitBoard::A8 | BitBoard::H8;
        b.pieces[Color::Black][Piece::Pawn] = BitBoard::A7 | BitBoard::B7 | BitBoard::C7 | BitBoard::D7 | BitBoard::E7 | BitBoard::F7 | BitBoard::G7 | BitBoard::H7;

        b.castling_options[Color::White] = CastlingOptions {
            king_side: true,
            queen_side: true,
        };

        b.castling_options[Color::Black] = CastlingOptions {
            king_side: true,
            queen_side: true,
        };

        b.update_hash();

        b
    }

    #[inline(always)]
    /// Update chessboard hash.
    pub fn update_hash(&mut self) {
        self.hash = zobrist::Z.hash(self);
    }

    /// Returns bitboard of all pieces of one color.
    #[inline(always)]
    pub fn pieces(&self, color: Color) -> BitBoard {
        let b = self.pieces[color].iter().fold(0u64, |res, val| res | val.0);

        BitBoard(b)
    }

    /// Returns bitboard of all pieces.
    #[inline(always)]
    pub fn all_pieces(&self) -> BitBoard {
        self.pieces(Color::White) | self.pieces(Color::Black)
    }

    /// Returns bitboard of next_move pieces.
    #[inline(always)]
    pub fn my_pieces(&self) -> BitBoard {
        self.pieces(self.next_move)
    }

    /// Returns bitboard of opponent pieces.
    #[inline(always)]
    pub fn opponent_pieces(&self) -> BitBoard {
        self.pieces(self.next_move.opponent())
    }

    /// Returns bitboard of available attacks.
    #[inline(always)]
    pub fn board_to_attack(&self) -> BitBoard {
        !self.my_pieces()
    }

    /// Returns index of next_move king.
    #[inline(always)]
    pub fn my_king(&self) -> Option<Index> {
        self.pieces[self.next_move][Piece::King].bitscan()
    }

    /// Returns index of opponent king.
    #[inline(always)]
    pub fn opponent_king(&self) -> Option<Index> {
        self.pieces[self.next_move.opponent()][Piece::King].bitscan()
    }
}
