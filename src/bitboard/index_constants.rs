use crate::bitboard::Index;

impl Index {
    pub const A1: Index = Index(0);
    pub const B1: Index = Index(1);
    pub const C1: Index = Index(2);
    pub const D1: Index = Index(3);
    pub const E1: Index = Index(4);
    pub const F1: Index = Index(5);
    pub const G1: Index = Index(6);
    pub const H1: Index = Index(7);

    pub const A2: Index = Index(8);
    pub const B2: Index = Index(9);
    pub const C2: Index = Index(10);
    pub const D2: Index = Index(11);
    pub const E2: Index = Index(12);
    pub const F2: Index = Index(13);
    pub const G2: Index = Index(14);
    pub const H2: Index = Index(15);

    pub const A3: Index = Index(16);
    pub const B3: Index = Index(17);
    pub const C3: Index = Index(18);
    pub const D3: Index = Index(19);
    pub const E3: Index = Index(20);
    pub const F3: Index = Index(21);
    pub const G3: Index = Index(22);
    pub const H3: Index = Index(23);

    pub const A4: Index = Index(24);
    pub const B4: Index = Index(25);
    pub const C4: Index = Index(26);
    pub const D4: Index = Index(27);
    pub const E4: Index = Index(28);
    pub const F4: Index = Index(29);
    pub const G4: Index = Index(30);
    pub const H4: Index = Index(31);

    pub const A5: Index = Index(32);
    pub const B5: Index = Index(33);
    pub const C5: Index = Index(34);
    pub const D5: Index = Index(35);
    pub const E5: Index = Index(36);
    pub const F5: Index = Index(37);
    pub const G5: Index = Index(38);
    pub const H5: Index = Index(39);

    pub const A6: Index = Index(40);
    pub const B6: Index = Index(41);
    pub const C6: Index = Index(42);
    pub const D6: Index = Index(43);
    pub const E6: Index = Index(44);
    pub const F6: Index = Index(45);
    pub const G6: Index = Index(46);
    pub const H6: Index = Index(47);

    pub const A7: Index = Index(48);
    pub const B7: Index = Index(49);
    pub const C7: Index = Index(50);
    pub const D7: Index = Index(51);
    pub const E7: Index = Index(52);
    pub const F7: Index = Index(53);
    pub const G7: Index = Index(54);
    pub const H7: Index = Index(55);

    pub const A8: Index = Index(56);
    pub const B8: Index = Index(57);
    pub const C8: Index = Index(58);
    pub const D8: Index = Index(59);
    pub const E8: Index = Index(60);
    pub const F8: Index = Index(61);
    pub const G8: Index = Index(62);
    pub const H8: Index = Index(63);

    #[rustfmt::skip]
    pub const FIELDS: [Index; 64] = [
        Index::A1, Index::B1, Index::C1, Index::D1, Index::E1, Index::F1, Index::G1, Index::H1,
        Index::A2, Index::B2, Index::C2, Index::D2, Index::E2, Index::F2, Index::G2, Index::H2,
        Index::A3, Index::B3, Index::C3, Index::D3, Index::E3, Index::F3, Index::G3, Index::H3,
        Index::A4, Index::B4, Index::C4, Index::D4, Index::E4, Index::F4, Index::G4, Index::H4,
        Index::A5, Index::B5, Index::C5, Index::D5, Index::E5, Index::F5, Index::G5, Index::H5,
        Index::A6, Index::B6, Index::C6, Index::D6, Index::E6, Index::F6, Index::G6, Index::H6,
        Index::A7, Index::B7, Index::C7, Index::D7, Index::E7, Index::F7, Index::G7, Index::H7,
        Index::A8, Index::B8, Index::C8, Index::D8, Index::E8, Index::F8, Index::G8, Index::H8,
    ];
}
