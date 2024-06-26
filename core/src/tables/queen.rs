use crate::board::{square::NUM_BOARD_SQUARES, Bitboard, EMPTY_BITBOARD};

use super::{bishop::gen_bishop_moves, rook::gen_rook_moves};

/// Generate all possible queen move tables.
pub fn gen_queen_moves() -> [Bitboard; NUM_BOARD_SQUARES] {
    let mut tables = [EMPTY_BITBOARD; NUM_BOARD_SQUARES];
    let rook_moves = gen_rook_moves();
    let bishop_moves = gen_bishop_moves();
    for sq in 0..NUM_BOARD_SQUARES {
        tables[sq] = rook_moves[sq] | bishop_moves[sq];
    }
    tables
}
