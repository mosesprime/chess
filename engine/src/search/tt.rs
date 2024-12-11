use std::collections::{BTreeMap, VecDeque};

use super::{EvalScore, Zobrist};

pub struct TranspositonTable {
    capacity: usize,
    eviction_queue: VecDeque<Zobrist>,
    inner: BTreeMap<Zobrist, EvalScore>,
}

impl TranspositonTable {
    pub fn new(capacity: usize) -> Self {
        Self { capacity, eviction_queue: VecDeque::with_capacity(capacity), inner: BTreeMap::new() }
    }
    
    fn insert(&mut self, zobrist: Zobrist, eval_score: EvalScore) -> Option<EvalScore> {
        if self.eviction_queue.len() >= self.capacity {
            let key = self.eviction_queue.pop_front().expect("transposition table eviction queue can not be at capacity and empty");
            self.inner.remove(&key);
        }
        self.eviction_queue.push_back(zobrist);
        self.inner.insert(zobrist, eval_score)
    }

    fn remove(&mut self, zobrist: Zobrist) -> Option<EvalScore> {
        self.eviction_queue.retain(|&z| z != zobrist);
        self.inner.remove(&zobrist)
    }

    fn query(&mut self, zobrist: Zobrist) -> Option<EvalScore> {
        if let Some(v) = self.inner.get(&zobrist) {
            if let Ok(i) = self.eviction_queue.binary_search(&zobrist) {
                self.eviction_queue.remove(i);
            }
            self.eviction_queue.push_back(zobrist);
            return Some(*v);
        }
        None
    }

    fn len(&self) -> usize {
        self.eviction_queue.len()
    }
}
