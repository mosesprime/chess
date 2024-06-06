use crate::board::{square::{Square, NUM_BOARD_SQUARES}, Bitboard, EMPTY_BITBOARD};

pub const ROOK_MOVE_TABLES: [Bitboard; NUM_BOARD_SQUARES] = gen_rook_tables();
pub const ROOK_ATTACK_TABLES: [Bitboard; NUM_BOARD_SQUARES] = gen_rook_tables();

pub(crate) const fn gen_rook_tables() -> [Bitboard; NUM_BOARD_SQUARES] {
    let mut table = [EMPTY_BITBOARD; NUM_BOARD_SQUARES];
    let mut sq = 0;
    while sq < NUM_BOARD_SQUARES {
        let square = Square::from_index(sq);
        let rank = square.rank().as_mask();
        let file = square.file().as_mask();
        table[sq] = rank ^ file;
        sq += 1;
    }
    table
}
