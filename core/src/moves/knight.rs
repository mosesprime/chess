use crate::board::{file::File, square::{NUM_BOARD_SQUARES}, Bitboard, EMPTY_BITBOARD};

pub const KNIGHT_MOVE_TABLES: [Bitboard; NUM_BOARD_SQUARES] = gen_knight_tables();
pub const KNIGHT_ATTACK_TABLES: [Bitboard; NUM_BOARD_SQUARES] = gen_knight_tables();

const fn gen_knight_tables() -> [Bitboard; NUM_BOARD_SQUARES]  {
    let mut table = [EMPTY_BITBOARD; NUM_BOARD_SQUARES];
    let mut square = 0;
    let a = File::A.as_mask();
    let b = File::B.as_mask();
    let g = File::G.as_mask();
    let h = File::H.as_mask();
    while square < NUM_BOARD_SQUARES {
        let sq = square as u64;
        table[square] = ((sq << 6) & !(g | h))
            | ((sq << 15) & !h)
            | ((sq << 17) & !a)
            | ((sq << 10) & !(a | b))
            | ((sq >> 6) & !(a | b))
            | ((sq >> 15) & !a)
            | ((sq >> 17) & !h)
            | ((sq >> 10) & !(g | h));
        square += 1;
    }
    table
}
