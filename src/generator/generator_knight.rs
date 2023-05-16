use crate::bitboard::{BitBoard, Index};

use crate::chessboard::{ChessBoard, Color, Move, Piece};

pub struct GeneratorKnight {
    attacks_cache: [BitBoard; Index::FIELDS.len()],
}

impl GeneratorKnight {
    pub fn new() -> Self {
        let mut cache = [BitBoard::EMPTY; Index::FIELDS.len()];

        for i in Index::FIELDS {
            let b = i.as_bitboard();

            cache[i] = b.shifted(2, 1)
                | b.shifted(2, -1)
                | b.shifted(1, 2)
                | b.shifted(-1, 2)
                | b.shifted(-2, 1)
                | b.shifted(-2, -1)
                | b.shifted(-1, -2)
                | b.shifted(1, -2)
        }
        GeneratorKnight {
            attacks_cache: cache,
        }
    }

    pub fn generate_attacks(&self, board: &ChessBoard, color: Color) -> BitBoard {
        let mut b = board.pieces[color][Piece::Knight];

        let mut attacks = BitBoard::EMPTY;
        while let (Some(from), tmp) = b.bitpop() {
            b = tmp;
            attacks |= self.attacks_cache[from];
        }

        attacks
    }

    pub fn generate_moves(&self, board: &ChessBoard, m: &mut impl FnMut(Move)) {
        let mut pieces = board.pieces[board.next_move][Piece::Knight];

        while let (Some(from), tmp) = pieces.bitpop() {
            pieces = tmp;
            let mut moves = self.attacks_cache[from] & board.board_to_attack();

            while let (Some(to), tmp) = moves.bitpop() {
                moves = tmp;
                m(Move {
                    piece: Piece::Knight,
                    from,
                    to,
                    specifier: None,
                })
            }
        }
    }
}
