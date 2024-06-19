use std::fmt;

use super::{file::NUM_BOARD_FILES, rank::NUM_BOARD_RANKS};

#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const FILE_A: BitBoard = BitBoard(0x0101010101010101);
    pub const FILE_B: BitBoard = BitBoard(0x0202020202020202);
    pub const FILE_C: BitBoard = BitBoard(0x0404040404040404);
    pub const FILE_D: BitBoard = BitBoard(0x0808080808080808);
    pub const FILE_E: BitBoard = BitBoard(0x1010101010101010);
    pub const FILE_F: BitBoard = BitBoard(0x2020202020202020);
    pub const FILE_G: BitBoard = BitBoard(0x4040404040404040);
    pub const FILE_H: BitBoard = BitBoard(0x8080808080808080);
    pub const RANK_1: BitBoard = BitBoard(0x00000000000000FF);
    pub const RANK_2: BitBoard = BitBoard(0x000000000000FF00);
    pub const RANK_3: BitBoard = BitBoard(0x0000000000FF0000);
    pub const RANK_4: BitBoard = BitBoard(0x00000000FF000000);
    pub const RANK_5: BitBoard = BitBoard(0x000000FF00000000);
    pub const RANK_6: BitBoard = BitBoard(0x0000FF0000000000);
    pub const RANK_7: BitBoard = BitBoard(0x00FF000000000000);
    pub const RANK_8: BitBoard = BitBoard(0xFF00000000000000);
    pub const FULL: BitBoard = BitBoard(!0);
    pub const EMPTY: BitBoard = BitBoard(0);    
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(128);
        for rank in (0..NUM_BOARD_RANKS).rev() {
            for file in 0..NUM_BOARD_FILES {
                s.push(match self.0 >> (file + rank * 8) & 1 {
                    0 => '.',
                    _ => '#',
                });
                if file != NUM_BOARD_FILES - 1 {
                    s.push(' ');
                }                
            }
            if rank != 0 {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}
