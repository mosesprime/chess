

#[derive(Clone, Copy)]
pub enum Side {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

const WHITE_KING_UNICODE: char = '\u{2654}';
const WHITE_QUEEN_UNICODE: char = '\u{2655}';
const WHITE_ROOK_UNICODE: char = '\u{2656}';
const WHITE_BISHOP_UNICODE: char = '\u{2657}';
const WHITE_KNIGHT_UNICODE: char = '\u{2658}';
const WHITE_PAWN_UNICODE: char = '\u{2659}';
const BLACK_KING_UNICODE: char = '\u{265A}';
const BLACK_QUEEN_UNICODE: char = '\u{265B}';
const BLACK_ROOK_UNICODE: char = '\u{265C}';
const BLACK_BISHOP_UNICODE: char = '\u{265D}';
const BLACK_KNIGHT_UNICODE: char = '\u{265E}';
const BLACK_PAWN_UNICODE: char = '\u{265F}';
