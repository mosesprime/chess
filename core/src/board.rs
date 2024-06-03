use self::{piece::{Piece, Side}, square::Square};

mod fen;
mod piece;
mod square;

pub const EMPTY_BITBOARD: Bitboard = 0;
pub const NUM_BOARD_RANKS: usize = 8;

pub type Bitboard = u64;

pub struct Board {
    /// Piece placement data.
    bitboards: [[Bitboard; 6]; 2],
    /// Which side is to move.
    active_side: Side,
    /// Availability to castle.
    castling: u8,
    /// Square over which a pawn hhas just passed while moving two squares.
    en_passant: Option<Square>,
    /// Number of halfmoves since last capture or pawn advance, used for fifty-move rule.
    halfmove_clock: u8,
    /// Number of full moves, starting at 1. Incriments after Black's move.
    fullmove_number: u16,
}

impl Board {
    pub fn new() -> Self {
        Self { 
            bitboards: [[EMPTY_BITBOARD; 6]; 2],
            active_side: Side::White,
            castling: 0,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn piece(&self, side: Side, piece: Piece) -> Bitboard {
        self.bitboards[side as usize][piece as usize]
    }

    pub fn piece_mut(&mut self, side: Side, piece: Piece) -> &mut Bitboard {
        &mut self.bitboards[side as usize][piece as usize]
    }

    pub fn side(&self, side: Side) -> Bitboard {
        let side = side as usize;
        self.bitboards[side][0] | self.bitboards[side][1] | self.bitboards[side][2] | self.bitboards[side][3] | self.bitboards[side][4] | self.bitboards[side][5]
    }

    pub fn occupied(&self) -> Bitboard {
        self.side(Side::White) | self.side(Side::Black)
    }

    pub fn remove_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.bitboards[side as usize][piece as usize] ^= square.as_mask()
    }

    pub fn place_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.bitboards[side as usize][piece as usize] |= square.as_mask()
    }

    pub fn move_piece(&mut self, side: Side, piece: Piece, from_square: Square, to_square: Square) {
        self.remove_piece(side, piece, from_square);
        self.place_piece(side, piece, to_square);
    }
}
