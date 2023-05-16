use crate::bitboard::{BitBoard, Index};
use crate::chessboard::{ChessBoard, Color, Move, Piece};
use crate::generator::Generator;

const WHITE_CASTLING_OO_EMPTY: BitBoard = BitBoard(BitBoard::F1.0 | BitBoard::G1.0);
const WHITE_CASTLING_OO_ATTACKS: BitBoard =
    BitBoard(BitBoard::E1.0 | BitBoard::F1.0 | BitBoard::G1.0);
const WHITE_CASTLING_OOO_EMPTY: BitBoard =
    BitBoard(BitBoard::B1.0 | BitBoard::C1.0 | BitBoard::D1.0);
const WHITE_CASTLING_OOO_ATTACKS: BitBoard =
    BitBoard(BitBoard::C1.0 | BitBoard::D1.0 | BitBoard::E1.0);
const BLACK_CASTLING_OO_EMPTY: BitBoard = BitBoard(BitBoard::F8.0 | BitBoard::G8.0);
const BLACK_CASTLING_OO_ATTACKS: BitBoard =
    BitBoard(BitBoard::E8.0 | BitBoard::F8.0 | BitBoard::G8.0);
const BLACK_CASTLING_OOO_EMPTY: BitBoard =
    BitBoard(BitBoard::B8.0 | BitBoard::C8.0 | BitBoard::D8.0);
const BLACK_CASTLING_OOO_ATTACKS: BitBoard =
    BitBoard(BitBoard::C8.0 | BitBoard::D8.0 | BitBoard::E8.0);

pub struct GeneratorKing {
    attacks_cache: [BitBoard; Index::FIELDS.len()],
}

impl GeneratorKing {
    pub fn new() -> Self {
        let mut cache = [BitBoard::EMPTY; Index::FIELDS.len()];

        for i in Index::FIELDS {
            let b = i.as_bitboard();

            cache[i] = b.shifted(1, -1)
                | b.shifted(1, 0)
                | b.shifted(1, 1)
                | b.shifted(0, -1)
                | b.shifted(0, 1)
                | b.shifted(-1, -1)
                | b.shifted(-1, 0)
                | b.shifted(-1, 1)
        }
        GeneratorKing {
            attacks_cache: cache,
        }
    }

    pub fn generate_attacks(&self, board: &ChessBoard, color: Color) -> BitBoard {
        let b = board.pieces[color][Piece::King];

        if let Some(i) = b.bitscan() {
            self.attacks_cache[i]
        } else {
            BitBoard::EMPTY
        }
    }

    pub fn generate_moves(&self, g: &Generator, board: &ChessBoard, m: &mut impl FnMut(Move)) {
        let Some(from) = board.pieces[board.next_move][Piece::King].bitscan() else {
            return
        };

        let mut moves = self.attacks_cache[from] & board.board_to_attack();

        while let (Some(to), tmp) = moves.bitpop() {
            moves = tmp;
            m(Move {
                piece: Piece::King,
                from,
                to,
                specifier: None,
            })
        }

        let castling_options = board.castling_options[board.next_move];
        if !castling_options.king_side && !castling_options.queen_side {
            return;
        }

        let all_pieces = board.all_pieces();

        match board.next_move {
            Color::White => {
                if castling_options.king_side
                    && (all_pieces.0 & WHITE_CASTLING_OO_EMPTY.0) == 0
                    && !g.is_bitmask_under_attack(board, Color::Black, WHITE_CASTLING_OO_ATTACKS)
                {
                    m(Move {
                        piece: Piece::King,
                        from,
                        to: Index::G1,
                        specifier: None,
                    })
                }
                if castling_options.queen_side
                    && (all_pieces.0 & WHITE_CASTLING_OOO_EMPTY.0) == 0
                    && !g.is_bitmask_under_attack(board, Color::Black, WHITE_CASTLING_OOO_ATTACKS)
                {
                    m(Move {
                        piece: Piece::King,
                        from,
                        to: Index::C1,
                        specifier: None,
                    })
                }
            }
            Color::Black => {
                if castling_options.king_side
                    && (all_pieces.0 & BLACK_CASTLING_OO_EMPTY.0) == 0
                    && !g.is_bitmask_under_attack(board, Color::White, BLACK_CASTLING_OO_ATTACKS)
                {
                    m(Move {
                        piece: Piece::King,
                        from,
                        to: Index::G8,
                        specifier: None,
                    })
                }
                if castling_options.queen_side
                    && (all_pieces.0 & BLACK_CASTLING_OOO_EMPTY.0) == 0
                    && !g.is_bitmask_under_attack(board, Color::White, BLACK_CASTLING_OOO_ATTACKS)
                {
                    m(Move {
                        piece: Piece::King,
                        from,
                        to: Index::C8,
                        specifier: None,
                    })
                }
            }
        }
    }
}
