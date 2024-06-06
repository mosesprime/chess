use crate::board::{square::NUM_BOARD_SQUARES, Bitboard, EMPTY_BITBOARD};

use super::{bishop::gen_bishop_tables, rook::gen_rook_tables};

pub const QUEEN_MOVE_TABLES: [Bitboard; NUM_BOARD_SQUARES] = gen_queen_tables();
pub const QUEEN_ATTACK_TABLES: [Bitboard; NUM_BOARD_SQUARES] = gen_queen_tables();

const fn gen_queen_tables() -> [Bitboard; NUM_BOARD_SQUARES] {
    let mut tables = [EMPTY_BITBOARD; NUM_BOARD_SQUARES];
    let mut sq = 0;
    let rook_tables = gen_rook_tables();
    let bishop_tables = gen_bishop_tables();
    while sq < NUM_BOARD_SQUARES {
        tables[sq] = rook_tables[sq] | bishop_tables[sq];
        sq += 1;
    }
    tables
}
