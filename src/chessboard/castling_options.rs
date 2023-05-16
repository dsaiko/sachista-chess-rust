/// Chess piece castling options.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CastlingOptions {
    pub king_side: bool,
    pub queen_side: bool,
}
