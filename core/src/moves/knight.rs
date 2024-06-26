use crate::{board::{bitboard_square_iter, piece::Piece, Board}, KNIGHT_MOVE_TABLE};

use super::{Move, MoveList};

impl MoveList {
    pub fn add_knight_moves(&mut self, board: &Board) {
        let active_side = board.active_side();
        let knights = board.piece(active_side, Piece::Knight);
        for from in bitboard_square_iter(knights) {
            let attacks = KNIGHT_MOVE_TABLE[from.0 as usize];
            for dest in bitboard_square_iter(attacks & !board.side(active_side)) {
                if (dest.as_mask() & board.side(active_side.other_side())) > 0 {
                    self.push(Move::new(from, dest, Move::CAPTURE));
                } else {
                    self.push(Move::new(from, dest, 0));
                }
            }
        }
    }
}
