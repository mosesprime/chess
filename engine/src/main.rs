use std::{io::{self, Write}, thread::available_parallelism};

use anyhow::Context;
use chess_core::uci::{output_event, IdEvent, UciCommand, UciEvent};
use clap::Parser;
use cli::{CliArgs, CliCommand};

mod cli;
mod search;

fn main() {
    let cli_args = CliArgs::parse();
    let threads = match cli_args.threads {
        Some(n) => n,
        None => available_parallelism().unwrap().get(),
    };
    let config = Config::new(threads);
    match &cli_args.command {
        Some(CliCommand::Uci) => run_uci_repl(config).unwrap(),
        None => {},
    }
}

fn run_uci_repl(config: Config) -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).context("failed to read line")?;
        let cmd = chess_core::uci::parse_command(buf)?;
        match cmd {
            UciCommand::Uci => {
                output_event(&mut stdout, UciEvent::Id(IdEvent::Name(ENGINE_NAME.to_string())))?;
                writeln!(stdout, "id author {}", ENGINE_AUTHOR)?;
                // TODO: send options that can change
                writeln!(stdout, "uciok")?;
            },
            UciCommand::IsReady => {
                // TODO: ensure ready
                writeln!(stdout, "readyok")?;
            },
            UciCommand::Debug(true) => println!("info string debug enabled"),
            UciCommand::Debug(false) => println!("info string debug disabled"),
            UciCommand::Quit => break,
            _ => todo!(),
        }
    }
    Ok(())
}

const ENGINE_NAME: &str = "MonteCristo";
const ENGINE_AUTHOR: &str = "mosesprime";

#[derive(Debug)]
struct Config {
    threads: usize,
}

impl Config {
    fn new(threads: usize) -> Self {
        let max_threads = available_parallelism().unwrap().get();
        assert!(threads <= max_threads, "cpu thread count configuration excedes availability: ask={} max={}", threads, max_threads);
        Self {
            threads
        }
    }
}
