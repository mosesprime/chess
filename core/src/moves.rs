use crate::{board::{bitboard_square_iter, piece::{Piece, Side}, rank::Rank, square::Square, Bitboard, Board, EMPTY_BITBOARD}, PAWN_ATTACK_TABLE, PAWN_MOVE_TABLE};

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

/// generate pseudo-legal moves
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

#[derive(Clone, Copy)]
pub struct Move(pub u16);

impl Move {
    const FROM: usize = 0;
    const DEST: usize = 6;
    const FLAGS: usize = 12; 
    const CAPTURE: u16 = 0b0001_0000_0000_0000;
    const CASTLING: u16 = 0b0010_0000_0000_0000;
    const EN_PASANT: u16 = 0b0100_0000_0000_0000;
    //const DOUBLE_STEP: u16 = 0b0100_0000_0000_0000;
    const KNIGHT_PROMOTION: u16 = 0b1000_0000_0000_0000;
    const BISHOP_PROMOTION: u16 = 0b1010_0000_0000_0000;
    const ROOK_PROMOTION: u16 = 0b1100_0000_0000_0000;
    const QUEEN_PROMOTION: u16 = 0b1110_0000_0000_0000;
    pub const INVALID: Move = Move(0);

    pub fn new(from: Square, dest: Square, flags: u16) -> Self {
        Self(from.0 as u16 | (dest.0 << Self::DEST) as u16 | (flags.wrapping_shl(Self::FLAGS as u32) as u16))
    }

    pub fn is_valid(&self) -> bool {
        (self.0 & 0x0FFF) != 0
    }

    pub fn from(&self) -> Square {
        Square::from_index(((self.0 >> Self::FROM) & 0x3F) as usize)
    }

    pub fn dest(&self) -> Square {
        Square::from_index(((self.0 >> Self::DEST) & 0x3F) as usize)
    }

    pub fn promoted(&self) -> Option<Piece> {
        // TODO: ensure that captures and promotions dont conflict
        match self.0 >> Self::FLAGS {
            Self::KNIGHT_PROMOTION => Some(Piece::Knight),
            Self::BISHOP_PROMOTION => Some(Piece::Bishop),
            Self::ROOK_PROMOTION => Some(Piece::Rook),
            Self::QUEEN_PROMOTION => Some(Piece::Queen),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct MoveList {
    list: [Move; MAX_LEGAL_MOVES],
    count: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self { 
            // PERF: IDK if this is actually faster than zeroed
            list: unsafe { [std::mem::MaybeUninit::uninit().assume_init(); MAX_LEGAL_MOVES] },
            count: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn push(&mut self, m: Move) {
        self.list[self.count] = m;
        self.count += 1;
        debug_assert!(self.count <= MAX_LEGAL_MOVES, "exceded max legal moves");
    }

    pub fn get(&self, index: usize) -> Move {
        debug_assert!(index <= MAX_LEGAL_MOVES, "index excedes max legal moves");
        debug_assert!(index <= self.count, "index out of bounds");
        self.list[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Move {
        debug_assert!(index <= MAX_LEGAL_MOVES, "index excedes max legal moves");
        debug_assert!(index <= self.count, "index out of bounds");
        &mut self.list[index]
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        debug_assert!((a <= self.count) && (b <= self.count), "index out of bounds");
        self.list.swap(a, b)
    }
}
