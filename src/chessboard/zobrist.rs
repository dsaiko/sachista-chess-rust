use lazy_static::lazy_static;
use strum::EnumCount;

use crate::bitboard::BitBoard;
use crate::chessboard::{ChessBoard, Color, Piece};

pub struct Zobrist {
    pub pieces: [[[u64; BitBoard::FIELDS.len()]; Piece::COUNT]; Color::COUNT],
    pub castling: [[u64; 2]; Color::COUNT],
    pub en_passant: [u64; BitBoard::FIELDS.len()],
    pub side: u64,
}

lazy_static! {
    pub static ref Z: Zobrist = Zobrist::new();
}

impl Zobrist {
    fn new() -> Zobrist {
        let mut rnd = oorandom::Rand64::new(13);

        let mut pieces = [[[0u64; BitBoard::FIELDS.len()]; Piece::COUNT]; Color::COUNT];
        let mut en_passant = [0u64; BitBoard::FIELDS.len()];
        let mut castling = [[0u64; 2]; Color::COUNT];
        let side = rnd.rand_u64();

        (0..pieces.len()).for_each(|i1| {
            (0..pieces[i1].len()).for_each(|i2| {
                (0..pieces[i1][i2].len()).for_each(|i3| {
                    pieces[i1][i2][i3] = rnd.rand_u64();
                });
            });
        });

        (0..en_passant.len()).for_each(|i| {
            en_passant[i] = rnd.rand_u64();
        });

        (0..castling.len()).for_each(|i1| {
            (0..castling[i1].len()).for_each(|i2| {
                castling[i1][i2] = rnd.rand_u64();
            });
        });

        Zobrist {
            pieces,
            castling,
            en_passant,
            side,
        }
    }

    /// Creates Zobrist hash of a chess board.
    pub fn hash(&self, b: &ChessBoard) -> u64 {
        let mut hash = 0u64;

        if b.next_move != Color::White {
            hash ^= self.side;
        }

        let castling0 = b.castling_options[0];
        if castling0.king_side {
            hash ^= self.castling[0][0];
        }
        if castling0.queen_side {
            hash ^= self.castling[0][1];
        }

        let castling1 = b.castling_options[1];
        if castling1.king_side {
            hash ^= self.castling[1][0];
        }
        if castling1.queen_side {
            hash ^= self.castling[1][1];
        }

        if let Some(en_passant_target) = b.en_passant_target {
            hash ^= self.en_passant[en_passant_target];
        }

        (0..Color::COUNT).for_each(|c| {
            (0..Piece::COUNT).for_each(|p| {
                let mut pieces = b.pieces[c][p];
                while let (Some(i), tmp) = pieces.bitpop() {
                    pieces = tmp;
                    hash ^= self.pieces[c][p][i];
                }
            });
        });

        hash
    }
}
