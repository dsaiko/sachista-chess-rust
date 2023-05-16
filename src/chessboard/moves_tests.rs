use crate::bitboard::Index;
use crate::chessboard::{moves::Specifier, ChessBoard, Color, Move, Piece};

#[test]
fn move_to_string() {
    let m = Move {
        piece: Piece::Pawn,
        from: Index::A2,
        to: Index::A3,
        specifier: None,
    };
    assert_eq!("a2a3", m.to_string());

    let m = Move {
        piece: Piece::Pawn,
        from: Index::A7,
        to: Index::B8,
        specifier: Some(Specifier::Promotion(Piece::Queen)),
    };
    assert_eq!("a7b8q", m.to_string());
}

#[test]
fn apply_to() {
    struct Test(Color, Move, &'static str, &'static str);
    let tests = vec![
        Test(
            Color::White,
            Move {
                piece: Piece::Pawn,
                from: Index::C7,
                to: Index::D8,
                specifier: Some(Specifier::Promotion(Piece::Queen)),
            },
            "
         - - - r - - - - 
         - - P - - - - - 
         - - - - - - - - 
         - - - - - - - - 
         - - - - - - - - 
         - - - - - - - - 
         - - - - - - - - 
         - - - - - - - - 
        ",
            "
        - - - Q - - - - 
        - - - - - - - - 
        - - - - - - - - 
        - - - - - - - - 
        - - - - - - - - 
        - - - - - - - - 
        - - - - - - - - 
        - - - - - - - -        
        ",
        ),
        Test(
            Color::White,
            Move {
                piece: Piece::Pawn,
                from: Index::H5,
                to: Index::G6,
                specifier: Some(Specifier::EnPassant),
            },
            "
         - - - r - - - -
         - - P - - - - -
         - - - - - - - -
         - - - - - - p P
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
        ",
            "
        - - - r - - - -
        - - P - - - - -
        - - - - - - P -
        - - - - - - - -
        - - - - - - - -
        - - - - - - - -
        - - - - - - - -
        - - - - - - - -
        ",
        ),
        Test(
            Color::White,
            Move {
                piece: Piece::King,
                from: Index::E1,
                to: Index::G1,
                specifier: None,
            },
            "
         r - - - k - - r
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         R - - - K - - R
        ",
            "
         r - - - k - - r
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         R - - - - R K -
        ",
        ),
        Test(
            Color::White,
            Move {
                piece: Piece::King,
                from: Index::E1,
                to: Index::C1,
                specifier: None,
            },
            "
         r - - - k - - r
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         R - - - K - - R
        ",
            "
         r - - - k - - r
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - K R - - - R
        ",
        ),
        Test(
            Color::Black,
            Move {
                piece: Piece::King,
                from: Index::E8,
                to: Index::G8,
                specifier: None,
            },
            "
         r - - - k - - r
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         R - - - K - - R
        ",
            "
         r - - - - r k -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         R - - - K - - R
        ",
        ),
        Test(
            Color::Black,
            Move {
                piece: Piece::King,
                from: Index::E8,
                to: Index::C8,
                specifier: None,
            },
            "
         r - - - k - - r
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         R - - - K - - R
        ",
            "
         - - k r - - - r
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         - - - - - - - -
         R - - - K - - R
        ",
        ),
    ];

    for test in tests {
        let mut board = ChessBoard::from_string(test.2).unwrap();
        board.next_move = test.0;
        board.update_hash();

        let board2 = ChessBoard::from_string(test.3).unwrap();

        let mut b = test.1.apply_to(&board);
        assert_eq!(b.next_move, board.next_move.opponent());

        if b.to_string() != board2.to_string() {
            print!("expected: {}", board2);
            print!("result: {}", b);
            assert_eq!(b.to_string(), board2.to_string());
        }

        let hash = b.hash;
        b.update_hash();
        let hash2 = b.hash;

        b.next_move = b.next_move.opponent();
        b.update_hash();
        let hash3 = b.hash;

        assert_eq!(hash, hash2);
        assert_ne!(hash, hash3);
    }
}
