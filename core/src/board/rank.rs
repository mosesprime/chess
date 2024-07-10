use std::str::FromStr;

use anyhow::bail;

use super::Bitboard;

pub const NUM_BOARD_RANKS: usize = 8;
pub const RANK_1: Bitboard = 0x00000000000000FF;
pub const RANK_2: Bitboard = 0x000000000000FF00;
pub const RANK_3: Bitboard = 0x0000000000FF0000;
pub const RANK_4: Bitboard = 0x00000000FF000000;
pub const RANK_5: Bitboard = 0x000000FF00000000;
pub const RANK_6: Bitboard = 0x0000FF0000000000;
pub const RANK_7: Bitboard = 0x00FF000000000000;
pub const RANK_8: Bitboard = 0xFF00000000000000;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum Rank {
    R1 = 0,
    R2 = 1,
    R3 = 2,
    R4 = 3,
    R5 = 4,
    R6 = 5,
    R7 = 6,
    R8 = 7,
}

impl Rank {
    pub const fn as_mask(&self) -> Bitboard {
        RANK_1 << (*self as usize * 8)
    }
}

impl From<u8> for Rank {
    fn from(value: u8) -> Self {
        debug_assert!(value < 8, "rank value out of bounds");
        unsafe { std::mem::transmute(value) }
    }
}

impl From<usize> for Rank {
    fn from(value: usize) -> Self {
        Self::from(value as u8)
    }
}

impl FromStr for Rank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c) = s.chars().next() {
            let rank = match c {
                '1' => Rank::R1,
                '2' => Rank::R2,
                '3' => Rank::R3,
                '4' => Rank::R4,
                '5' => Rank::R5,
                '6' => Rank::R6,
                '7' => Rank::R7,
                '8' => Rank::R8, 
                _ => bail!("invalid rank character"),
            };
            return Ok(rank);
        }
        bail!("missing rank character")
    }
}
