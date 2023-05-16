use crate::bitboard::BitBoard;

impl BitBoard {
    pub const A1: BitBoard = BitBoard(1);
    pub const B1: BitBoard = BitBoard(1 << 1);
    pub const C1: BitBoard = BitBoard(1 << 2);
    pub const D1: BitBoard = BitBoard(1 << 3);
    pub const E1: BitBoard = BitBoard(1 << 4);
    pub const F1: BitBoard = BitBoard(1 << 5);
    pub const G1: BitBoard = BitBoard(1 << 6);
    pub const H1: BitBoard = BitBoard(1 << 7);

    pub const A2: BitBoard = BitBoard(1 << 8);
    pub const B2: BitBoard = BitBoard(1 << 9);
    pub const C2: BitBoard = BitBoard(1 << 10);
    pub const D2: BitBoard = BitBoard(1 << 11);
    pub const E2: BitBoard = BitBoard(1 << 12);
    pub const F2: BitBoard = BitBoard(1 << 13);
    pub const G2: BitBoard = BitBoard(1 << 14);
    pub const H2: BitBoard = BitBoard(1 << 15);

    pub const A3: BitBoard = BitBoard(1 << 16);
    pub const B3: BitBoard = BitBoard(1 << 17);
    pub const C3: BitBoard = BitBoard(1 << 18);
    pub const D3: BitBoard = BitBoard(1 << 19);
    pub const E3: BitBoard = BitBoard(1 << 20);
    pub const F3: BitBoard = BitBoard(1 << 21);
    pub const G3: BitBoard = BitBoard(1 << 22);
    pub const H3: BitBoard = BitBoard(1 << 23);

    pub const A4: BitBoard = BitBoard(1 << 24);
    pub const B4: BitBoard = BitBoard(1 << 25);
    pub const C4: BitBoard = BitBoard(1 << 26);
    pub const D4: BitBoard = BitBoard(1 << 27);
    pub const E4: BitBoard = BitBoard(1 << 28);
    pub const F4: BitBoard = BitBoard(1 << 29);
    pub const G4: BitBoard = BitBoard(1 << 30);
    pub const H4: BitBoard = BitBoard(1 << 31);

    pub const A5: BitBoard = BitBoard(1 << 32);
    pub const B5: BitBoard = BitBoard(1 << 33);
    pub const C5: BitBoard = BitBoard(1 << 34);
    pub const D5: BitBoard = BitBoard(1 << 35);
    pub const E5: BitBoard = BitBoard(1 << 36);
    pub const F5: BitBoard = BitBoard(1 << 37);
    pub const G5: BitBoard = BitBoard(1 << 38);
    pub const H5: BitBoard = BitBoard(1 << 39);

    pub const A6: BitBoard = BitBoard(1 << 40);
    pub const B6: BitBoard = BitBoard(1 << 41);
    pub const C6: BitBoard = BitBoard(1 << 42);
    pub const D6: BitBoard = BitBoard(1 << 43);
    pub const E6: BitBoard = BitBoard(1 << 44);
    pub const F6: BitBoard = BitBoard(1 << 45);
    pub const G6: BitBoard = BitBoard(1 << 46);
    pub const H6: BitBoard = BitBoard(1 << 47);

    pub const A7: BitBoard = BitBoard(1 << 48);
    pub const B7: BitBoard = BitBoard(1 << 49);
    pub const C7: BitBoard = BitBoard(1 << 50);
    pub const D7: BitBoard = BitBoard(1 << 51);
    pub const E7: BitBoard = BitBoard(1 << 52);
    pub const F7: BitBoard = BitBoard(1 << 53);
    pub const G7: BitBoard = BitBoard(1 << 54);
    pub const H7: BitBoard = BitBoard(1 << 55);

