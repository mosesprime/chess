use std::{borrow::{Borrow, BorrowMut}, cell::{Cell, RefCell}, collections::{BTreeMap, BTreeSet, VecDeque}, fmt::write, io::{self, Write}, mem::uninitialized, ops::Deref, rc::{Rc, Weak}, sync::{atomic::{AtomicBool, AtomicPtr, AtomicUsize}, Arc}, thread::JoinHandle, time::{Duration, Instant}};

use anyhow::{bail, Context};
use chess_core::{board::{piece::Side, Board}, game::GameState, moves::{generate_moves, ShortMove}, uci::{Check, IdEvent, OptionEvent, Spin, UciEvent}};
use crossbeam::{atomic::AtomicCell, channel::{Receiver, Sender}};

pub mod depth;
pub mod infinite;
pub mod nodes;

pub const ENGINE_NAME: &str = "MonteCristo";
pub const ENGINE_AUTHOR: &str = "mosesprime";

const DEFAULT_NUM_THREADS: usize = 1;
const MINIMUM_NUM_THREADS: usize = 1;
const DEFAULT_HASH_CAPACITY: usize = 16;
const DEFAULT_CAN_PONDER: bool = true;

/// Engine options. Should be done before engine initialization.
pub struct Config {
    /// Number of threads to use in next search.
    pub num_threads: usize,
    /// Capacity in MB of hash tables.
    pub hash_capacity: usize,
    /// Allow pondering.
    pub can_ponder: bool,
    /// Run in analysis mode.
    pub analyse_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            num_threads: DEFAULT_NUM_THREADS,
            hash_capacity: DEFAULT_HASH_CAPACITY,
            can_ponder: DEFAULT_CAN_PONDER,
            analyse_mode: false
        }
    }
}

/// Core structure of the engine. Methods should be non-blocking to prevent interuption to REPL.
/// Search-Worker threads should manipulate fields independently. Risk to thread safety but necessary.
pub struct Engine {
    /// Current game state.
    game_state: GameState,
    /// Search space in question.
    search_tree: SearchTree,
    /// The search currently being run.
    searching: Option<Box<dyn Search>>,
    /// Store of previously evaluated board states.
    transpositions: TranspositonTable,
    /// 
    pondering: Option<ShortMove>,
    /// 
    pub config: Config,
}

impl Engine {
    pub fn new(start: Board, config: Config) -> Self {
        Self {
            game_state: GameState::new(start),
            search_tree: SearchTree::new(),
            searching: None,
            transpositions: TranspositonTable::new(config.hash_capacity),
            pondering: None,
            config,
        }
    }

    /// Stop the current search and report best move.
    pub fn stop(&mut self) {
        if let Some(s) = &mut self.searching {
            s.stop();
        }
        let best = todo!();
        let ponder = todo!();
        UciEvent::BestMove { best, ponder }.write()
    } 

    pub fn quit(self) { todo!() }

    pub fn reset(&mut self, game_state: GameState) {
        if let Some(s) = &mut self.searching {
            let _ = s.stop();
            self.searching = None;
        }
        self.pondering = None;
        self.game_state = game_state;
        self.search_tree = SearchTree::new();
    }

    pub fn report_about(&self) {
        UciEvent::Id(IdEvent::Name(ENGINE_NAME.to_string())).write();
        UciEvent::Id(IdEvent::Author(ENGINE_AUTHOR.to_string())).write();
        // TODO: report non-hard codes defaults
        UciEvent::Option(OptionEvent::Threads(Spin { default: 1, min: 1, max: 32 })).write();
        UciEvent::Option(OptionEvent::Hash(Spin { default: 16, min: 1, max: 16_000 })).write();
        UciEvent::Option(OptionEvent::Ponder(Check { default: DEFAULT_CAN_PONDER })).write();
        // TODO: add options here
    }

    pub fn apply_move(&mut self, choice: ShortMove) {
        todo!()
    }

    pub fn ponder_hit(&mut self) {
        // TODO: handle instead of panic
        let choice = self.pondering.expect("missing pondering move");
        self.apply_move(choice);
        self.pondering = None;
    }
}

pub(crate) trait Search {
    /// Stop the current search and report the best move.
    fn stop(self: &mut Self) -> ShortMove;
}

/// Working search space.
pub struct SearchTree {
    root: SearchNode,
}

impl SearchTree {
    pub fn new() -> Self {
        let root = SearchNode { parent: None, children: vec![], eval_score: 0, siml_score: None, short_move: ShortMove::INVALID };
        Self { root }
    }

    /// Return the best move from the searched space.
    pub fn best_move(&self) -> Option<ShortMove> {
        // TODO: change to simulation score max
        let x = self.root.children.iter().max_by(|a, b| a.eval_score.cmp(&b.eval_score))?;
        Some(x.short_move)
    }

    fn decend(&mut self, choice: ShortMove) -> anyhow::Result<()> {
        self.root = self.root.get_child(choice).context("missing child node")?.to_owned();
        Ok(())
    }
}

#[derive(Clone)]
struct SearchNode {
    pub parent: Option<Box<SearchNode>>,
    pub children: Vec<Box<SearchNode>>,
    // pub zobrist: Zobrist,
    pub eval_score: i32,
    pub siml_score: Option<f32>,
    pub short_move: ShortMove,
}

impl SearchNode {
    fn get_child(&self, choice: ShortMove) -> Option<&SearchNode> {
        Some(self.children.iter().find(|m| m.short_move == choice)?)
    }

    fn get_child_mut(&mut self, choice: ShortMove) -> Option<&mut SearchNode> {
        Some(self.children.iter_mut().find(|m| m.short_move == choice)?)
    }

    fn get_parent(&self) -> Option<&SearchNode> {
        match &self.parent {
            Some(p) => Some(p.as_ref()),
            None => None,
        }
    }

    fn get_parent_mut(&mut self) -> Option<&mut SearchNode> {
        match &mut self.parent {
            Some(p) => Some(p.as_mut()),
            None => None,
        }
    }

    fn x(&mut self) {
        if let Some(p) = self.get_parent_mut() {
            p.eval_score = 0;
        }
    } 
}

/// Piece-Square Table score
type Score = f32;
const WIN_SCORE: f32 = f32::MAX;
const LOSS_SCORE: f32 = f32::MIN;

/// Zobrist hash key
type Zobrist = u64;

pub struct TranspositonTable {
    capacity: usize,
    eviction_queue: VecDeque<Zobrist>,
    inner: BTreeMap<Zobrist, Score>,
}

impl TranspositonTable {
    pub fn new(capacity: usize) -> Self {
        Self { capacity, eviction_queue: VecDeque::with_capacity(capacity), inner: BTreeMap::new() }
    }
    
    fn insert(&mut self, zobrist: Zobrist, score: Score) -> Option<Score> {
        if self.eviction_queue.len() >= self.capacity {
            let key = self.eviction_queue.pop_front().expect("transposition table eviction queue can not be at capacity and empty");
            self.inner.remove(&key);
        }
        self.eviction_queue.push_back(zobrist);
        self.inner.insert(zobrist, score)
    }

    fn remove(&mut self, zobrist: Zobrist) -> Option<Score> {
        self.eviction_queue.retain(|&z| z != zobrist);
        self.inner.remove(&zobrist)
    }

    fn query(&mut self, zobrist: Zobrist) -> Option<Score> {
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
