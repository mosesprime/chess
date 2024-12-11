use super::Search;

pub struct MateSearch {
    num_threads: usize,
    mate_in: usize,
}

impl MateSearch {
    pub fn new(num_threads: usize, mate_in: usize) -> Self {
        Self { num_threads, mate_in }
    }
}

impl Search for MateSearch {
    fn stop(&mut self) -> chess_core::moves::ShortMove {
        todo!()
    }

    fn start(&mut self, search_tree: &super::tree::SearchTree, transpositions: std::sync::Arc<std::sync::RwLock<super::tt::TranspositonTable>>) {
        todo!()
    }

    fn report(&mut self) {
        todo!()
    }

    fn best_move(&self) -> Option<chess_core::moves::ShortMove> {
        todo!()
    }
}
