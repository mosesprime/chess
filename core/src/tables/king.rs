use crate::board::{file::{FILE_A, FILE_H}, square::{Square, NUM_BOARD_SQUARES}, Bitboard, EMPTY_BITBOARD};

/// Generate all possible king move tables. 
pub fn gen_king_moves() -> [Bitboard; NUM_BOARD_SQUARES] {
    let mut table = [EMPTY_BITBOARD; NUM_BOARD_SQUARES];
    let a = FILE_A;
    let h = FILE_H;
    for n in 0..NUM_BOARD_SQUARES {
        let sq = Square::from_index(n).as_mask();
        table[n] = ((sq << 1) & !a)
            | ((sq << 7) & !h)
            | ((sq << 9) & !a)
            | (sq << 8)
            | (sq >> 8)
            | ((sq >> 7) & !a)
            | ((sq >> 9) & !h)
            | ((sq >> 1) & !h);
    }
    table
}

