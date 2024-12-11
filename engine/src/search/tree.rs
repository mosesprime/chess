use std::sync::{Arc, RwLock};

use chess_core::{board::Board, moves::{generate_moves, ShortMove}};

use crate::evaluate::Evaluate;

use super::tt::TranspositonTable;

/// Working search space. Uses a variation of Monte-Carlowe Tree Search.
pub struct SearchTree {
    root: SearchNode,
    board: Box<Board>,

    //num_nodes: AtomicUsize,
}

impl SearchTree {
    pub fn new(board: &Board) -> Self {
        let root = SearchNode { parent: None, children: vec![], eval_score: 0, siml_score: 0f32, short_move: ShortMove::INVALID };
        // TODO: 
        Self { root, board: Box::new(board.clone()) }
    }

    /// Return the best move from the searched space.
    pub fn best_move(&self) -> Option<ShortMove> {
        // TODO: change to simulation score max
        todo!();
    }

    fn decend(&self, choice: ShortMove) {
        todo!()
    }

    pub fn select(&self) -> (&Board, &mut SearchNode) {
        todo!()
    }
}

pub struct SearchNode {
    parent: Option<Box<SearchNode>>,
    children: Vec<SearchNode>, // TODO: change to short vec with optimized pagination
    eval_score: i32,
    siml_score: f32,
    short_move: ShortMove,
}

impl SearchNode {
    /// Generate moves from this node, evaluate, and sort for later.
    pub fn expand(&mut self, board: &Board, transpositions: Arc<RwLock<TranspositonTable>>) {
        self.children = generate_moves(board).into_iter().map(|m| unsafe {
            let mut variation = board.clone();
            if let Some((side, piece)) = variation.square(m.src()) {
                assert!(side == board.active_side(), "out of turn");
                if m.is_capturing() {
                    //variation.clear_square(m.dest()); // TODO: doesnt score capture   
                    todo!()
                }
                variation.move_piece(side, piece, m.src(), m.dest());
            } else {
                todo!()
            }
            /*let zobrist = todo!(); // zobrist hash of variation
            let eval_score = if let Some(eval_score) = transpositions.read().expect("transposition table poisoned").query(zobrist) {
                eval_score
            } else {
                let eval_score = Evaluate::material(board, perspective); // TODO: additional evals here
                transpositions.write().expect("transposition table poisoned").insert(zobrist, eval_score);
                eval_score
            };*/
            let eval_score = Evaluate::material(&variation);
            SearchNode {
                parent: Some(Box::from_raw(self)),
                children: vec![],
                eval_score,
                siml_score: eval_score as f32, // TODO: idk, sigmoid function?
                short_move: m,
            }
        }).collect();
        self.children.sort_by(|a, b| a.eval_score.cmp(&b.eval_score));
    }

    /// Use known evaluations to generate a upper confidence bound score.
    pub fn simulate(&mut self) {
        let mut div = 1;
        let mut score = 0f32;
        for child in self.children.iter() {
            div += 1;
            score += child.siml_score;
        }
        self.siml_score = score/div as f32;
    }

    pub fn backpropagate(&mut self) {
        if let Some(p) = &mut self.parent {
            p.simulate();
            p.backpropagate()
        }
    }
}
