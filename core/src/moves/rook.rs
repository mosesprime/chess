use crate::{board::{bitboard_square_iter, piece::Piece, Board}, ROOK_ATTACK_TABLE, ROOK_MAGIC_TABLE};

use super::{Move, MoveList};

impl MoveList {
    pub fn add_rook_moves(&mut self, board: &Board) {
        let active_side = board.active_side();
        let rooks = board.piece(active_side, Piece::Rook);
        for from in bitboard_square_iter(rooks) {
            let magic = ROOK_MAGIC_TABLE[from.0 as usize];
            let attacks = ROOK_ATTACK_TABLE[magic.as_index(board.occupied())] & !board.side(active_side.other_side());
            for dest in bitboard_square_iter(attacks) {
                if dest.as_mask() & board.side(active_side) > 0 {
                    self.push(Move::new(from, dest, Move::CAPTURE));
                } else {
                    self.push(Move::new(from, dest, 0));
                }
            }
        }
    }
}
