use std::str::FromStr;

use anyhow::{bail, Context};

use super::{Bitboard, NUM_BOARD_FILES, NUM_BOARD_RANKS};

pub const NUM_BOARD_SQUARES: usize = 64;
pub const FILE_NAMES: [char; NUM_BOARD_FILES] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub const RANK_NAMES: [char; NUM_BOARD_RANKS] = ['8', '7', '6', '5', '4', '3', '2', '1'];
#[rustfmt::skip]
pub const SQUARE_NAMES: [&str; NUM_BOARD_SQUARES] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",    
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];

pub struct Square(u8);

impl Square {
    pub fn from_coord(rank: u8, file: u8) -> Self {
        debug_assert!(rank <= 7, "rank out of bounds ");
        debug_assert!(file <= 7, "file out of bounds");
        Self((rank * 8) + file)
    }

    pub fn as_mask(&self) -> Bitboard {
        0 | (1 << self.0)        
    }

    fn rank(&self) -> u8 {
        self.0 / 8
    }

    fn file(&self) -> u8 {
        self.0 % 8
    }

    pub fn name(&self) -> &str {
        SQUARE_NAMES[self.0 as usize]
    }
}

impl FromStr for Square {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut n = match chars.next().context("notation missing file char")? {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => bail!("invalid file char"),
        };
        match chars.next().context("notation missing rank char")? {
            '1' => n += 8 * 7,
            '2' => n += 8 * 6,
            '3' => n += 8 * 5,
            '4' => n += 8 * 4,
            '5' => n += 8 * 3,
            '6' => n += 8 * 2,
            '7' => n += 8 * 1,
            '8' => n += 8 * 0, 
            _ => bail!("invalid rank char"),
        }
        if chars.next().is_some() {
            bail!("invalid notation length")
        }
        Ok(Square(n)) 
    }
}

#[test]
fn square_as_mask() {
    let s = Square::from_coord(2, 2);
    let b: Bitboard = 0b0000000000000000000000000000000000000000000001000000000000000000;
    assert_eq!(s.as_mask(), b)
}

#[test]
fn square_name() {
    let s = Square::from_coord(2, 2);
    assert_eq!(s.name(), "c6")
}

#[test]
fn square_parse() {
    match Square::from_str("c6") {
        Ok(s) => assert_eq!(s.name(), "c6"),
        Err(e) => assert!(false, "{}", e),        
    }
}
