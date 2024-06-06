use std::fmt::Display;

use crate::board::{file::{File, NUM_BOARD_FILES}, rank::{Rank, NUM_BOARD_RANKS}};

use self::{fen::DEFAULT_FEN_START, piece::{Piece, Side, BLACK_BISHOP_UNICODE, BLACK_KING_UNICODE, BLACK_KNIGHT_UNICODE, BLACK_PAWN_UNICODE, BLACK_QUEEN_UNICODE, BLACK_ROOK_UNICODE, NUM_PIECE_KINDS, NUM_PIECE_SIDES, WHITE_BISHOP_UNICODE, WHITE_KING_UNICODE, WHITE_KNIGHT_UNICODE, WHITE_PAWN_UNICODE, WHITE_QUEEN_UNICODE, WHITE_ROOK_UNICODE}, square::{Square, NUM_BOARD_SQUARES, RANK_NAMES}};

pub mod fen;
pub mod file;
pub mod piece;
pub mod rank;
pub mod square;

pub const EMPTY_BITBOARD: Bitboard = 0;

pub type Bitboard = u64;

pub fn display_bitboard(bitboard: Bitboard) -> String {
    let mut board = String::with_capacity(128);
    for r in (0..8).rev() {
        for f in 0..8 {
            board.push(match bitboard >> (f + r * 8) & 1 {
                0 => '.',
                _ => '#',
            });
            if f != 7 {
                board.push(' ');
            }
        }
        if r != 0 {
            board.push('\n');
        }
    }
    board
}

pub struct Board {
    /// Piece placement data.
    bitboards: [[Bitboard; NUM_PIECE_KINDS]; NUM_PIECE_SIDES],
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
            bitboards: [[EMPTY_BITBOARD; NUM_PIECE_KINDS]; NUM_PIECE_SIDES],
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

    pub fn square(&self, square: Square) -> Option<(Side, Piece)> {
        for s in 0..NUM_PIECE_SIDES {
            for p in 0..NUM_PIECE_KINDS {
                let bb = self.bitboards[s][p];
                if (bb & square.as_mask()) != 0 {
                    let side = match s {
                        0 => Side::White,
                        1 => Side::Black,
                        _ => unreachable!(),
                    };
                    let piece = match p {
                        0 => Piece::Pawn,
                        1 => Piece::Knight,
                        2 => Piece::Bishop,
                        3 => Piece::Rook,
                        4 => Piece::Queen,
                        5 => Piece::King,
                        _ => unreachable!(),
                    };
                    return Some((side, piece));
                }
            }
        }
        None
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::new();
        board.load_fen(DEFAULT_FEN_START).expect("failed to load default fen");
        board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.as_fen())?;
        let mut board = String::with_capacity(NUM_BOARD_SQUARES);
        for rank in (0..NUM_BOARD_RANKS).rev() {
            board.push(RANK_NAMES[rank]);
            board.push(' ');
            for file in 0..NUM_BOARD_FILES {
                if let Some((side, piece)) = self.square(Square::from_coord(Rank::from_index(rank), File::from_index(file))) {
                    let c = match (side, piece) {
                        (Side::White, Piece::Pawn) => WHITE_PAWN_UNICODE,
                        (Side::White, Piece::Knight) => WHITE_KNIGHT_UNICODE,
                        (Side::White, Piece::Bishop) => WHITE_BISHOP_UNICODE,
                        (Side::White, Piece::Rook) => WHITE_ROOK_UNICODE,
                        (Side::White, Piece::Queen) => WHITE_QUEEN_UNICODE,
                        (Side::White, Piece::King) => WHITE_KING_UNICODE,
                        (Side::Black, Piece::Pawn) => BLACK_PAWN_UNICODE,
                        (Side::Black, Piece::Knight) => BLACK_KNIGHT_UNICODE,
                        (Side::Black, Piece::Bishop) => BLACK_BISHOP_UNICODE,
                        (Side::Black, Piece::Rook) => BLACK_ROOK_UNICODE,
                        (Side::Black, Piece::Queen) => BLACK_QUEEN_UNICODE,
                        (Side::Black, Piece::King) => BLACK_KING_UNICODE,
                    };
                    board.push(c);
                    board.push(' ');
                } else {
                    board.push_str(". ");
                }
            }
            board.push('\n');
        }
        board.push_str("  a b c d e f g h");
        write!(f, "{}", board)
    }
}

#[test]
fn board_load_and_query() {
    use self::fen::DEFAULT_FEN_START;
    use std::str::FromStr;
    
    let mut board = Board::new();
    let square = Square::from_str("d1").expect("failed to parse notation");
    board.load_fen(DEFAULT_FEN_START).expect("failed to parse fen");
    match board.square(square) {
        Some((side, piece)) => {
            assert_eq!(piece, Piece::Queen, "flipped pieces");
            assert_eq!(side, Side::White, "flipped sides");
        },
        None => assert!(false)
    }
}
