use strum::EnumCount;

use crate::bitboard::{BitBoard, Index};
use crate::chessboard::Specifier::{EnPassant, Promotion};
use crate::chessboard::{ChessBoard, Color, Move, Piece};

pub struct GeneratorPawn {
    attacks_cache: [[BitBoard; Index::FIELDS.len()]; Color::COUNT],
}

impl GeneratorPawn {
    pub fn new() -> Self {
        let mut cache = [[BitBoard::EMPTY; Index::FIELDS.len()]; Color::COUNT];

        for i in Index::FIELDS {
            let b = i.as_bitboard();

            cache[Color::White][i] = b.shifted_northeast() | b.shifted_northwest();
            cache[Color::Black][i] = b.shifted_southeast() | b.shifted_southwest();
        }

        GeneratorPawn {
            attacks_cache: cache,
        }
    }

    pub fn generate_attacks(&self, board: &ChessBoard, color: Color) -> BitBoard {
        let b = board.pieces[color][Piece::Pawn];

        match color {
            Color::White => b.shifted_northeast() | b.shifted_northwest(),
            Color::Black => b.shifted_southeast() | b.shifted_southwest(),
        }
    }

    pub fn generate_moves(&self, board: &ChessBoard, m: &mut impl FnMut(Move)) {
        let empty_board = !board.all_pieces();
        let mut pieces = board.pieces[board.next_move][Piece::Pawn];

        while let (Some(from), tmp) = pieces.bitpop() {
            pieces = tmp;
            let b = from.as_bitboard();
            let mut attacks = BitBoard::EMPTY;
            let mut moves = BitBoard::EMPTY;

            match board.next_move {
                Color::White => {
                    moves = b.shifted_north() & empty_board;
                    // double move
                    if from < Index::A3 {
                        moves |= moves.shifted_north() & empty_board;
                    }
                    // attacks
                    attacks = b.shifted_northeast() | b.shifted_northwest();
                    moves |= attacks & board.opponent_pieces();
                }
                Color::Black => {
                    moves = b.shifted_south() & empty_board;
                    // double move
                    if from > Index::H6 {
                        moves |= moves.shifted_south() & empty_board;
                    }
                    // attacks
                    attacks = b.shifted_southeast() | b.shifted_southwest();
                    moves |= attacks & board.opponent_pieces();
                }
            };

            while let (Some(to), tmp) = moves.bitpop() {
                moves = tmp;
                if to > Index::H7 || to < Index::A2 {
                    // promotion
                    m(Move {
                        piece: Piece::Pawn,
                        from,
                        to,
                        specifier: Some(Promotion(Piece::Bishop)),
                    });
                    m(Move {
                        piece: Piece::Pawn,
                        from,
                        to,
                        specifier: Some(Promotion(Piece::Knight)),
                    });
                    m(Move {
                        piece: Piece::Pawn,
                        from,
                        to,
                        specifier: Some(Promotion(Piece::Queen)),
                    });
                    m(Move {
                        piece: Piece::Pawn,
                        from,
                        to,
                        specifier: Some(Promotion(Piece::Rook)),
                    });
                } else {
                    m(Move {
                        piece: Piece::Pawn,
                        from,
                        to,
                        specifier: None,
                    });
                }
            }

            if let Some(en_passant_target) = board.en_passant_target {
                moves = attacks & en_passant_target.as_bitboard();
                if let Some(to) = moves.bitscan() {
                    m(Move {
                        piece: Piece::Pawn,
                        from,
                        to,
                        specifier: Some(EnPassant),
                    });
                }
            }
        }
    }
}
