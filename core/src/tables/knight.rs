use crate::board::{file::{FILE_A, FILE_B, FILE_G, FILE_H}, square::{Square, NUM_BOARD_SQUARES}, Bitboard, EMPTY_BITBOARD};

/// Generate all possible knight move tables.
pub fn gen_knight_moves() -> [Bitboard; NUM_BOARD_SQUARES]  {
    let mut tables = [EMPTY_BITBOARD; NUM_BOARD_SQUARES];
    let a = FILE_A;
    let b = FILE_B;
    let g = FILE_G;
    let h = FILE_H;
    for n in 0..NUM_BOARD_SQUARES {
        let sq = Square::from(n).as_mask();
        tables[n] = ((sq << 6) & !(g | h))
            | ((sq << 15) & !h)
            | ((sq << 17) & !a)
            | ((sq << 10) & !(a | b))
            | ((sq >> 6) & !(a | b))
            | ((sq >> 15) & !a)
            | ((sq >> 17) & !h)
            | ((sq >> 10) & !(g | h));
    }
    tables
}
