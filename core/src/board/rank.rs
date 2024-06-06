use std::str::FromStr;

use anyhow::bail;

use super::Bitboard;

pub const NUM_BOARD_RANKS: usize = 8;

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
    #[inline]
    pub const fn from_index(value: usize) -> Self {
        // match should get optimized to no-op, wraps if value > 7
        match value & 7 {
            0 => Self::R1,
            1 => Self::R2,
            2 => Self::R3,
            3 => Self::R4,
            4 => Self::R5,
            5 => Self::R6,
            6 => Self::R7,
            7 => Self::R8,
            _ => unreachable!(),
        }
    }

    pub const fn as_index(&self) -> usize {
        *self as usize
    }

    pub const fn as_mask(&self) -> Bitboard {
        let r1: Bitboard = 0xFF;
        r1 << (self.as_index() * 8)
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
