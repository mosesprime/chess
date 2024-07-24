use crate::{board::{bitboard_square_iter, piece::Piece, square::Square, Board}, KING_MOVE_TABLE};

use super::{ShortMove, MoveList};

impl MoveList {
    pub fn add_king_moves(&mut self, board: &Board) {
        let active_side = board.active_side();
        let king = board.piece(active_side, Piece::King);
        let from = Square(king.trailing_zeros() as u8);
        let attacks = KING_MOVE_TABLE[from.0 as usize] & !board.side(active_side);
        for dest in bitboard_square_iter(attacks) {
            if dest.as_mask() & board.side(active_side.other()) != 0 {
                self.push(ShortMove::new(from, dest, ShortMove::CAPTURE_FLAG));
            } else {
                self.push(ShortMove::new(from, dest, 0));
            }
        }
    }

    // TODO: add castling moves
}
