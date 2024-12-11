use std::sync::{Arc, RwLock};

use chess_core::moves::ShortMove;
use crossbeam::atomic::AtomicCell;
use tree::SearchTree;
use tt::TranspositonTable;

pub mod depth;
pub mod engine;
pub mod infinite;
pub mod node;
pub mod mate;
pub mod tree;
pub mod tt;

///
pub trait Search {
    /// Stop the search as soon as possible and return best move.
    fn stop(&mut self) -> ShortMove;
    /// Run the search.
    fn start(&mut self, search_tree: &SearchTree, transpositions: Arc<RwLock<TranspositonTable>>);
    /// Report search progress.
    fn report(&mut self);
    fn best_move(&self) -> Option<ShortMove>;
}

pub type CancelToken = AtomicCell<bool>;

type EvalScore = i32;
type SimulScore = f32;
const WIN_SCORE: SimulScore = f32::MAX;
const LOSS_SCORE: SimulScore = f32::MIN;

/// Zobrist hash key
type Zobrist = u64;


