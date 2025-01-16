use std::{fmt::Debug, ops::Deref};

use crate::{board::{bitboard_square_iter, piece::{Piece, Side}, rank::Rank, square::Square, Bitboard, Board, EMPTY_BITBOARD}, PAWN_ATTACK_TABLE, PAWN_MOVE_TABLE};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

/// Generate pseudo-legal moves. Does not account for illegal moves like moving a pinned piece or
/// castling whil under attack.
pub fn generate_moves(board: &Board) -> MoveList {
    let mut moves = MoveList::new();
    moves.add_pawn_moves(board);
    moves.add_knight_moves(board);
    moves.add_bishop_moves(board);
    moves.add_rook_moves(board);
    moves.add_queen_moves(board);
    moves.add_king_moves(board);
    // TODO: add castling moves
    moves
}

/// Theoretical max number of possible legal moves.
pub const MAX_LEGAL_MOVES: usize = 218;

#[derive(Clone, Copy, PartialEq)]
pub struct ShortMove(pub(crate) u16);

impl ShortMove {
    const FROM_OFFSET: usize = 0;
    const FROM_MASK: u16 = 0b0000_0000_0011_1111;
    const DEST_OFFSET: usize = 6;
    const DEST_MASK: u16 = 0b0000_1111_1100_0000;
    const FLAGS_OFFSET: usize = 12;
    const FLAGS_MASK: u16 = 0b1111_0000_0000_0000;
    pub const CAPTURE_FLAG: u16 = 0b0001_0000_0000_0000;
    pub const CASTLING_FLAG: u16 = 0b0010_0000_0000_0000;
    pub const EN_PASANT_FLAG: u16 = 0b0100_0000_0000_0000;
    const PROMOTION_MASK: u16 = 0b1110_0000_0000_0000;
    pub const KNIGHT_PROMOTION_FLAG: u16 = 0b1000_0000_0000_0000;
    pub const BISHOP_PROMOTION_FLAG: u16 = 0b1010_0000_0000_0000;
    pub const ROOK_PROMOTION_FLAG: u16 = 0b1100_0000_0000_0000;
    pub const QUEEN_PROMOTION_FLAG: u16 = 0b1110_0000_0000_0000;
    pub const INVALID: ShortMove = ShortMove(0);

    pub fn new(src: Square, dest: Square, flags: u16) -> Self {
        Self(src.0 as u16 | ((dest.0 as u16) << Self::DEST_OFFSET) | (flags << Self::FLAGS_OFFSET))
    }

    /// Checks validity of [ShortMove]. Does not necessarily check move for legality.
    pub fn is_valid(&self) -> bool {
        debug_assert!(self.src() != self.dest(), "src and dest should not be the same");
        debug_assert!((self.is_en_pasant() && self.is_capturing()) && self.is_capturing(), "all en pasant should also be captures"); 
        debug_assert!(!(self.is_capturing() && self.is_castling()), "should not be able to castle and capture simultaneously");
        *self != Self::INVALID
    }

    pub fn src(&self) -> Square {
        Square::from((self.0 >> Self::FROM_OFFSET) & 0x3F)
    }

    pub fn dest(&self) -> Square {
        Square::from((self.0 >> Self::DEST_OFFSET) & 0x3F)
    }

    pub fn promoted(&self) -> Option<Piece> {
        // TODO: ensure that flags dont conflict
        match self.0 & Self::PROMOTION_MASK {
            Self::KNIGHT_PROMOTION_FLAG => Some(Piece::Knight),
            Self::BISHOP_PROMOTION_FLAG => Some(Piece::Bishop),
            Self::ROOK_PROMOTION_FLAG => Some(Piece::Rook),
            Self::QUEEN_PROMOTION_FLAG => Some(Piece::Queen),
            _ => None,
        }
    }

    pub fn is_capturing(&self) -> bool {
        self.0 & Self::CAPTURE_FLAG != 0
    }

    pub fn is_en_pasant(&self) -> bool {
        self.0 & Self::EN_PASANT_FLAG != 0
    }

    pub fn is_castling(&self) -> bool {
        self.0 & Self::CASTLING_FLAG != 0
    }

    pub fn set_src(&mut self, src: Square) {
        self.0 &= !Self::FROM_MASK;
        self.0 |= (src.0 << Self::FROM_OFFSET) as u16;
    }

    pub fn set_dest(&mut self, dest: Square) {
        self.0 &= !Self::DEST_MASK;
        self.0 |= (dest.0 << Self::DEST_OFFSET) as u16;
    }
}

impl Default for ShortMove {
    fn default() -> Self {
        Self::INVALID
    }
}

impl From<u16> for ShortMove {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Deref for ShortMove {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for ShortMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ShortMove({:#b})", self.0)
    }
}

#[derive(Clone, Copy)]
pub struct MoveList {
    list: [ShortMove; MAX_LEGAL_MOVES],
    count: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self { 
            // PERF: IDK if this is actually faster than zeroed?
            list: unsafe { [std::mem::MaybeUninit::uninit().assume_init(); MAX_LEGAL_MOVES] },
            count: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn push(&mut self, short_move: ShortMove) {
        self.list[self.count] = short_move;
        self.count += 1;
        debug_assert!(self.count <= MAX_LEGAL_MOVES, "exceded max legal moves");
    }

    pub fn get(&self, index: usize) -> ShortMove {
        debug_assert!(index <= MAX_LEGAL_MOVES, "index excedes max legal moves");
        debug_assert!(index <= self.count, "index out of bounds");
        self.list[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut ShortMove {
        debug_assert!(index <= MAX_LEGAL_MOVES, "index excedes max legal moves");
        debug_assert!(index <= self.count, "index out of bounds");
        &mut self.list[index]
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        debug_assert!((a <= self.count) && (b <= self.count), "index out of bounds");
        self.list.swap(a, b) // PERF: use nightly swap_unchecked?
    }

    pub fn into_iter(self) -> impl Iterator<Item = ShortMove> {
        let mut n = 0;
        std::iter::from_fn(move || {
            if n < self.count {
                Some(self.get(n))
            } else {
                n += 1;
                None
            }
        })
    }

    pub fn as_slice(&self) -> &[ShortMove] {
        &self.list[0..self.count]
    }

    pub fn contains(&self, m: &ShortMove) -> bool {
        self.list.as_slice().contains(m)
    }
}
