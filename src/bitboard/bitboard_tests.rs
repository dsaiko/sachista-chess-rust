use crate::bitboard::{BitBoard, Index};

#[test]
fn bit_pop() {
    let mut b = BitBoard::UNIVERSE;
    let mut index: Option<Index> = None;

    assert!(index.is_none());
    assert_eq!(b.popcnt(), 64);

    for i in 0..64 {
        assert_eq!(b.popcnt(), 64 - i);
        (index, b) = b.bitpop();
        assert!(index.is_some())
    }
    assert_eq!(b, BitBoard::EMPTY);

    (index, b) = b.bitpop();
    assert!(index.is_none());
    assert_eq!(b, BitBoard::EMPTY);
}

#[test]
fn indices() {
    let corner = "  
  a b c d e f g h
8 x x x - - - - - 8
7 x x - - - - - - 7
6 x - - - - - - - 6
5 - - - - - - - - 5
4 - - - - - - - - 4
3 - - - - - - - - 3
2 - - - - - - - - 2
1 - - - - - - - x 1
  a b c d e f g h
";

    let b = BitBoard::from_string(corner).unwrap();

    let indices = b.indices();
    assert_eq!(indices.len(), 7);

    let b2 = BitBoard::from_indices(indices);

    assert_eq!(b, b2);
}

#[test]
fn to_string() {
    let frame = "  a b c d e f g h
8 x x x x x x x x 8
7 x - - - - - - x 7
6 x - - - - - - x 6
5 x - - - - - - x 5
4 x - - - - - - x 4
3 x - - - - - - x 3
2 x - - - - - - x 2
1 x x x x x x x x 1
  a b c d e f g h
";

    #[rustfmt::skip]
    let b = BitBoard::from_indices(vec![
        Index::A1, Index::A2, Index::A3, Index::A4, Index::A5, Index::A6, Index::A7, Index::A8,
        Index::H1, Index::H2, Index::H3, Index::H4, Index::H5, Index::H6, Index::H7, Index::H8,
        Index::B1, Index::C1, Index::D1, Index::E1, Index::F1, Index::G1,
        Index::B8, Index::C8, Index::D8, Index::E8, Index::F8, Index::G8,
    ]);

    assert_eq!(b.to_string(), frame);

    let b2 = BitBoard::from_string(b.to_string().as_str());

    assert_eq!(b.to_string(), b2.unwrap().to_string());

    let corner = "  a b c d e f g h
8 x x x - - - - - 8
7 x x - - - - - - 7
6 x - - - - - - - 6
5 - - - - - - - - 5
4 - - - - - - - - 4
3 - - - - - - - - 3
2 - - - - - - - - 2
1 - - - - - - - x 1
  a b c d e f g h
";

    let b3 = BitBoard::from_string(corner);
    assert_eq!(corner, b3.unwrap().to_string());
}

#[test]
fn popcnt() {
    let mut sum = 0u64;
    for i in 1..1_000_000u64 {
        let b = BitBoard(i);
        sum += b.popcnt() as u64;
    }
    assert_eq!(sum, 9884992)
}

#[test]
fn bitscan() {
    for (i, b) in BitBoard::FIELDS.iter().enumerate() {
        assert_eq!(b.bitscan().unwrap().0, i)
    }

    assert!(BitBoard::EMPTY.bitscan().is_none());
}

