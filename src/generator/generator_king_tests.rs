use crate::bitboard::BitBoard;
use crate::chessboard::{ChessBoard, Color};
use crate::generator;
use crate::generator::generator_king::GeneratorKing;

#[test]
fn attacks() {
    let g = GeneratorKing::new();

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
 - - - - - - K -
 - - - - - - - -
",
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - x x x
 - - - - - x - x
 - - - - - x x x
",
        ),
        Test(
            Color::White,
            "
 - - - - - - - -
 K - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
",
            "
 x x - - - - - -
 - x - - - - - -           
 x x - - - - - -
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
 k - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
",
            "
 x x - - - - - -
 - x - - - - - -           
 x x - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
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
 - - - - - - K -
 - - - - - - - -
",
            8,
        ),
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 R - - - K - - R
",
            7,
        ),
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - n - - - - -
 - - - - - - p -
 R - - - K - - R
",
            5,
        ),
    ];

    let g = &generator::G;

    for test in tests {
        let b = ChessBoard::from_string(test.0).unwrap();

        let mut c = 0u8;

        g.generator_king.generate_moves(g, &b, &mut |_m| {
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
    let g = &generator::G;

    struct Test(&'static str, u8);
    let tests = vec![
        Test(
            "
 - - - - - - - -
 - - - - - - - -
 - - - - k - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
",
            8,
        ),
        Test(
            "
 r - - - k - - r
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
 - - - - - - - -
",
            7,
        ),
        Test(
            "
r - - - k - - r
- K - - - - - -
- - - - - - - -
- - - - - - - -
- - - - - - - -
- - - - - - - -
- - - - - - - -
- - - - - - - -
",
            6,
        ),
    ];

    for test in tests {
        let mut b = ChessBoard::from_string(test.0).unwrap();
        b.next_move = Color::Black;

        let mut c = 0u8;

        g.generator_king.generate_moves(g, &b, &mut |_m| {
            c += 1;
        });

        if c != test.1 {
            println!("--------\n{}\nExpected: {}\n Got:", b, test.1);
            g.generate_moves(&b, &mut |m| println!("{}", m));
            assert_eq!(c, test.1)
        }
    }
}