    pub const A8: BitBoard = BitBoard(1 << 56);
    pub const B8: BitBoard = BitBoard(1 << 57);
    pub const C8: BitBoard = BitBoard(1 << 58);
    pub const D8: BitBoard = BitBoard(1 << 59);
    pub const E8: BitBoard = BitBoard(1 << 60);
    pub const F8: BitBoard = BitBoard(1 << 61);
    pub const G8: BitBoard = BitBoard(1 << 62);
    pub const H8: BitBoard = BitBoard(1 << 63);

    pub const EMPTY: BitBoard = BitBoard(0);
    pub const UNIVERSE: BitBoard = BitBoard(!BitBoard::EMPTY.0);

    #[rustfmt::skip]
    pub const FIELDS: [BitBoard; 64] = [
        BitBoard::A1, BitBoard::B1, BitBoard::C1, BitBoard::D1, BitBoard::E1, BitBoard::F1, BitBoard::G1, BitBoard::H1,
        BitBoard::A2, BitBoard::B2, BitBoard::C2, BitBoard::D2, BitBoard::E2, BitBoard::F2, BitBoard::G2, BitBoard::H2,
        BitBoard::A3, BitBoard::B3, BitBoard::C3, BitBoard::D3, BitBoard::E3, BitBoard::F3, BitBoard::G3, BitBoard::H3,
        BitBoard::A4, BitBoard::B4, BitBoard::C4, BitBoard::D4, BitBoard::E4, BitBoard::F4, BitBoard::G4, BitBoard::H4,
        BitBoard::A5, BitBoard::B5, BitBoard::C5, BitBoard::D5, BitBoard::E5, BitBoard::F5, BitBoard::G5, BitBoard::H5,
        BitBoard::A6, BitBoard::B6, BitBoard::C6, BitBoard::D6, BitBoard::E6, BitBoard::F6, BitBoard::G6, BitBoard::H6,
        BitBoard::A7, BitBoard::B7, BitBoard::C7, BitBoard::D7, BitBoard::E7, BitBoard::F7, BitBoard::G7, BitBoard::H7,
        BitBoard::A8, BitBoard::B8, BitBoard::C8, BitBoard::D8, BitBoard::E8, BitBoard::F8, BitBoard::G8, BitBoard::H8,
    ];

    #[rustfmt::skip]
    pub const RANKS: [BitBoard; 8] = [
        BitBoard(BitBoard::A1.0 | BitBoard::B1.0 | BitBoard::C1.0 | BitBoard::D1.0 | BitBoard::E1.0 | BitBoard::F1.0 | BitBoard::G1.0 | BitBoard::H1.0),
        BitBoard(BitBoard::A2.0 | BitBoard::B2.0 | BitBoard::C2.0 | BitBoard::D2.0 | BitBoard::E2.0 | BitBoard::F2.0 | BitBoard::G2.0 | BitBoard::H2.0),
        BitBoard(BitBoard::A3.0 | BitBoard::B3.0 | BitBoard::C3.0 | BitBoard::D3.0 | BitBoard::E3.0 | BitBoard::F3.0 | BitBoard::G3.0 | BitBoard::H3.0),
        BitBoard(BitBoard::A4.0 | BitBoard::B4.0 | BitBoard::C4.0 | BitBoard::D4.0 | BitBoard::E4.0 | BitBoard::F4.0 | BitBoard::G4.0 | BitBoard::H4.0),
        BitBoard(BitBoard::A5.0 | BitBoard::B5.0 | BitBoard::C5.0 | BitBoard::D5.0 | BitBoard::E5.0 | BitBoard::F5.0 | BitBoard::G5.0 | BitBoard::H5.0),
        BitBoard(BitBoard::A6.0 | BitBoard::B6.0 | BitBoard::C6.0 | BitBoard::D6.0 | BitBoard::E6.0 | BitBoard::F6.0 | BitBoard::G6.0 | BitBoard::H6.0),
        BitBoard(BitBoard::A7.0 | BitBoard::B7.0 | BitBoard::C7.0 | BitBoard::D7.0 | BitBoard::E7.0 | BitBoard::F7.0 | BitBoard::G7.0 | BitBoard::H7.0),
        BitBoard(BitBoard::A8.0 | BitBoard::B8.0 | BitBoard::C8.0 | BitBoard::D8.0 | BitBoard::E8.0 | BitBoard::F8.0 | BitBoard::G8.0 | BitBoard::H8.0),
    ];

