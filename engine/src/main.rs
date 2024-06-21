use std::io;

use anyhow::Context;
use clap::{Parser, Subcommand};

fn main() {
    let cli_args = CliArgs::parse();
    match &cli_args.command {
        Some(Command::Uci) => run_repl().unwrap(),
        None => {},
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct CliArgs {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// runs the bot via the UCI protocol
    Uci,
}

fn run_repl() -> anyhow::Result<()> {
    let stdin = io::stdin();
    // let stdout = io::stdout();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).context("failed to read line")?;
        let cmd = chess_core::uci::parse_command(buf)?;
        println!("commmand: {:?}", cmd);
    }
}


