use crate::{board::Board, moves::ShortMove};

pub struct GameState {
    history: GameHistory,
    current: Board,
}

impl GameState {
    pub fn new(start: Board) -> Self {
        Self {
            history: GameHistory {
                start: start.clone(),
                moves: vec![]
            },
            current: start,
        }
    }

    pub fn board(&self) -> &Board {
        &self.current
    }
}

pub struct GameHistory {
    start: Board,
    moves: Vec<ShortMove>,
}

impl GameHistory {
    pub fn from_fen(start: String, moves: Vec<ShortMove>) -> anyhow::Result<Self> {
        let mut board = Board::new();
        board.load_fen(start.as_str())?;
        Ok(Self { start: board, moves })
    }
}
