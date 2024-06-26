use crate::{board::{bitboard_square_iter, piece::Piece, Board}, BISHOP_ATTACK_TABLE, BISHOP_MAGIC_TABLE, ROOK_ATTACK_TABLE, ROOK_MAGIC_TABLE};

use super::{Move, MoveList};

impl MoveList {
    pub fn add_queen_moves(&mut self, board: &Board) {
        let active_side = board.active_side();
        let queens = board.piece(active_side, Piece::Queen);
        for from in bitboard_square_iter(queens) {
            let bishop_magic = BISHOP_MAGIC_TABLE[from.0 as usize];
            let rook_magic = ROOK_MAGIC_TABLE[from.0 as usize];
            let occupied = board.occupied();
            let attacks = (BISHOP_ATTACK_TABLE[bishop_magic.as_index(occupied)] | ROOK_ATTACK_TABLE[rook_magic.as_index(occupied)]) & !board.side(active_side);
            for dest in bitboard_square_iter(attacks) {
                if dest.as_mask() & board.side(active_side.other_side()) > 0 {
                    self.push(Move::new(from, dest, Move::CAPTURE));
                } else {
                    self.push(Move::new(from, dest, 0));
                }
            }
        }
    }
}
