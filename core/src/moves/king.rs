use crate::board::{file::File, square::NUM_BOARD_SQUARES, Bitboard, EMPTY_BITBOARD};

pub const KING_MOVE_TABLES: [Bitboard; NUM_BOARD_SQUARES] = gen_king_tables();
pub const KING_ATTACK_TABLES: [Bitboard; NUM_BOARD_SQUARES] = gen_king_tables();

const fn gen_king_tables() -> [Bitboard; NUM_BOARD_SQUARES] {
    let mut table = [EMPTY_BITBOARD; NUM_BOARD_SQUARES];
    let mut square = 0;
    let a = File::A.as_mask();
    let h = File::H.as_mask();
    while square < NUM_BOARD_SQUARES {
        let sq = square as u64;
        table[square] = ((sq << 1) & !h)
            | ((sq << 7) & !a)
            | (sq << 8)
            | ((sq << 9) & !h)
            | ((sq >> 1) & !a)
            | ((sq >> 7) & !h)
            | (sq >> 8)
            | ((sq >> 9) & !a);
        square += 1;
    }
    table
}

