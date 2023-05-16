use strum::{EnumCount, IntoEnumIterator};

use crate::chessboard::Color;

#[test]
fn to_string() {
    let w = Color::White.to_string();
    assert_eq!(Color::from_string(w.as_str()).unwrap(), Color::White);

    let b = Color::Black.to_string();
    assert_eq!(Color::from_string(b.as_str()).unwrap(), Color::Black);

    let x = Color::from_string("?");
    assert!(x.is_none());

    let y = Color::from_string("");
    assert!(y.is_none());
}

#[test]
fn to_opponent() {
    let w = Color::White;
    let b = Color::Black;

    assert_eq!(w.opponent(), b);
    assert_eq!(b.opponent(), w);

    assert_eq!(w.opponent().opponent(), w);
    assert_eq!(b.opponent().opponent(), b);
}

#[test]
fn range() {
    let v: [u64; Color::COUNT] = [0, 1];

    for (i, c) in Color::iter().enumerate() {
        assert_eq!(v[c as usize], i as u64);
    }
}
