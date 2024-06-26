use crate::board::{file::{FILE_A, FILE_H}, piece::{Side, NUM_PIECE_SIDES}, rank::{Rank, RANK_3, RANK_4, RANK_5, RANK_6}, square::{Square, NUM_BOARD_SQUARES}, Bitboard, EMPTY_BITBOARD};

/// Generate all possible pawn move tables.
pub fn gen_pawn_moves() -> [[Bitboard; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES] {
    let mut moves = [[EMPTY_BITBOARD; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES];
    for si in 0..NUM_PIECE_SIDES {
        for sq in 0..NUM_BOARD_SQUARES {
            let square = Square::from_index(sq);
            moves[si][sq] = match (Side::from_index(si), square.rank(), square.file()) {
                (Side::White, Rank::R2, file) => file.as_mask() & (RANK_3 | RANK_4),
                (Side::White, rank, file) => file.as_mask() & (rank.as_mask() << 8),
                (Side::Black, Rank::R7, file) => file.as_mask() & (RANK_6 | RANK_5),
                (Side::Black, rank, file) => file.as_mask() & (rank.as_mask() >> 8),
            };
        }
    }
    moves
}

/// Generate all possible pawn attack tables.
pub fn gen_pawn_attacks() -> [[Bitboard; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES] {
    let mut attacks = [[EMPTY_BITBOARD; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES]; 
    for si in 0..NUM_PIECE_SIDES {
        for sq in 0..NUM_BOARD_SQUARES {
            let mask = Square::from_index(sq).as_mask();
            attacks[si][sq] = match Side::from_index(si) {
                Side::White => ((mask << 7) & !FILE_H) | ((mask << 9) & !FILE_A),
                Side::Black => ((mask >> 9) & !FILE_H) | ((mask >> 7) & !FILE_A),
            };
        }
    }
    attacks
}
