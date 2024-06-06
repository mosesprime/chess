use crate::board::{file::File, piece::{Side, NUM_PIECE_SIDES}, rank::Rank, square::{Square, NUM_BOARD_SQUARES}, Bitboard, EMPTY_BITBOARD};

pub const PAWN_ATTACK_TABLES: [[Bitboard; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES] = gen_pawn_attack_tables();
pub const PAWN_MOVE_TABLES: [[Bitboard; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES] = gen_pawn_move_tables();

/// Generate all possible pawn move tables.
const fn gen_pawn_move_tables() -> [[Bitboard; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES] {
    let mut moves = [[EMPTY_BITBOARD; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES];
    let mut si = 0;
    let mut sq = 0;
    while si < NUM_PIECE_SIDES {
        while sq < NUM_BOARD_SQUARES {
            let square = Square::from_index(sq);
            moves[si][sq] = match (Side::from_index(si), square.rank(), square.file()) {
                (Side::White, Rank::R2, file) => file.as_mask() & (Rank::R3.as_mask() | Rank::R4.as_mask()),
                (Side::White, rank, file) => file.as_mask() & (rank.as_mask() << 8),
                (Side::Black, Rank::R7, file) => file.as_mask() & (Rank::R6.as_mask() | Rank::R5.as_mask()),
                (Side::Black, rank, file) => file.as_mask() & (rank.as_mask() >> 8),
            };
            sq += 1;
        }
        sq = 0;
        si += 1;
    }
    moves
}

/// Generate all possible pawn attack tables.
const fn gen_pawn_attack_tables() -> [[Bitboard; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES] {
    let mut attacks = [[EMPTY_BITBOARD; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES]; 
    let mut si = 0;
    let mut sq = 0;
    while si < NUM_PIECE_SIDES {
        while sq < NUM_BOARD_SQUARES {
            let mask = sq as u64;
            attacks[si][sq] = match Side::from_index(si) {
                Side::White => ((mask << 7) & !File::H.as_mask()) | ((mask << 9) & !File::A.as_mask()),
                Side::Black => ((mask >> 9) & !File::H.as_mask()) | ((mask >> 7) & !File::A.as_mask()),
            };
            sq += 1;
        }
        sq = 0;
        si += 1;
    }
    attacks
}
