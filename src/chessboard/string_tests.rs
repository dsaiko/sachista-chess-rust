use crate::chessboard::ChessBoard;

#[test]
fn display() {
    let b = ChessBoard::new_standard_board();

    assert_eq!(
        b.to_string(),
        "  a b c d e f g h
8 r n b q k b n r 8
7 p p p p p p p p 7
6 - - - - - - - - 6
5 - - - - - - - - 5
4 - - - - - - - - 4
3 - - - - - - - - 3
2 P P P P P P P P 2
1 R N B Q K B N R 1
  a b c d e f g h
"
    );
}

#[test]
fn from_string() {
    let standard = ChessBoard::new_standard_board();

    let b = ChessBoard::from_string(
        "
      a b c d e f g h
    8 r n b q k b n r 8
    7 p p p p p p p p 7
    6 - - - - - - - - 6
    5 - - - - - - - - 5
    4 - - - - - - - - 4
    3 - - - - - - - - 3
    2 P P P P P P P P 2
    1 R N B Q K B N R 1
      a b c d e f g h",
    )
    .unwrap();

    assert_eq!(b.hash, standard.hash);
    assert_eq!(b.to_fen(), standard.to_fen());

    assert_eq!(
        ChessBoard::from_string(&standard.to_string())
            .unwrap()
            .to_fen(),
        standard.to_fen()
    );
}

#[test]
fn from_string_test_no_decoration() {
    let standard = ChessBoard::new_standard_board();

    let b = ChessBoard::from_string(
        "
     r n b q k b n r 
     p p p p p p p p 
     - - - - - - - - 
     - - - - - - - - 
     - - - - - - - - 
     - - - - - - - - 
     P P P P P P P P 
     R N B Q K B N R ",
    )
    .unwrap();

    assert_eq!(b.hash, standard.hash);
    assert_eq!(b.to_fen(), standard.to_fen());

    assert_eq!(
        ChessBoard::from_string(&standard.to_string())
            .unwrap()
            .to_fen(),
        standard.to_fen()
    );
}
