pub const NUM_PIECE_SIDES: usize = 2;
pub const NUM_PIECE_KINDS: usize = 6;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    White = 0,
    Black = 1,
}

impl Side {
    pub const fn from_index(index: usize) -> Self {
        match index & 1 {
            0 => Side::White,
            _ => Side::Black,
        }
    }

    pub fn other_side(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
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

impl Piece {
    pub fn from_index(index: usize) -> Self {
        match index & 5 {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => unreachable!(),
        }
    }
}

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
