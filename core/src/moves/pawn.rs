use crate::{board::{bitboard_square_iter, piece::Piece, rank::Rank, Board, EMPTY_BITBOARD}, PAWN_ATTACK_TABLE, PAWN_MOVE_TABLE};

use super::{Move, MoveList};

impl MoveList {
    pub fn add_pawn_moves(&mut self, board: &Board) {
        let active_side = board.active_side();
        let pawns = board.piece(active_side, Piece::Pawn);
        for from in bitboard_square_iter(pawns) {
            let pushes = PAWN_MOVE_TABLE[active_side as usize][from.0 as usize] & !board.occupied();
            let en_passant = board.en_passant().map_or(EMPTY_BITBOARD, |sq| sq.as_mask());
            let enemy = board.side(active_side.other());
            let attacks = PAWN_ATTACK_TABLE[active_side as usize][from.0 as usize] & (en_passant | enemy);
            for dest in bitboard_square_iter(pushes | attacks) {
                let mut flags = 0;
                if (dest.as_mask() & attacks) > 0 {
                    flags &= Move::CAPTURE;
                }
                if (dest.as_mask() & en_passant) > 0 {
                    flags &= Move::EN_PASANT;
                }
                if dest.rank() == Rank::R8 || dest.rank()  == Rank::R1 {
                    self.push(Move::new(from, dest, flags & Move::KNIGHT_PROMOTION));
                    self.push(Move::new(from, dest, flags & Move::BISHOP_PROMOTION));
                    self.push(Move::new(from, dest, flags & Move::ROOK_PROMOTION));
                    self.push(Move::new(from, dest, flags & Move::QUEEN_PROMOTION));
                } else {
                    self.push(Move::new(from, dest, flags));
                }
            }    
        }
    }
}
