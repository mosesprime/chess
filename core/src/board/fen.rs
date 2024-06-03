use anyhow::{bail, Context};

use super::{piece::{Piece, Side}, square::Square, Board, NUM_BOARD_RANKS};

pub const DEFAULT_FEN_START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const NUM_FEN_FIELDS: usize = 6;

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
        let mut rank = 7;
        let mut file = 0;
        for part in ranks {
            for c in part.chars() {
                let mask = Square::from_coord(rank, file).as_mask();
                match c {
                    'p' => *self.piece_mut(Side::White, Piece::Pawn) |= mask,
                    'n' => *self.piece_mut(Side::White, Piece::Knight) |= mask,
                    'b' => *self.piece_mut(Side::White, Piece::Bishop) |= mask,
                    'r' => *self.piece_mut(Side::White, Piece::Rook) |= mask,
                    'q' => *self.piece_mut(Side::White, Piece::Queen) |= mask,
                    'k' => *self.piece_mut(Side::White, Piece::King) |= mask,
                    'P' => *self.piece_mut(Side::Black, Piece::Pawn) |= mask,
                    'N' => *self.piece_mut(Side::Black, Piece::Knight) |= mask,
                    'B' => *self.piece_mut(Side::Black, Piece::Bishop) |= mask,
                    'R' => *self.piece_mut(Side::Black, Piece::Rook) |= mask,
                    'Q' => *self.piece_mut(Side::Black, Piece::Queen) |= mask,
                    'K' => *self.piece_mut(Side::Black, Piece::King) |= mask,
                    '1'..='8' => {
                        file += c.to_digit(10).expect("failed to parse number") as u8;
                        continue;
                    },
                    _ => bail!("invalid FEN string char: {c}"),
                }
                file += 1;
            }
            rank -= 1;
            file = 0;
        }

        match side_field.as_str() {
            "w" => self.active_side = Side::White,
            "b" => self.active_side = Side::Black,
            _ => bail!("invalid FEN string active side"),
        }

        todo!("parse castling");
        todo!("parse en passant");
        todo!("parse halfmove clock");
        todo!("parse fullmove number");

        Ok(())
    }

    pub fn as_fen(&self) -> &str {
        todo!("board to FEN string")  
    }
}