#[test]
fn shift_north() {
    struct Test(fn(BitBoard) -> BitBoard, &'static str);
    let tests = vec![
        Test(
            |x: BitBoard| x.shifted_north(),
            "
 x - - - - - - x
 x - - - - - - x
 x - - - - - - x
 x - - - - - - x
 x - - - - - - x
 x - - - - - - x
 x x x x x x x x
 - - - - - - - -
",
        ),
        Test(
            |x: BitBoard| x.shifted_south(),
            "
 - - - - - - - -
 x x x x x x x x 
 x - - - - - - x 
 x - - - - - - x 
 x - - - - - - x 
 x - - - - - - x 
 x - - - - - - x 
 x - - - - - - x 
",
        ),
        Test(
            |x: BitBoard| x.shifted_east(),
            "
 - x x x x x x x  
 - x - - - - - -  
 - x - - - - - -  
 - x - - - - - -  
 - x - - - - - -  
 - x - - - - - -  
 - x - - - - - -  
 - x x x x x x x  
 ",
        ),
        Test(
            |x: BitBoard| x.shifted_west(),
            "
  x x x x x x x - 
  - - - - - - x -
  - - - - - - x -
  - - - - - - x -
  - - - - - - x -
  - - - - - - x -
  - - - - - - x -
  x x x x x x x -
 ",
        ),
        Test(
            |x: BitBoard| x.shifted_northeast(),
            "
 - x - - - - - - 
 - x - - - - - - 
 - x - - - - - - 
 - x - - - - - - 
 - x - - - - - - 
 - x - - - - - - 
 - x x x x x x x 
 - - - - - - - - 
 ",
        ),
        Test(
            |x: BitBoard| x.shifted_northwest(),
            "
  - - - - - - x -
  - - - - - - x -
  - - - - - - x -
  - - - - - - x -
  - - - - - - x -
  - - - - - - x -
  x x x x x x x -
  - - - - - - - -
 ",
        ),
        Test(
            |x: BitBoard| x.shifted_southeast(),
            "
 - - - - - - - - 
 - x x x x x x x  
 - x - - - - - -  
 - x - - - - - -  
 - x - - - - - -  
 - x - - - - - -  
 - x - - - - - -  
 - x - - - - - -  
 ",
        ),
        Test(
            |x: BitBoard| x.shifted_southwest(),
            "
 - - - - - - - -
 x x x x x x x - 
 - - - - - - x - 
 - - - - - - x -
 - - - - - - x -
 - - - - - - x -
 - - - - - - x -
 - - - - - - x -
 ",
        ),
        Test(
            |x: BitBoard| x.shifted(2, 2),
            "
 - - x - - - - -   
 - - x - - - - -   
 - - x - - - - -   
 - - x - - - - -   
 - - x - - - - -   
 - - x x x x x x   
 - - - - - - - -
 - - - - - - - -
 ",
        ),
        Test(
            |x: BitBoard| x.shifted(-2, -2),
            "
 - - - - - - - -
 - - - - - - - -
 x x x x x x - -
 - - - - - x - -
 - - - - - x - -
 - - - - - x - -
 - - - - - x - -
 - - - - - x - -
",
        ),
    ];

    for test in tests {
        let b = test.0(BitBoard::FRAME);
        assert_eq!(b, BitBoard::from_string(test.1).unwrap());
    }
}

#[test]
fn shift_mirror() {
    struct Test(fn(BitBoard) -> BitBoard, &'static str, &'static str);
    let tests = vec![
        Test(
            |x: BitBoard| x.mirrored_horizontally(),
            "
 x x x - - - - - 
 x x - - - - - - 
 x - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - x 
",
            "
 - - - - - x x x 
 - - - - - - x x 
 - - - - - - - x 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 x - - - - - - - 
",
        ),
        Test(
            |x: BitBoard| x.mirrored_vertically(),
            "
 x x x - - - - - 
 x x - - - - - - 
 x - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - x 
",
            "
 - - - - - - - x 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 x - - - - - - - 
 x x - - - - - - 
 x x x - - - - - 
",
        ),
        Test(
            |x: BitBoard| x.mirrored_a1h8(),
            "
 x x x - - - - - 
 x x - - - - - - 
 x - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - x x 
",
            "
 x - - - - - - - 
 x - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - x 
 - - - - - - x x 
 - - - - - x x x 
",
        ),
        Test(
            |x: BitBoard| x.mirrored_a8h1(),
            "
 x x x - - - - - 
 x x - - - - - - 
 x - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - x x 
",
            "
 x x x - - - - - 
 x x - - - - - - 
 x - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - - 
 - - - - - - - x 
 - - - - - - - x 
",
        ),
    ];

    for test in tests {
        let mut b = BitBoard::from_string(test.1).unwrap();
        let b2 = BitBoard::from_string(test.2).unwrap();
        b = test.0(b);
        assert_eq!(b, b2);
    }
}
