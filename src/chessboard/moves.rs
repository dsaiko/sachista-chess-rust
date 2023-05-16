use std::fmt::Display;

use crate::bitboard::{BitBoard, Index};
use crate::chessboard::{zobrist, ChessBoard, Color, Piece};

#[derive(Debug)]
pub enum Specifier {
    EnPassant,
    Promotion(Piece),
}

#[derive(Debug)]
pub struct Move {
    pub piece: Piece,
    pub from: Index,
    pub to: Index,

    pub specifier: Option<Specifier>,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from, self.to)?;
        if let Some(Specifier::Promotion(promotion)) = self.specifier {
            write!(f, "{}", promotion)?;
        }

        Ok(())
    }
}

impl Move {
    pub fn apply_to(&self, board: &ChessBoard) -> ChessBoard {
        let mut b = *board;

        let source_index = self.from;
        let target_index = self.to;
        let source_bitboard = self.from.as_bitboard();
        let target_bitboard = self.to.as_bitboard();
        let opponent_color = board.next_move.opponent();

        b.half_move_clock += 1;

        // reset enPassant
        if let Some(en_passant_target) = b.en_passant_target {
            b.en_passant_target = None;
            b.hash ^= zobrist::Z.en_passant[en_passant_target];
        }

        // reset castling
        if b.castling_options[Color::White].king_side {
            b.hash ^= zobrist::Z.castling[Color::White][0];
        }
        if b.castling_options[Color::White].queen_side {
            b.hash ^= zobrist::Z.castling[Color::White][1];
        }
        if b.castling_options[Color::Black].king_side {
            b.hash ^= zobrist::Z.castling[Color::Black][0];
        }
        if b.castling_options[Color::Black].queen_side {
            b.hash ^= zobrist::Z.castling[Color::Black][1];
        }

        // make the move
        b.pieces[b.next_move][self.piece] ^= source_bitboard | target_bitboard;
        b.hash ^= zobrist::Z.pieces[b.next_move][self.piece][source_index]
            ^ zobrist::Z.pieces[b.next_move][self.piece][target_index];

        match self.piece {
            Piece::Rook => match b.next_move {
                Color::White => match self.from {
                    Index::A1 => b.castling_options[b.next_move].queen_side = false,
                    Index::H1 => b.castling_options[b.next_move].king_side = false,
                    _ => {}
                },
                Color::Black => match self.from {
                    Index::A8 => b.castling_options[b.next_move].queen_side = false,
                    Index::H8 => b.castling_options[b.next_move].king_side = false,
                    _ => {}
                },
            },
            Piece::King => {
                b.castling_options[b.next_move].queen_side = false;
                b.castling_options[b.next_move].king_side = false;
                match b.next_move {
                    Color::White => {
                        if self.from == Index::E1 {
                            match self.to {
                                Index::C1 => {
                                    b.pieces[b.next_move][Piece::Rook] ^=
                                        BitBoard::A1 | BitBoard::D1;
                                    b.hash ^= zobrist::Z.pieces[b.next_move][Piece::Rook]
                                        [Index::A1]
                                        ^ zobrist::Z.pieces[b.next_move][Piece::Rook][Index::D1];
                                }
                                Index::G1 => {
                                    b.pieces[b.next_move][Piece::Rook] ^=
                                        BitBoard::H1 | BitBoard::F1;
                                    b.hash ^= zobrist::Z.pieces[b.next_move][Piece::Rook]
                                        [Index::H1]
                                        ^ zobrist::Z.pieces[b.next_move][Piece::Rook][Index::F1];
                                }
                                _ => {}
                            }
                        }
                    }
                    Color::Black => {
                        if self.from == Index::E8 {
                            match self.to {
                                Index::C8 => {
                                    b.pieces[b.next_move][Piece::Rook] ^=
                                        BitBoard::A8 | BitBoard::D8;
                                    b.hash ^= zobrist::Z.pieces[b.next_move][Piece::Rook]
                                        [Index::A8]
                                        ^ zobrist::Z.pieces[b.next_move][Piece::Rook][Index::D8];
                                }
                                Index::G8 => {
                                    b.pieces[b.next_move][Piece::Rook] ^=
                                        BitBoard::H8 | BitBoard::F8;
                                    b.hash ^= zobrist::Z.pieces[b.next_move][Piece::Rook]
                                        [Index::H8]
                                        ^ zobrist::Z.pieces[b.next_move][Piece::Rook][Index::F8];
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Piece::Pawn => {
                b.half_move_clock = 0;
                if target_index.distance_to(source_index) > 10 {
                    let i = if b.next_move == Color::White {
                        source_index + 8
                    } else {
                        source_index - 8
                    };
                    b.en_passant_target = Some(i);
                } else if let Some(Specifier::Promotion(promotion)) = self.specifier {
                    b.pieces[b.next_move][Piece::Pawn] ^= target_bitboard;
                    b.hash ^= zobrist::Z.pieces[b.next_move][Piece::Pawn][target_index];

                    b.pieces[b.next_move][promotion] ^= target_bitboard;
                    b.hash ^= zobrist::Z.pieces[b.next_move][promotion][target_index];
                }
            }
            _ => {}
        }

        let is_capture = (target_bitboard & b.opponent_pieces()) != BitBoard::EMPTY;
        let is_enpassant = matches!(self.specifier, Some(Specifier::EnPassant));

        if is_capture || is_enpassant {
            b.half_move_clock = 0;

            macro_rules! handle_capture {
                ($a:expr) => {
                    if (b.pieces[opponent_color][$a] & target_bitboard) != BitBoard::EMPTY {
                        b.pieces[opponent_color][$a] ^= target_bitboard;
                        b.hash ^= zobrist::Z.pieces[opponent_color][$a][target_index];
                        true
                    } else {
                        false
                    }
                };
            }

            if is_enpassant {
                match b.next_move {
                    Color::White => {
                        b.pieces[Color::Black][Piece::Pawn] ^= target_bitboard.shifted_south();
                        b.hash ^= zobrist::Z.pieces[Color::Black][Piece::Pawn][target_index - 8];
                    }
                    Color::Black => {
                        b.pieces[Color::White][Piece::Pawn] ^= target_bitboard.shifted_north();
                        b.hash ^= zobrist::Z.pieces[Color::White][Piece::Pawn][target_index + 8];
                    }
                }
            } else if handle_capture!(Piece::Bishop)
                || handle_capture!(Piece::Knight)
                || handle_capture!(Piece::Pawn)
                || handle_capture!(Piece::Queen)
                || handle_capture!(Piece::Rook)
            {
                match b.next_move {
                    Color::White => match target_index {
                        Index::A8 => b.castling_options[Color::Black].queen_side = false,
                        Index::H8 => b.castling_options[Color::Black].king_side = false,
                        _ => {}
                    },
                    Color::Black => match target_index {
                        Index::A1 => b.castling_options[Color::White].queen_side = false,
                        Index::H1 => b.castling_options[Color::White].king_side = false,
                        _ => {}
                    },
                }
            }
        }

        b.next_move = opponent_color;

        if b.next_move == Color::Black {
            b.full_move_number += 1;
        }

        // update zobrist
        b.hash ^= zobrist::Z.side;

        let castling0 = b.castling_options[0];
        if castling0.king_side {
            b.hash ^= zobrist::Z.castling[0][0];
        }
        if castling0.queen_side {
            b.hash ^= zobrist::Z.castling[0][1];
        }

        let castling1 = b.castling_options[1];
        if castling1.king_side {
            b.hash ^= zobrist::Z.castling[1][0];
        }
        if castling1.queen_side {
            b.hash ^= zobrist::Z.castling[1][1];
        }

        if let Some(en_passant_target) = b.en_passant_target {
            b.hash ^= zobrist::Z.en_passant[en_passant_target];
        }

        b
    }
}
