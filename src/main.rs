use chess::{board::Board, uci::{UciCommand, UciEvent, UciOption}};
use engine::{Engine, EngineConfig};
mod engine;

fn main() {
    let config = EngineConfig::default();
    let mut engine = Engine::new(config);
    let stdin = std::io::stdin();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).expect("failed to read stdin");
        match UciCommand::parse(&buf) {
            Ok(UciCommand::Uci) => {
                engine.report_about();
                println!("{}", UciEvent::UciOk);
            },
            Ok(UciCommand::Debug(b)) => engine.config.enable_debug = b,
            Ok(UciCommand::IsReady) => {
                todo!("uci isready");
                println!("{}", UciEvent::ReadyOk);
            },
            Ok(UciCommand::SetOption(opt)) => match opt {
                UciOption::Threads(n) => engine.config.num_threads = n,
                UciOption::Hash(n) => engine.config.hash_capacity = n,
                UciOption::Ponder(b) => engine.config.enable_ponder = b,
                UciOption::MultiPV(n) => engine.config.multi_pv = n,
                UciOption::UciAnalyseMode(b) => engine.config.enable_analyse = b,
            },
            Ok(UciCommand::Register(reg)) => todo!(), // TODO: idk if this is needed
            Ok(UciCommand::UciNewGame) => engine.reset(Board::default()),
            Ok(UciCommand::Position(pos)) => todo!(),
            Ok(UciCommand::Go(go)) => todo!(),
            Ok(UciCommand::Stop) => {
                let (best, ponder) = engine.stop();
                println!("{}", UciEvent::BestMove { best, ponder });
            },
            Ok(UciCommand::PonderHit) => todo!(),
            Ok(UciCommand::Quit) => {
                let _ = engine.stop();
                break;
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
