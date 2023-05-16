use crate::bitboard::BitBoard;
use crate::chessboard::{ChessBoard, Color};
use crate::generator::generator_pawn::GeneratorPawn;

#[test]
fn attacks() {
    let g = GeneratorPawn::new();

    struct Test(Color, &'static str, &'static str);
    let tests = vec![
        Test(
            Color::White,
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
",
            "
 - - - - - - - -
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
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - P -
 - - - - - - - -
",
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - x - x
 - - - - - - - -
 - - - - - - - -
",
        ),
        Test(
            Color::White,
            "
 - - - - - - - -
 P - - - - - - P
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
",
            "
 - x - - - - x -
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
            Color::Black,
            "
 - - - - - - - -
 p - - - - - - p
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - p - - - - - -
 - - - - - - - p
",
            "
 - - - - - - - -
 - - - - - - - -
 - x - - - - x -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 x - x - - - - -
",
        ),
    ];

    for test in tests {
        let b = ChessBoard::from_string(test.1).unwrap();
        let b2 = BitBoard::from_string(test.2).unwrap();

        let attacks = g.generate_attacks(&b, test.0);
        if attacks != b2 {
            println!("{}", attacks);
            println!("{}", b2);
            assert_eq!(attacks, b2);
        }
    }
}

#[test]
fn enpassant() {
    let g = GeneratorPawn::new();

    struct Test(&'static str, u8);
    let tests = vec![Test(
        "111n1n111/11111111/11n11pP1/11111111/11111111/11n1n1n1/11111111/11111111 w KQkq f7 0 1",
        2,
    ),
        Test(
        "k7/8/8/8/5Pp1/6P1/8/K7 b - f3 0 1",
        1,
    )
    ];

    for test in tests {
        let b = ChessBoard::from_fen(test.0).unwrap();

        let mut c = 0u8;

        g.generate_moves(&b, &mut |_m| {
            c += 1;
        });

        if c != test.1 {
            println!("--------\n{}\nExpected: {}\n Got:", b, test.1);
            g.generate_moves(&b, &mut |m| println!("{}", m));
            assert_eq!(c, test.1)
        }
    }
}

#[test]
fn moves_white() {
    let g = GeneratorPawn::new();

    struct Test(&'static str, u8);
    let tests = vec![
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - P -
 - - - - - - - -
",
            2,
        ),
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - p -
 - - - - - - P -
 - - - - - - - -
",
            0,
        ),
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - p - p
 - - - - - - P -
 - - - - - - - -
",
            4,
        ),
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - p p -
 - - - - - - P P
 - - - - - - - -
",
            4,
        ),
        Test(
            "
 - - - - - - - -
 - - - - p - p -
 - - - - - P - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
",
            3,
        ),
        Test(
            "
 - - - - p - p -
 - - - - - P - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
",
            12,
        ),
        Test(
            "
 - - - - - - - -
 P - - - - - - -
 - N p - k - - -
 - P - - P - - -
 - - - - - - - -
 - - - - - P - -
 - P - - - - - -
 - - - - - - - -
",
            8,
        ),
    ];

    for test in tests {
        let b = ChessBoard::from_string(test.0).unwrap();

        let mut c = 0u8;

        g.generate_moves(&b, &mut |_m| {
            c += 1;
        });

        if c != test.1 {
            println!("--------\n{}\nExpected: {}\n Got:", b, test.1);
            g.generate_moves(&b, &mut |m| println!("{}", m));
            assert_eq!(c, test.1)
        }
    }
}

#[test]
fn moves_black() {
    let g = GeneratorPawn::new();

    struct Test(&'static str, u8);
    let tests = vec![
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - p
 - - - - - - P -
 - - - - - - - -
",
            2,
        ),
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - p -
 - - - - - - P -
 - - - - - - - -
",
            0,
        ),
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - p - p
 - - - - P - P -
 - - - - - - - -
",
            5,
        ),
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - p p -
 - - - - - - P P
 - - - - - - - -
",
            3,
        ),
        Test(
            "
 - - - - - - - -
 - - - - p - p -
 - - - - - P - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
",
            6,
        ),
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - p - -
 - - - - P - P -
",
            12,
        ),
        Test(
            "
 - - - - - - - -
 P - - - - - - p
 - N p - k - - -
 - P - - P - - -
 - - - - - - - -
 - - - - - P - -
 - P - - - - - -
 - - - - - - - -
",
            4,
        ),
    ];

    for test in tests {
        let mut b = ChessBoard::from_string(test.0).unwrap();
        b.next_move = Color::Black;

        let mut c = 0u8;

        g.generate_moves(&b, &mut |_m| {
            c += 1;
        });

        if c != test.1 {
            println!("--------\n{}\nExpected: {}\n Got:", b, test.1);
            g.generate_moves(&b, &mut |m| println!("{}", m));
            assert_eq!(c, test.1)
        }
    }
}
