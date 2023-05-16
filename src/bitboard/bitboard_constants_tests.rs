use crate::bitboard::BitBoard;

#[test]
fn popcount() {
    assert_eq!(BitBoard::EMPTY.popcnt(), 0);
    assert_eq!(BitBoard::UNIVERSE.popcnt(), 64);
    assert_eq!(BitBoard::FRAME.popcnt(), 28);
}

#[test]
fn ranks() {
    let mut f = BitBoard::EMPTY;

    for r in BitBoard::RANKS {
        assert_eq!(r.popcnt(), 8);
        f |= r;
    }
    assert_eq!(f, BitBoard::UNIVERSE)
}

#[test]
fn files() {
    let mut b = BitBoard::EMPTY;

    for f in BitBoard::FILES {
        assert_eq!(f.popcnt(), 8);
        b |= f;
    }
    assert_eq!(b, BitBoard::UNIVERSE)
}

#[test]
fn a1h8() {
    let mut b = BitBoard::EMPTY;

    for f in BitBoard::A1H8 {
        b |= f;
    }
    assert_eq!(b, BitBoard::UNIVERSE)
}

#[test]
fn a8h1() {
    let mut b = BitBoard::EMPTY;

    for f in BitBoard::A1H8 {
        b |= f;
    }
    assert_eq!(b, BitBoard::UNIVERSE)
}

#[test]
fn fields() {
    let mut b = BitBoard::EMPTY;

    for f in BitBoard::FIELDS {
        assert_eq!(f.popcnt(), 1);
        b |= f;
    }
    assert_eq!(b, BitBoard::UNIVERSE)
}
