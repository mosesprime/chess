use super::Search;

pub struct NodeSearch {
    num_threads: usize,
    num_nodes: usize,
}

impl NodeSearch {
    pub fn new(num_threads: usize, num_nodes: usize) -> Self {
        Self { num_threads, num_nodes }
    }
}

impl Search for NodeSearch {
    fn best_move(&self) -> Option<chess_core::moves::ShortMove> {
        todo!()
    }

    fn report(&mut self) {
        todo!()
    }

    fn start(&mut self, search_tree: &super::tree::SearchTree, transpositions: std::sync::Arc<std::sync::RwLock<super::tt::TranspositonTable>>) {
        todo!()
    }

    fn stop(&mut self) -> chess_core::moves::ShortMove {
        todo!()
    }
}
