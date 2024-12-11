use std::sync::{Arc, RwLock};

use chess_core::{board::Board, game::GameState, moves::ShortMove, uci::{Check, IdEvent, OptionEvent, Spin, UciEvent}};

use crate::search::{infinite::InfiniteSearch, tree::SearchTree, Search};

use super::{depth::DepthSearch, mate::MateSearch, node::NodeSearch, tt::TranspositonTable};

const ENGINE_NAME: &str = "MonteCristo";
const ENGINE_AUTHOR: &str = "mosesprime";

const DEFAULT_NUM_THREADS: usize = 1;
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

pub struct Engine {
    /// Current game state.
    game_state: GameState,
    /// Search space in question.
    search_tree: SearchTree,
    /// The search currently being run.
    searching: Option<Box<dyn Search>>,
    /// Store of previously evaluated board states.
    transpositions: Arc<RwLock<TranspositonTable>>,
    /// 
    pondering: Option<ShortMove>,
    /// 
    pub config: Config,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(Board::default(), Config::default())
    }
}

impl Engine {
    pub fn new(start: Board, config: Config) -> Self {
        let game_state = GameState::new(start);
        let search_tree = SearchTree::new(game_state.board());
        Self {
            game_state,
            search_tree: search_tree.into(),
            searching: None,
            transpositions: Arc::new(RwLock::new(TranspositonTable::new(config.hash_capacity))),
            pondering: None,
            config,
        }
    }

    /// Stop the current search and report best move.
    pub fn stop(&mut self) {
        if let Some(s) = &mut self.searching {
            let best = s.stop();
            let ponder = todo!();
            // TODO: println!("{}", UciEvent::BestMove { best, ponder })
        }
        // TODO: else?
    } 

    pub fn quit(self) { todo!() }

    pub fn reset(&mut self, game_state: GameState) {
        if let Some(s) = &mut self.searching {
            let _ = s.stop();
            self.searching = None;
        }
        self.pondering = None;
        self.search_tree = SearchTree::new(game_state.board()).into();
        self.game_state = game_state;
    }

    pub fn report_about(&self) {
        println!("{}", UciEvent::Id(IdEvent::Name(ENGINE_NAME.to_string())));
        println!("{}", UciEvent::Id(IdEvent::Author(ENGINE_AUTHOR.to_string())));
        // TODO: report non-hard codes defaults
        println!("{}", UciEvent::Option(OptionEvent::Threads(Spin { default: 1, min: 1, max: 32 })));
        println!("{}", UciEvent::Option(OptionEvent::Hash(Spin { default: 16, min: 1, max: 16_000 })));
        println!("{}", UciEvent::Option(OptionEvent::Ponder(Check { default: DEFAULT_CAN_PONDER })));
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

    pub fn search_infinite(&mut self) {
        if let Some(s) = &mut self.searching {
            s.stop();
            self.searching = None;
        }
        let mut search = InfiniteSearch::new(self.config.num_threads);
        search.start(&mut self.search_tree, self.transpositions.clone());
        self.searching = Some(Box::new(search));
    }

    pub fn search_depth(&mut self, depth: usize) {
        if let Some(s) = &mut self.searching {
            s.stop();
            self.searching = None;
        }
        let mut search = DepthSearch::new(self.config.num_threads, depth);
        search.start(&mut self.search_tree, self.transpositions.clone());
        self.searching = Some(Box::new(search));
    }

    pub fn search_nodes(&mut self, num_nodes: usize) {
        if let Some(s) = &mut self.searching {
            s.stop();
            self.searching = None;
        }
        let mut search = NodeSearch::new(self.config.num_threads, num_nodes);
        search.start(&mut self.search_tree, self.transpositions.clone());
        self.searching = Some(Box::new(search));
    }

    pub fn search_mate(&mut self, mate_in: usize) {
        if let Some(s) = &mut self.searching {
            s.stop();
            self.searching = None;
        }
        let mut search = MateSearch::new(self.config.num_threads, mate_in);
        search.start(&mut self.search_tree, self.transpositions.clone());
        self.searching = Some(Box::new(search));
    }
}

