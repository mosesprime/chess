use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct CliArgs {
    /// number of CPU threads to utilize, default = max
    #[arg(short, long)]
    pub threads: Option<usize>,

    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    /// runs the bot via the UCI protocol
    Uci,
}
