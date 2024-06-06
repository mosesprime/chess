use crate::board::{piece::{Piece, Side, NUM_PIECE_SIDES}, square::{Square, NUM_BOARD_SQUARES}, Bitboard, EMPTY_BITBOARD};

pub mod bishop;
mod king;
mod knight;
mod magic;
mod pawn;
pub mod queen;
pub mod rook;

pub struct MoveGenerator {
    pawn: [[Bitboard; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES],
    knight: [Bitboard; NUM_BOARD_SQUARES],
    king: [Bitboard; NUM_BOARD_SQUARES],
}
