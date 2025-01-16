use std::str::FromStr;

use anyhow::bail;

use super::Bitboard;

pub const NUM_BOARD_FILES: usize = 8;
pub const FILE_A: Bitboard = 0x0101010101010101;
pub const FILE_B: Bitboard = 0x0202020202020202;
pub const FILE_C: Bitboard = 0x0404040404040404;
pub const FILE_D: Bitboard = 0x0808080808080808;
pub const FILE_E: Bitboard = 0x1010101010101010;
pub const FILE_F: Bitboard = 0x2020202020202020;
pub const FILE_G: Bitboard = 0x4040404040404040;
pub const FILE_H: Bitboard = 0x8080808080808080;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl File {
    pub const fn as_mask(&self) -> Bitboard {
        FILE_A << *self as usize
    }
}

impl From<u8> for File {
    fn from(value: u8) -> Self {
        debug_assert!(value < 8, "file value out of bounds");
        unsafe { std::mem::transmute(value) }
    }
}

impl From<usize> for File {
    fn from(value: usize) -> Self {
        Self::from(value as u8)
    }
}

impl FromStr for File {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c) = s.chars().next() {
            let file = match c {
                'a' => File::A,
                'b' => File::B,
                'c' => File::C,
                'd' => File::D,
                'e' => File::E,
                'f' => File::F,
                'g' => File::G,
                'h' => File::H,
                _ => bail!("invalid file character"),
            };
            return Ok(file);
        }
        bail!("missing file character")
    }
}