    #[rustfmt::skip]
    pub const FILES: [BitBoard; 8] = [
        BitBoard(BitBoard::A1.0 | BitBoard::A2.0 | BitBoard::A3.0 | BitBoard::A4.0 | BitBoard::A5.0 | BitBoard::A6.0 | BitBoard::A7.0 | BitBoard::A8.0),
        BitBoard(BitBoard::B1.0 | BitBoard::B2.0 | BitBoard::B3.0 | BitBoard::B4.0 | BitBoard::B5.0 | BitBoard::B6.0 | BitBoard::B7.0 | BitBoard::B8.0),
        BitBoard(BitBoard::C1.0 | BitBoard::C2.0 | BitBoard::C3.0 | BitBoard::C4.0 | BitBoard::C5.0 | BitBoard::C6.0 | BitBoard::C7.0 | BitBoard::C8.0),
        BitBoard(BitBoard::D1.0 | BitBoard::D2.0 | BitBoard::D3.0 | BitBoard::D4.0 | BitBoard::D5.0 | BitBoard::D6.0 | BitBoard::D7.0 | BitBoard::D8.0),
        BitBoard(BitBoard::E1.0 | BitBoard::E2.0 | BitBoard::E3.0 | BitBoard::E4.0 | BitBoard::E5.0 | BitBoard::E6.0 | BitBoard::E7.0 | BitBoard::E8.0),
        BitBoard(BitBoard::F1.0 | BitBoard::F2.0 | BitBoard::F3.0 | BitBoard::F4.0 | BitBoard::F5.0 | BitBoard::F6.0 | BitBoard::F7.0 | BitBoard::F8.0),
        BitBoard(BitBoard::G1.0 | BitBoard::G2.0 | BitBoard::G3.0 | BitBoard::G4.0 | BitBoard::G5.0 | BitBoard::G6.0 | BitBoard::G7.0 | BitBoard::G8.0),
        BitBoard(BitBoard::H1.0 | BitBoard::H2.0 | BitBoard::H3.0 | BitBoard::H4.0 | BitBoard::H5.0 | BitBoard::H6.0 | BitBoard::H7.0 | BitBoard::H8.0),
    ];

    pub const FILE_A: BitBoard = BitBoard::FILES[0];
    pub const FILE_H: BitBoard = BitBoard::FILES[7];

    pub const RANK_1: BitBoard = BitBoard::RANKS[0];
    pub const RANK_8: BitBoard = BitBoard::RANKS[7];

    #[rustfmt::skip]
    pub const FRAME: BitBoard = BitBoard(BitBoard::RANK_1.0 | BitBoard::RANK_8.0 | BitBoard::FILE_A.0 | BitBoard::FILE_H.0);

