use std::{sync::{Arc, RwLock}, thread::JoinHandle};

use chess_core::moves::ShortMove;

use super::{tree::SearchTree, tt::TranspositonTable, CancelToken, Search};

pub struct InfiniteSearch {
    num_threads: usize,
    best_move: Option<ShortMove>,
    cancel_token: CancelToken,
    handles: Vec<JoinHandle<()>>
}

impl InfiniteSearch {
    pub fn new(num_threads: usize) -> Self {
        let cancel_token = CancelToken::new(false.into());
        Self { num_threads, best_move: None, cancel_token, handles: Vec::with_capacity(num_threads) }
    }
}

impl Search for InfiniteSearch {
    fn stop(&mut self) -> chess_core::moves::ShortMove {
        self.cancel_token.store(true);
        // TODO: self.engine.best_move()
        todo!()
    }

    fn start(&mut self, search_tree: &SearchTree, transpositions: Arc<RwLock<TranspositonTable>>) {
        crossbeam::thread::scope(|s| {
            for _ in 0..self.num_threads {
                let tt = Arc::clone(&transpositions);
                let cancel_token = &self.cancel_token;
                s.spawn(move |_| {
                    loop {
                        if cancel_token.load() {
                            break;
                        }
                        let (board, node) = search_tree.select();
                        node.expand(board, tt.clone());
                        node.simulate();
                        node.backpropagate();
                    }
                });
            }
        }).unwrap(); // TODO: handle thread error
        todo!() 
    }

    fn report(&mut self) {
        todo!()
    }

    fn best_move(&self) -> Option<ShortMove> {
        self.best_move
    }
}
