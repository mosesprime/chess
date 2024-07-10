use crate::{board::{display_bitboard, file::{FILE_A, FILE_H, NUM_BOARD_FILES}, rank::{NUM_BOARD_RANKS, RANK_1, RANK_8}, square::{Square, NUM_BOARD_SQUARES}, Bitboard, EMPTY_BITBOARD}, magic::{blocker_boards, Magic, ROOK_MAGIC_NUMS, ROOK_MAGIC_TABLE_SIZE}};

/// Generate all possible rook move tables.
pub fn gen_rook_moves() -> [Bitboard; NUM_BOARD_SQUARES] {
    let mut tables = [EMPTY_BITBOARD; NUM_BOARD_SQUARES];
    for n in 0..NUM_BOARD_SQUARES {
        let square = Square::from(n);
        let rank = square.rank().as_mask();
        let file = square.file().as_mask();
        tables[n] = rank ^ file;
    }
    tables
}

/// Generate all possible rook magic tables.
pub fn gen_rook_magics() -> (Vec<Bitboard>, [Magic; NUM_BOARD_SQUARES]) {
    let mut table = vec![EMPTY_BITBOARD; ROOK_MAGIC_TABLE_SIZE];
    let mut magics = [Magic::default(); NUM_BOARD_SQUARES];
    let mut offset = 0;
    for sq in 0..NUM_BOARD_SQUARES {
        let square = Square::from(sq);
        let mask = {
            let rank = square.rank().as_mask();
            let file = square.file().as_mask();
            let edges = (FILE_A & !file) | (FILE_H & !file) | (RANK_1 & !rank) | (RANK_8 & !rank);
            (rank | file) & !edges & !square.as_mask()
        };
        let bits = mask.count_ones();
        let permutations = 2u64.pow(bits);
        let blockers = blocker_boards(mask);
        let attacks: Vec<Bitboard> = blockers.iter().map(|blocker| {
            let mut attack = 0;
            let mut left = square.as_mask();
            let mut right = square.as_mask();
            for _ in 0..NUM_BOARD_FILES {
                left = (left << 1) & !FILE_H;
                right = (right >> 1) & !FILE_A;
                attack |= left | right;
                left &= !blocker;
                right &= !blocker;
            }
            let mut up = square.as_mask();
            let mut down = square.as_mask();
            for _ in 0..NUM_BOARD_RANKS {
                up = (up << 8) & !RANK_8;
                down = (down >> 8) & !RANK_1;
                attack |= up | down;
                up &= !blocker;
                down &= !blocker;
            }
            attack & !square.as_mask() 
        }).collect();
        let magic = Magic::new(mask, ROOK_MAGIC_NUMS[sq], (64 - bits) as u8, offset);
        for next in 0..permutations {
            let index = magic.as_index(blockers[next as usize]);
            if table[index] == EMPTY_BITBOARD {
                assert!(!(index < offset as usize), "magic index is short of the offset");
                assert!(!(index > (offset + permutations - 1) as usize), "magic index excedes end");
                table[index] = attacks[next as usize]
            } else {
                panic!("rook magic table index already occupied:\nindex: {}\nold:\n{}\nnew:\n{}",
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