    #[rustfmt::skip]
    pub const A1H8: [BitBoard; 15] = [
        BitBoard(BitBoard::A8.0),
        BitBoard(BitBoard::A7.0 | BitBoard::B8.0),
        BitBoard(BitBoard::A6.0 | BitBoard::B7.0 | BitBoard::C8.0),
        BitBoard(BitBoard::A5.0 | BitBoard::B6.0 | BitBoard::C7.0 | BitBoard::D8.0),
        BitBoard(BitBoard::A4.0 | BitBoard::B5.0 | BitBoard::C6.0 | BitBoard::D7.0 | BitBoard::E8.0),
        BitBoard(BitBoard::A3.0 | BitBoard::B4.0 | BitBoard::C5.0 | BitBoard::D6.0 | BitBoard::E7.0 | BitBoard::F8.0),
        BitBoard(BitBoard::A2.0 | BitBoard::B3.0 | BitBoard::C4.0 | BitBoard::D5.0 | BitBoard::E6.0 | BitBoard::F7.0 | BitBoard::G8.0),
        BitBoard(BitBoard::A1.0 | BitBoard::B2.0 | BitBoard::C3.0 | BitBoard::D4.0 | BitBoard::E5.0 | BitBoard::F6.0 | BitBoard::G7.0 | BitBoard::H8.0),
        BitBoard(BitBoard::B1.0 | BitBoard::C2.0 | BitBoard::D3.0 | BitBoard::E4.0 | BitBoard::F5.0 | BitBoard::G6.0 | BitBoard::H7.0),
        BitBoard(BitBoard::C1.0 | BitBoard::D2.0 | BitBoard::E3.0 | BitBoard::F4.0 | BitBoard::G5.0 | BitBoard::H6.0),
        BitBoard(BitBoard::D1.0 | BitBoard::E2.0 | BitBoard::F3.0 | BitBoard::G4.0 | BitBoard::H5.0),
        BitBoard(BitBoard::E1.0 | BitBoard::F2.0 | BitBoard::G3.0 | BitBoard::H4.0),
        BitBoard(BitBoard::F1.0 | BitBoard::G2.0 | BitBoard::H3.0),
        BitBoard(BitBoard::G1.0 | BitBoard::H2.0),
        BitBoard(BitBoard::H1.0),
    ];

    #[rustfmt::skip]
    pub const H8H1: [BitBoard; 15] = [
        BitBoard(BitBoard::A1.0),
        BitBoard(BitBoard::A2.0 | BitBoard::B1.0),
        BitBoard(BitBoard::A3.0 | BitBoard::B2.0 | BitBoard::C1.0),
        BitBoard(BitBoard::A4.0 | BitBoard::B3.0 | BitBoard::C2.0 | BitBoard::D1.0),
        BitBoard(BitBoard::A5.0 | BitBoard::B4.0 | BitBoard::C3.0 | BitBoard::D2.0 | BitBoard::E1.0),
        BitBoard(BitBoard::A6.0 | BitBoard::B5.0 | BitBoard::C4.0 | BitBoard::D3.0 | BitBoard::E2.0 | BitBoard::F1.0),
        BitBoard(BitBoard::A7.0 | BitBoard::B6.0 | BitBoard::C5.0 | BitBoard::D4.0 | BitBoard::E3.0 | BitBoard::F2.0 | BitBoard::G1.0),
        BitBoard(BitBoard::A8.0 | BitBoard::B7.0 | BitBoard::C6.0 | BitBoard::D5.0 | BitBoard::E4.0 | BitBoard::F3.0 | BitBoard::G2.0 | BitBoard::H1.0),
        BitBoard(BitBoard::B8.0 | BitBoard::C7.0 | BitBoard::D6.0 | BitBoard::E5.0 | BitBoard::F4.0 | BitBoard::G3.0 | BitBoard::H2.0),
        BitBoard(BitBoard::C8.0 | BitBoard::D7.0 | BitBoard::E6.0 | BitBoard::F5.0 | BitBoard::G4.0 | BitBoard::H3.0),
        BitBoard(BitBoard::D8.0 | BitBoard::E7.0 | BitBoard::F6.0 | BitBoard::G5.0 | BitBoard::H4.0),
        BitBoard(BitBoard::E8.0 | BitBoard::F7.0 | BitBoard::G6.0 | BitBoard::H5.0),
        BitBoard(BitBoard::F8.0 | BitBoard::G7.0 | BitBoard::H6.0),
        BitBoard(BitBoard::G8.0 | BitBoard::H7.0),
        BitBoard(BitBoard::H8.0),
    ];
}
