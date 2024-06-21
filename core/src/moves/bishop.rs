use crate::board::{display_bitboard, file::{File, FILE_A, FILE_H}, rank::{RANK_1, RANK_8}, square::{Square, NUM_BOARD_SQUARES}, Bitboard, EMPTY_BITBOARD};

use super::magic::{blocker_boards, magic_index, BISHOP_MAGIC_NUMS, BISHOP_MAGIC_TABLE_SIZE};

/// Generate all possible bishop move tables.
pub fn gen_bishop_moves() -> [Bitboard; NUM_BOARD_SQUARES] {
    let mut tables = [EMPTY_BITBOARD; NUM_BOARD_SQUARES];
    let a = File::A.as_mask();
    let h = File::H.as_mask();
    for sq in 0..NUM_BOARD_SQUARES {
        let mut ray_bb = 0;
        let square = Square::from_index(sq);
        let mut up_rank = square.rank().as_mask();
        let mut down_rank = square.rank().as_mask();
        let mut left_file = square.file().as_mask();
        let mut right_file = square.file().as_mask();
        for _ in 0..8 {
            up_rank <<= 8;
            down_rank >>= 8;
            left_file = (left_file << 1) & !(h << 1);
            right_file = (right_file >> 1) & !(a >> 1);
            ray_bb |= up_rank & (left_file | right_file);
            ray_bb |= down_rank & (left_file | right_file);
        }
        tables[sq] = ray_bb;
        }
    tables
}


pub fn gen_bishop_magics() -> [Bitboard; BISHOP_MAGIC_TABLE_SIZE] {
    let mut magics = [EMPTY_BITBOARD; BISHOP_MAGIC_TABLE_SIZE];
    let moves = gen_bishop_moves();
    let mut offset = 0;
    for sq in 0..NUM_BOARD_SQUARES {
        let square = Square::from_index(sq);
        let mask = moves[sq] & !RANK_1 & !RANK_8 & !FILE_A & !FILE_H;
        let bits = mask.count_ones();
        let permutations = 2u64.pow(bits);
        let blockers = blocker_boards(mask);
        let attacks: Vec<Bitboard> = blockers.iter().map(|blocker| {
            let mut attack = 0;
            let mut lu = square.as_mask();
            let mut ru = square.as_mask(); 
            let mut ld = square.as_mask();
            let mut rd = square.as_mask();
            for _ in 0..8 {
                lu = (lu << 7) & !FILE_H & !blocker;
                ru = (ru << 9) & !FILE_A & !blocker;
                ld = (ld >> 7) & !FILE_H & !blocker;
                rd = (rd >> 9) & !FILE_A & !blocker;
                attack |= (lu | ru | ld | rd) & mask;
            }
            attack
        }).collect();
        for next in 0..permutations {
            let shift = (64 - bits) as u8;
            let index = magic_index(mask, blockers[next as usize], BISHOP_MAGIC_NUMS[sq], shift, offset);
            if magics[index] == EMPTY_BITBOARD {
                assert!(!(index < offset as usize), "magic index is short of the offset");
                assert!(!(index > (offset + permutations - 1) as usize), "magic index excedes end");
                magics[index] = attacks[next as usize]
            } else {
                let old = magics[index];
                panic!("bishop magic table index already occupied:\nindex: {}\nold:\n{}\nnew:\n{}",
                    index,
                    display_bitboard(old),
                    display_bitboard(attacks[next as usize]),
                );
            }
        }
        offset += permutations;
    }
    magics
}
