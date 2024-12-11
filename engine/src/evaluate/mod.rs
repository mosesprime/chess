use chess_core::board::{piece::{Piece, Side, NUM_PIECE_KINDS}, Board};

// TODO: find good values
const MATERIAL_WEIGHT: [i32; NUM_PIECE_KINDS] = [100, 300, 350, 400, 900, 0];

pub struct Evaluate;

impl Evaluate {
    /// From white persepective
    pub fn material(board: &Board) -> i32 {
        let mut sum = 0;
        for p in 0..NUM_PIECE_KINDS {
            let piece = Piece::from(p);
            let value = MATERIAL_WEIGHT[p];
            sum += board.piece(Side::White, piece).count_ones() as i32 * value;
            sum -= board.piece(Side::Black, piece).count_ones() as i32 * value;
        }
        sum
    }
}
