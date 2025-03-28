use chess_core::board::Board;
use crate::uci::{IdEvent, OptionEvent, UciEvent};
use crate::search::{SearchTree, Searching};

/// Default size of hash table in MB.
const DEFAULT_HASH_CAPACITY: usize = 512;
const DEFAULT_ENABLE_PONDER: bool = false;
const DEFAULT_NUM_THREADS: usize = 1;
const ENGINE_NAME: &'static str = "MonteCristo";
const ENGINE_AUTHOR: &'static str = "mosesprime";

pub struct EngineConfig {
    pub num_threads: usize,
    max_threads: usize,
    pub hash_capacity: usize,
    pub multi_pv: usize,
    pub enable_debug: bool,
    pub enable_ponder: bool,
    pub enable_analyse: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        let available_threads = std::thread::available_parallelism().expect("failed to get available_parallelism").get();
        assert!(available_threads >= 1, "must have at least one available thread");
        Self { 
            num_threads: DEFAULT_NUM_THREADS,
            max_threads: available_threads,
            hash_capacity: DEFAULT_HASH_CAPACITY,
            multi_pv: 1,
            enable_debug: false,
            enable_ponder: DEFAULT_ENABLE_PONDER,
            enable_analyse: false,
        }
    }
}

pub struct Engine {
    search_tree: SearchTree,
    searching: Option<Searching>,
    pub(crate) config: EngineConfig,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self {
        Self {
            search_tree: SearchTree::new(Board::default()),
            searching: None,
            config,
        }
    }

    pub(crate) fn report_about(&self) {
        println!("{}", UciEvent::Id(IdEvent::Name(ENGINE_NAME.to_string())));
        println!("{}", UciEvent::Id(IdEvent::Author(ENGINE_AUTHOR.to_string())));
        println!("{}", UciEvent::Option(OptionEvent::Threads(crate::uci::Spin { default: DEFAULT_NUM_THREADS, min: 1, max: self.config.max_threads })));
        println!("{}", UciEvent::Option(OptionEvent::Hash(crate::uci::Spin { default: DEFAULT_HASH_CAPACITY, min: 1, max: usize::MAX })));
        println!("{}", UciEvent::Option(OptionEvent::Ponder(crate::uci::Check { default: DEFAULT_ENABLE_PONDER })))
    }

    pub fn reset(&mut self, board: Board) {
        if let Some(search) = &self.searching {
            let _ = search.stop();
            self.searching = None;
        }
        self.search_tree = SearchTree::new(board); // TODO: can any of the tree be saved?
    }

    /// Stops any ongoing search and returns (best move, ponder move).
    pub fn stop(&mut self) -> (String, Option<String>) {
        todo!()
    }
}
