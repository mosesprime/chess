use std::thread::JoinHandle;

use chess_core::moves::ShortMove;

use super::Search;

pub struct DepthSearch {
    num_threads: usize,
    depth: usize,
    handle: Vec<JoinHandle<()>>,
}

impl DepthSearch {
    pub fn new(num_threads: usize, depth: usize) -> Self {
        Self {
            num_threads,
            depth,
            handle: vec![]
        }
    }
}

impl Search for DepthSearch {
    fn stop(&mut self) -> ShortMove {
        todo!()
    }

    fn start(&mut self, search_tree: &super::tree::SearchTree, transpositions: std::sync::Arc<std::sync::RwLock<super::tt::TranspositonTable>>) {
        todo!()
    }

    fn report(&mut self) {
        todo!()
    }

    fn best_move(&self) -> Option<ShortMove> {
        todo!()
    }
}
