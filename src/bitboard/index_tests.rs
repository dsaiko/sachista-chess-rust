use crate::bitboard::Index;

#[test]
fn display() {
    let d: [&str; Index::FIELDS.len()] = [
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2",
        "h2", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4",
        "g4", "h4", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6",
        "f6", "g6", "h6", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8",
        "e8", "f8", "g8", "h8",
    ];
    for i in Index::FIELDS {
        let s = i.to_string();
        assert_eq!(s, d[i.0]);

        let i2 = Index::from_notation(s.as_str());
        assert!(i2.is_some());
        assert!(i2.is_some_and(|x| x.0 == i.0));
    }

    assert!(Index::from_notation("").is_none());
    assert!(Index::from_notation("XX").is_none());
}

#[test]
fn rank() {
    #[rustfmt::skip]
        let d: [usize; Index::FIELDS.len()] = [
            0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1,
            2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3,
            4, 4, 4, 4, 4, 4, 4, 4,
            5, 5, 5, 5, 5, 5, 5, 5,
            6, 6, 6, 6, 6, 6, 6, 6,
            7, 7, 7, 7, 7, 7, 7, 7,
        ];
    for i in Index::FIELDS {
        let f = i.rank();
        assert_eq!(f, d[i.0]);
    }
}

#[test]
fn file() {
    #[rustfmt::skip]
        let d: [usize; Index::FIELDS.len()] = [
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
            0, 1, 2, 3, 4, 5, 6, 7,
        ];
    for i in Index::FIELDS {
        let f = i.file();
        assert_eq!(f, d[i.0]);
    }
}

#[test]
fn as_bitboard() {
    for i in Index::FIELDS {
        let b = i.as_bitboard();
        let indices = b.indices();
        let i2 = indices.get(0);
        assert!(i2.is_some());
        assert_eq!(i2.unwrap().0, i.0);
    }
}
