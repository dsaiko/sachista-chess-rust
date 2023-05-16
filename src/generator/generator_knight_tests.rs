use crate::bitboard::BitBoard;
use crate::chessboard::{ChessBoard, Color};
use crate::generator::generator_knight::GeneratorKnight;

#[test]
fn attacks() {
    let g = GeneratorKnight::new();

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
 - - - - - - N -
 - - - - - - - -
",
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - x - x
 - - - - x - - -
 - - - - - - - -
 - - - - x - - -
",
        ),
        Test(
            Color::White,
            "
 - - - - - - - -
 N - - - - - - N
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
",
            "
 - - x - - x - -
 - - - - - - - -
 - - x - - x - -
 - x - - - - x -
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
 n - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - n - - - - - -
 - - - - - - - n
",
            "
 - - x - - - - -
 - - - - - - - -
 - - x - - - - -
 - x - - - - - -
 x - x - - - - -
 - - - x - - x -
 - - - - - x - -
 - - - x - - - -
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
fn moves_white() {
    let g = GeneratorKnight::new();

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
 - - - - - - N -
 - - - - - - - -
",
            4,
        ),
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - N - - - -
 - - - - - - - -
 - - - - N - - -
 - - - - - - - -
 - - - - - - - -
",
            14,
        ),
        Test(
            "
 N - - - - - - -
 - - p - - - - -
 - p - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - P - P
 - - - - P - - -
 - - - - - - N -
",
            2,
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
    let g = GeneratorKnight::new();

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
 - - - - - - n -
 - - - - - - - -
",
            4,
        ),
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - n - - - -
 - - - - - - - -
 - - - - n - - -
 - - - - - - - -
 - - - - - - - -
",
            14,
        ),
        Test(
            "
 n - - - - - - -
 - - P - - - - -
 - P - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - p - p
 - - - - p - - -
 - - - - - - n -
",
            2,
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
