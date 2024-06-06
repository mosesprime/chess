use crate::board::{file::File, square::{Square, NUM_BOARD_SQUARES}, Bitboard, EMPTY_BITBOARD};

pub const BISHOP_MOVE_TABLES: [Bitboard; NUM_BOARD_SQUARES] = gen_bishop_tables();
pub const BISHOP_ATTACK_TABLES: [Bitboard; NUM_BOARD_SQUARES] = gen_bishop_tables();

pub(crate) const fn gen_bishop_tables() -> [Bitboard; NUM_BOARD_SQUARES] {
    let mut tables = [EMPTY_BITBOARD; NUM_BOARD_SQUARES];
    let mut sq = 0; 
    let a = File::A.as_mask();
    let h = File::H.as_mask();
    while sq < NUM_BOARD_SQUARES {
        let mut ray_bb = 0;
        let mut ray_step = 0;
        let square = Square::from_index(sq);
        let mut up_rank = square.rank().as_mask();
        let mut down_rank = square.rank().as_mask();
        let mut left_file = square.file().as_mask();
        let mut right_file = square.file().as_mask();
        while ray_step < 8 {
            up_rank <<= 8;
            down_rank >>= 8;
            left_file = (left_file << 1) & !(h << 1);
            right_file = (right_file >> 1) & !(a >> 1);
            ray_bb |= up_rank & (left_file | right_file);
            ray_bb |= down_rank & (left_file | right_file);
            ray_step += 1;
        }
        tables[sq] = ray_bb;
        sq += 1; 
        }
    tables
}
