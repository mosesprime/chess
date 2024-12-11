use anyhow::Context;
use chess_core::{board::Board, game::{GameHistory, GameState}, uci::{GoKind, PositionCommand, UciCommand, UciEvent, UciOption}};
use search::engine::Engine;

mod evaluate;
mod search;

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    run_uci_repl(engine)
}

fn run_uci_repl(mut engine: Engine) -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).context("failed to read line")?;
        match UciCommand::parse(&buf)? {
            UciCommand::Uci => {
                engine.report_about();
                println!("{}", UciEvent::UciOk)
            },
            UciCommand::Debug(true) => println!("info string debug enabled"),
            UciCommand::Debug(false) => println!("info string debug disabled"),
            UciCommand::IsReady => {
                // TODO: ensure readyok
                println!("{}", UciEvent::ReadyOk)
            },
            UciCommand::SetOption(set_opt) => match set_opt {
                UciOption::Threads(n) => engine.config.num_threads = n,
                UciOption::Hash(n) => engine.config.hash_capacity = n,
                UciOption::Ponder(b) => engine.config.can_ponder = b,
                UciOption::MultiPV(n) => todo!(),
                UciOption::UciAnalyseMode(b) => engine.config.analyse_mode = b,
            },
            UciCommand::Register(reg_cmd) => todo!(),
            UciCommand::UciNewGame => engine.reset(GameState::new(Board::default())),
            UciCommand::Position(pos_cmd) => {
                let game_history: GameHistory = match pos_cmd {
                    PositionCommand::StartPos { moves } => todo!(),
                    PositionCommand::Fen { fen, moves } => todo!(),
                };
                todo!("set game state");
            },
            UciCommand::Go(go_cmd) => {
                // TODO: what if already searching
                let params = {
                    for cmd in go_cmd.params {
                        todo!()
                    }
                };
                match go_cmd.kind { 
                    GoKind::Infinite => engine.search_infinite(),
                    GoKind::Depth(d) => engine.search_depth(d),
                    GoKind::Nodes(n) => engine.search_nodes(n),
                    GoKind::Mate(n) => engine.search_mate(n),
                }
            },
            UciCommand::Stop => {
                engine.stop();
                continue;
            },                        
            UciCommand::PonderHit => engine.ponder_hit(),
            UciCommand::Quit => {
                engine.quit();
                break;
            },
        }
    }
    Ok(())
}

