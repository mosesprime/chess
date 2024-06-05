use std::str::FromStr;

use anyhow::{bail, Context};

use super::{file::{File, NUM_BOARD_FILES}, piece::{Piece, Side}, rank::{Rank, NUM_BOARD_RANKS}, square::Square, Board};

pub const DEFAULT_FEN_START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const NUM_FEN_FIELDS: usize = 6;

const WHITE_KING_SIDE_CASTLING: u8 = 1;
const WHITE_QUEEN_SIDE_CASTLING: u8 = 2;
const BLACK_KING_SIDE_CASTLING: u8 = 4;
const BLACK_QUEEN_SIDE_CASTLING: u8 = 8;
const ALL_CASTLING: u8 = 15;

impl Board {
    pub fn load_fen(&mut self, fen: &str) -> anyhow::Result<()> {
        let fields: Vec<String> = fen.split(' ').map(|s| s.to_string()).collect();
        if fields.len() != NUM_FEN_FIELDS {
            bail!("invalid number of FEN string fields");
        }
        let pos_field = fields.get(0).context("FEN string missing placement data")?;
        let side_field = fields.get(1).context("FEN string missing active side field")?;
        let castling_field = fields.get(2).context("FEN string missing castling field")?;
        let en_passant_field = fields.get(3).context("FEN string missing en passant field")?;
        let halfmove_clock_field = fields.get(4).context("FEN string missing halfmove clock field")?;
        let fullmove_number_field = fields.get(5).context("FEN string missing fullmove number field")?;

        let ranks: Vec<String> = pos_field.split('/').map(|s| s.to_string()).collect();
        if ranks.len() != NUM_BOARD_RANKS {
            bail!("invalid number of ranks in FEN string");
        }
        let mut rank = 8;
        let mut file = 0;
        for part in ranks {
            rank -= 1;
            for c in part.chars() {
                let mask = Square::from_coord(Rank::from_index(rank), File::from_index(file)).as_mask();
                match c {
                    'p' => *self.piece_mut(Side::Black, Piece::Pawn) |= mask,
                    'n' => *self.piece_mut(Side::Black, Piece::Knight) |= mask,
                    'b' => *self.piece_mut(Side::Black, Piece::Bishop) |= mask,
                    'r' => *self.piece_mut(Side::Black, Piece::Rook) |= mask,
                    'q' => *self.piece_mut(Side::Black, Piece::Queen) |= mask,
                    'k' => *self.piece_mut(Side::Black, Piece::King) |= mask,
                    'P' => *self.piece_mut(Side::White, Piece::Pawn) |= mask,
                    'N' => *self.piece_mut(Side::White, Piece::Knight) |= mask,
                    'B' => *self.piece_mut(Side::White, Piece::Bishop) |= mask,
                    'R' => *self.piece_mut(Side::White, Piece::Rook) |= mask,
                    'Q' => *self.piece_mut(Side::White, Piece::Queen) |= mask,
                    'K' => *self.piece_mut(Side::White, Piece::King) |= mask,
                    '1'..='8' => {
                        file += c.to_digit(10).expect("failed to parse number") as usize;
                        continue;
                    },
                    _ => bail!("invalid FEN string char: {c}"),
                }
                file += 1;
            }
            file = 0;
        }

        match side_field.as_str() {
            "w" => self.active_side = Side::White,
            "b" => self.active_side = Side::Black,
            _ => bail!("invalid FEN string active side"),
        }

        self.castling = 0;
        if (1..=4).contains(&castling_field.len()) {
            for c in castling_field.chars() {
                match c {
                    'K' => self.castling |= WHITE_KING_SIDE_CASTLING,
                    'Q' => self.castling |= WHITE_QUEEN_SIDE_CASTLING,
                    'k' => self.castling |= BLACK_KING_SIDE_CASTLING,
                    'q' => self.castling |= BLACK_QUEEN_SIDE_CASTLING,
                    _ => bail!("invalid FEN string castling"),
                }   
            }
        }

        if en_passant_field.as_str() == "-" {
            self.en_passant = None;
        } else {
            self.en_passant = Some(Square::from_str(en_passant_field.as_str())?);
        }

        self.halfmove_clock = halfmove_clock_field.as_str().parse::<u8>().context("failed to parse halfmove clock")?;

        self.fullmove_number = fullmove_number_field.as_str().parse::<u16>().context("failed to parse fullmove number")?;

        Ok(())
    }

    pub fn as_fen(&self) -> String {
        let mut fen = String::with_capacity(90);

        let mut empty_counter = 0;
        for rank in (0..NUM_BOARD_RANKS).rev() {
            let mut part = String::with_capacity(NUM_BOARD_FILES);
            for file in 0..NUM_BOARD_FILES {
                if let Some(res) = self.square(Square::from_coord(Rank::from_index(rank), File::from_index(file))) {
                    if empty_counter != 0 {
                        part.push(char::from_digit(empty_counter, 10).unwrap());
                        empty_counter = 0;
                    }
                    let c = match res {
                        (Side::White, Piece::Pawn) => 'P',
                        (Side::White, Piece::Knight) => 'N',
                        (Side::White, Piece::Bishop) => 'B',
                        (Side::White, Piece::Rook) => 'R',
                        (Side::White, Piece::Queen) => 'Q',
                        (Side::White, Piece::King) => 'K',
                        (Side::Black, Piece::Pawn) => 'p',
                        (Side::Black, Piece::Knight) => 'n',
                        (Side::Black, Piece::Bishop) => 'b',
                        (Side::Black, Piece::Rook) => 'r',
                        (Side::Black, Piece::Queen) => 'q',
                        (Side::Black, Piece::King) => 'k',
                    };
                    part.push(c);
                } else {
                    empty_counter += 1;
                }
            }
            if empty_counter != 0 {
                part.push(char::from_digit(empty_counter, 10).unwrap());
                empty_counter = 0;
            }
            if rank != 0 {
                part.push('/')
            }
            fen.push_str(part.as_str());
        }

        fen.push(' ');

        match self.active_side {
            Side::White => fen.push('w'),
            Side::Black => fen.push('b'),
        }

        fen.push(' ');

        let wk = (self.castling & WHITE_KING_SIDE_CASTLING) != 0;
        let wq = (self.castling & WHITE_QUEEN_SIDE_CASTLING) != 0;
        let bk = (self.castling & BLACK_KING_SIDE_CASTLING) != 0;
        let bq = (self.castling & BLACK_QUEEN_SIDE_CASTLING) != 0;
        if wk {
            fen.push('K');
        }
        if wq {
            fen.push('Q');
        }
        if bk {
            fen.push('k');
        }
        if bq {
            fen.push('q');
        }
        if !wk && !wq && !bk && !bq {
            fen.push('-');
        }

        fen.push(' ');

        match &self.en_passant {
            Some(square) => fen.push_str(square.name()),
            None => fen.push('-'),
        }

        fen.push(' ');

        fen.push_str(self.halfmove_clock.to_string().as_str());

        fen.push(' ');

        fen.push_str(self.fullmove_number.to_string().as_str());

        fen
    }
}

#[test]
fn fen_to_board_to_fen() {
    let mut board = Board::new();
    board.load_fen(DEFAULT_FEN_START).expect("failed to load fen");
    assert_eq!(board.as_fen(), DEFAULT_FEN_START)
}
