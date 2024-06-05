use std::str::FromStr;

use anyhow::bail;

pub const NUM_BOARD_FILES: usize = 8;

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
    #[inline]
    pub fn from_index(value: usize) -> Self {
        // match should optimize to no-op, wraps if value > 7
        match value & 7 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn as_index(&self) -> usize {
        *self as usize
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
