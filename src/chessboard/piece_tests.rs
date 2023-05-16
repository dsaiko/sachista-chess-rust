use strum::{EnumCount, IntoEnumIterator};

use crate::chessboard::Piece;

#[test]
fn to_string() {
    for p in Piece::iter() {
        let s = p.to_string();
        assert_eq!(
            p,
            Piece::from_char(s.as_str().as_bytes()[0] as char)
                .unwrap()
                .1
        );
    }

    assert!(Piece::from_char(' ').is_none());
}

#[test]
fn range() {
    let v: [u64; Piece::COUNT] = [0, 1, 2, 3, 4, 5];

    for (i, p) in Piece::iter().enumerate() {
        assert_eq!(v[p as usize], i as u64);
    }
}
