use std::{io::{self, Write}, thread::available_parallelism};

use anyhow::Context;
use chess_core::uci::UciCommmand;
use clap::{Parser, Subcommand};

fn main() {
    let cli_args = CliArgs::parse();
    let threads = match cli_args.threads {
        Some(n) => n,
        None => available_parallelism().unwrap().get(),
    };
    let config = Config::new(threads);
    println!("{:?}", config);
    match &cli_args.command {
        Some(Command::Uci) => run_uci_repl().unwrap(),
        None => {},
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
    /// number of CPU threads to utilize, default = max
    #[arg(short, long)]
    threads: Option<usize>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// runs the bot via the UCI protocol
    Uci,
}

fn run_uci_repl() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).context("failed to read line")?;
        let cmd = chess_core::uci::parse_command(buf)?;
        if cmd == UciCommmand::Debug(true) {
            writeln!(stdout, "info string debug activated").context("failed to write line")?;
        } else {
            println!("command: {:?}", cmd);
        }
    }
}

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
