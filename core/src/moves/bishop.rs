use crate::{board::{bitboard_square_iter, piece::Piece, Board}, BISHOP_ATTACK_TABLE, BISHOP_MAGIC_TABLE};

use super::{ShortMove, MoveList};

impl MoveList {
    pub fn add_bishop_moves(&mut self, board: &Board) {
        let active_side = board.active_side();
        let bishops = board.piece(active_side, Piece::Bishop);
        for from in bitboard_square_iter(bishops) {
            let magic = BISHOP_MAGIC_TABLE[from.0 as usize];
            let attacks = BISHOP_ATTACK_TABLE[magic.as_index(board.occupied())] & !board.side(active_side);
            for dest in bitboard_square_iter(attacks) {
                if (dest.as_mask() & board.side(active_side.other())) != 0 {
                    self.push(ShortMove::new(from, dest, ShortMove::CAPTURE_FLAG));
                } else {
                    self.push(ShortMove::new(from, dest, 0));
                }
            }
        }
    }
}
