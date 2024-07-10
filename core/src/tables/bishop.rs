use crate::{board::{display_bitboard, file::{File, FILE_A, FILE_H}, rank::{RANK_1, RANK_8}, square::{Square, NUM_BOARD_SQUARES}, Bitboard, EMPTY_BITBOARD}, magic::{blocker_boards, Magic, BISHOP_MAGIC_NUMS, BISHOP_MAGIC_TABLE_SIZE}};

/// Generate all possible bishop move tables.
pub fn gen_bishop_moves() -> [Bitboard; NUM_BOARD_SQUARES] {
    let mut tables = [EMPTY_BITBOARD; NUM_BOARD_SQUARES];
    let a = File::A.as_mask();
    let h = File::H.as_mask();
    for sq in 0..NUM_BOARD_SQUARES {
        let mut ray_bb = 0;
        let square = Square::from(sq);
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

/// Generate all possible bishop magic tables.
pub fn gen_bishop_magics() -> (Vec<Bitboard>, [Magic; NUM_BOARD_SQUARES]) {
    let mut table = vec![EMPTY_BITBOARD; BISHOP_MAGIC_TABLE_SIZE];
    let mut magics = [Magic::default(); NUM_BOARD_SQUARES];
    let moves = gen_bishop_moves();
    let mut offset = 0;
    for sq in 0..NUM_BOARD_SQUARES {
        let square = Square::from(sq);
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
                attack |= lu | ru | ld | rd;
            }
            attack
        }).collect();
        let magic = Magic::new(mask, BISHOP_MAGIC_NUMS[sq], (64 - bits) as u8, offset);
        for next in 0..permutations {
            let index = magic.as_index(blockers[next as usize]);
            if table[index] == EMPTY_BITBOARD {
                assert!(!(index < offset as usize), "magic index is short of the offset");
                assert!(!(index > (offset + permutations - 1) as usize), "magic index excedes end");
                table[index] = attacks[next as usize]
            } else {
                panic!("bishop magic table index already occupied:\nindex: {}\nold:\n{}\nnew:\n{}",
                    index,
                    display_bitboard(table[index]),
                    display_bitboard(attacks[next as usize]),
                );
            }
        }
        magics[sq] = magic;
        offset += permutations;
    }
    (table, magics)
}
