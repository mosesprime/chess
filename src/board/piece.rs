pub const NUM_PIECE_SIDES: usize = 2;
pub const NUM_PIECE_KINDS: usize = 6;

pub const WHITE_KING_UNICODE: char = '\u{2654}';
pub const WHITE_QUEEN_UNICODE: char = '\u{2655}';
pub const WHITE_ROOK_UNICODE: char = '\u{2656}';
pub const WHITE_BISHOP_UNICODE: char = '\u{2657}';
pub const WHITE_KNIGHT_UNICODE: char = '\u{2658}';
pub const WHITE_PAWN_UNICODE: char = '\u{2659}';
pub const BLACK_KING_UNICODE: char = '\u{265A}';
pub const BLACK_QUEEN_UNICODE: char = '\u{265B}';
pub const BLACK_ROOK_UNICODE: char = '\u{265C}';
pub const BLACK_BISHOP_UNICODE: char = '\u{265D}';
pub const BLACK_KNIGHT_UNICODE: char = '\u{265E}';
pub const BLACK_PAWN_UNICODE: char = '\u{265F}';

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    White = 0,
    Black = 1,
}

impl Side {
    pub fn other(&self) -> Self {
        Side::from(*self as u8 ^ 1)
    }
}

impl From<u8> for Side {
    fn from(value: u8) -> Self {
        debug_assert!(value < 2, "side value out of bounds");
        unsafe { std::mem::transmute(value) }
    }
}

impl From<usize> for Side {
    fn from(value: usize) -> Self {
        Self::from(value as u8)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl From<u8> for Piece {
    fn from(value: u8) -> Self {
        debug_assert!(value < 6, "piece value out of bounds");
        unsafe { std::mem::transmute(value) }
    }
}

impl From<usize> for Piece {
    fn from(value: usize) -> Self {
        Self::from(value as u8)
    }
}
