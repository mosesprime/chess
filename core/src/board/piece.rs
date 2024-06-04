pub const NUM_PIECE_SIDES: usize = 2;
pub const NUM_PIECE_KINDS: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
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
