use crate::{board::Board, moves::Move};

pub struct GameState {
    start: Board,
    current: Board,
    moves: Vec<Move>,
}

impl GameState {
    pub fn new(start: Board) -> Self {
        let mut board = Board::new();
        board.load_fen(start.as_fen().as_str()).expect("failed to duplicate board");
        Self {
            start,
            current: board,
            moves: vec![],
        }
    }
}
