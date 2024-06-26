use std::thread::JoinHandle;

use crossbeam::channel::{Receiver, Sender};

pub struct SearchManager {
    handles: Vec<JoinHandle<()>>,
    command_tx: Sender<SearchCommand>,
    event_rx: Receiver<SearchEvent>,
}

impl SearchManager {
    pub fn new(threads: usize) -> Self {
        let mut handles = vec![];
        let (command_tx, command_rx) = crossbeam::channel::unbounded();
        let (event_tx, event_rx) = crossbeam::channel::unbounded();
        for _ in 0..threads {
            let rx = command_rx.clone();
            let tx = event_tx.clone();
            let h = std::thread::spawn(move || {
                loop {
                    match rx.recv().expect("failed to recv search command") {
                        SearchCommand::Start { mode, debug } => todo!(),
                        SearchCommand::Stop => {
                            todo!();
                            continue;
                        },
                        SearchCommand::Quit => break,
                    }
                }
            });
            handles.push(h);
        }
        Self {
            handles,
            command_tx,
            event_rx,
        }
    }
}

enum SearchCommand {
    Start {
        mode: SearchMode,
        /// give intermediate search stats
        debug: bool,
    },
    /// suspend the current iteration of search
    Stop,
    /// quit searching and shutdown gracefully
    Quit,
}

enum SearchEvent {
    Info(SearchInfo),
}

enum SearchMode {
    /// search down a certain depth
    Depth,
    /// search for a certain amount of time
    Time,
    /// search a certain number of nodes
    Nodes,
    /// search until stopped
    Infinite,
}

struct SearchInfo {}
