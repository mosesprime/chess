use anyhow::Context;
use chess_core::{board::Board, game::{GameHistory, GameState}, moves::ShortMove, uci::{GoCommand, PositionCommand, UciCommand, UciEvent, UciOption}};
use search::{Config, Engine};

pub mod evaluate;
pub mod search;

pub fn run_uci_repl() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut engine = Engine::new(Board::default(), Config::default());
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).context("failed to read line")?;
        let cmd = UciCommand::parse(buf)?;
        match cmd {
            UciCommand::Uci => {
                engine.report_about();
                UciEvent::UciOk.write();
            },
            UciCommand::Debug(true) => println!("info string debug enabled"),
            UciCommand::Debug(false) => println!("info string debug disabled"),
            UciCommand::IsReady => {
                // TODO: ensure readyok
                UciEvent::ReadyOk.write();
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
            UciCommand::Go(go_cmd) => match go_cmd {
                GoCommand::Infinite => engine.search_infinite(),
                GoCommand::Depth(depth) => engine.search_depth(depth),
                GoCommand::Ponder => engine.search_infinite(),
                GoCommand::Nodes(nodes) => engine.search_nodes(nodes),
                GoCommand::SearchMoves(v) => todo!(),
                GoCommand::WTime(t) => todo!(),
                GoCommand::BTime(t) => todo!(),
                GoCommand::WInc(n) => todo!(),
                GoCommand::BInc(n) => todo!(),
                GoCommand::MovesToGo(n) => todo!(),
                GoCommand::Mate(n) => todo!(),
                GoCommand::MoveTime(t) => todo!(),
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
