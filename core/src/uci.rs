//! UCI Protocol Reference: https://backscattering.de/chess/uci/

use std::{fmt::Display, str::SplitAsciiWhitespace};

use anyhow::{bail, Context};

use crate::{board::fen::NUM_FEN_FIELDS, moves};

#[derive(Debug, PartialEq)]
pub enum UciCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(UciOption),
    Register(RegisterCommand),
    UciNewGame,
    Position(PositionCommand),
    Go(GoCommand),
    Stop,
    PonderHit,
    Quit,
}

impl UciCommand {
    pub fn parse(line: String) -> anyhow::Result<Self> {
        let mut parts = line.split_ascii_whitespace();
        Ok(match parts.next() {
            Some("uci") => UciCommand::Uci,
            Some("debug") => match parts.next() {
                Some("on") => UciCommand::Debug(true),
                Some("off") => UciCommand::Debug(false),
                _ => bail!("failed to parse debug command"),
            },
            Some("isready") => UciCommand::IsReady,
            Some("setoption") => {
                if parts.next() != Some("name") {
                    bail!("malformed setoption: {}", line);
                }
                let name = parts.next().context("missing setoption name")?;
                let value = match parts.next() {
                    Some("value") => Some(parts.next().context("missing setoption value")?),
                    None => None,
                    Some(_) => bail!("malformed setoption: {}", line),
                };
                match (name, value) {
                    ("Threads", Some(v)) => UciCommand::SetOption(UciOption::Threads(v.parse::<usize>().context("failed to parse setoption threads")?)),
                    ("Hash", Some(v)) => UciCommand::SetOption(UciOption::Hash(v.parse::<usize>().context("failed to parse setoption hash")?)),
                    _ => bail!("malformed setoption: {}", line), 
                }
            },
            Some("register") => todo!("register"),
            Some("ucinewgame") => UciCommand::UciNewGame,
            Some("position") => match parts.next() {
                Some("startpos") => {
                    let mut moves = None;
                    if parts.next() == Some("moves") {
                        moves = Some(parts.map(|p| p.to_string()).collect())
                    }
                    UciCommand::Position(PositionCommand::StartPos { moves })
                },
                Some("fen") => {
                    let fen = parts.clone().take(NUM_FEN_FIELDS).collect::<Vec<&str>>().join(" ");
                    let mut parts = parts.skip(NUM_FEN_FIELDS);
                    let mut moves: Option<Vec<String>> = None;
                    if parts.next() == Some("moves") {
                        moves = Some(parts.map(|p| p.to_string()).collect());
                    }
                    UciCommand::Position(PositionCommand::Fen { fen, moves })
                },
                _ => bail!("malformed position: {}", line),
            },
            Some("go") => match parts.next() {
                Some("searchmoves") => todo!(),
                Some("ponder") => UciCommand::Go(GoCommand::Ponder),
                Some("wtime") => todo!(),
                Some("btime") => todo!(),
                Some("winc") => todo!(),
                Some("binc") => todo!(),
                Some("movestogo") => todo!(),
                Some("depth") => UciCommand::Go(GoCommand::Depth(parts.next().context("missing go depth value")?.parse::<usize>()?)),
                Some("nodes") => UciCommand::Go(GoCommand::Nodes(parts.next().context("missing go nodes value")?.parse::<usize>()?)),
                Some("mate") => todo!(),
                Some("movetime") => UciCommand::Go(GoCommand::MoveTime(parts.next().context("missing go movetime value")?.parse::<usize>()?)),
                Some("infinite") => UciCommand::Go(GoCommand::Infinite),
                _ => bail!("malformed go command: {}", line),
            },
            Some("stop") => UciCommand::Stop,
            Some("ponderhit") => UciCommand::PonderHit,
            Some("quit") => UciCommand::Quit,
            Some(s) => bail!("unable to parse uci command: {}", s),
            None => bail!("missing uci command to parse"),
        })
    }

    pub fn write(&self) {
        let msg = match self {
            UciCommand::Uci => "uci".to_string(),
            UciCommand::Debug(check) => match check {
                true => "debug on".to_string(),
                false => "debug off".to_string(),
            },
            UciCommand::IsReady => "isready".to_string(),
            UciCommand::SetOption(opts) => match opts {
                UciOption::Threads(t) => format!("setoption name Threads value {t}"),
                UciOption::Hash(h) => format!("setoption name Hash value {h}"),
                UciOption::Ponder(b) => format!("setoption name Ponder value {b}"),
                UciOption::MultiPV(n) => format!("setoption name MultiPV value {n}"),
                UciOption::UciAnalyseMode(b) => format!("setoption name UciAnalyseMode value {b}"),
            },
            UciCommand::Register(reg_cmd) => todo!(),
            UciCommand::UciNewGame => "ucinewgame".to_string(),
            UciCommand::Position(pos_cmd) => match pos_cmd {
                PositionCommand::StartPos { moves } => match moves {
                    Some(mv) => format!("position startpos moves {}", mv.join(" ")),
                    None => "position startpos".to_string(),
                },
                PositionCommand::Fen { fen, moves } => match moves {
                    Some(mv) => format!("position fen {} moves {}", fen, mv.join(" ")),
                    None => format!("position fen {}", fen),
                },
            },
            UciCommand::Go(go_cmd) => match go_cmd {
                GoCommand::SearchMoves(s) => format!("go searchmoves {}", s.join(" ")),
                GoCommand::Ponder => "go ponder".to_string(),
                GoCommand::WTime(t) => format!("go wtime {t}"),
                GoCommand::BTime(t) => format!("go btime {t}"),
                GoCommand::WInc(i) => format!("go winc {i}"),
                GoCommand::BInc(i) => format!("go binc {i}"),
                GoCommand::MovesToGo(n) => format!("go movestogo {n}"),
                GoCommand::Depth(n) => format!("go depth {n}"),
                GoCommand::Nodes(n) => format!("go nodes {n}"),
                GoCommand::Mate(n) => format!("go mate {n}"),
                GoCommand::MoveTime(t) => format!("go movetime {t}"),
                GoCommand::Infinite => "go infinite".to_string(),
            },
            UciCommand::Stop => "stop".to_string(),
            UciCommand::PonderHit => "ponderhit".to_string(),
            UciCommand::Quit => "quit".to_string(),
        };
        println!("{}", msg)
    }
}

#[derive(Debug, PartialEq)]
pub enum UciOption {
    Threads(usize),
    Hash(usize),
    Ponder(bool),
    MultiPV(usize),
    UciAnalyseMode(bool),
}

#[derive(Debug, PartialEq)]
pub enum RegisterCommand {
    Later,
    Name(String),
    Code(String),
}

#[derive(Debug, PartialEq)]
pub enum PositionCommand {
    StartPos { moves: Option<Vec<String>> },
    Fen {
        fen: String,
        moves: Option<Vec<String>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum GoCommand {
    SearchMoves(Vec<String>),
    Ponder,
    WTime(usize),
    BTime(usize),
    WInc(usize),
    BInc(usize),
    MovesToGo(usize),
    Depth(usize),
    Nodes(usize),
    Mate(usize),
    MoveTime(usize),
    Infinite,
}

#[derive(Debug)]
pub enum UciEvent {
    Id(IdEvent),
    UciOk,
    ReadyOk,
    BestMove {
        best: String,
        ponder: Option<String>,
    },
    CopyProtection,
    Registration,
    Info(InfoEvent),
    Option(OptionEvent),
}

impl UciEvent {
    pub fn parse(line: String) -> anyhow::Result<Self> {
        let mut parts = line.split_ascii_whitespace();
        Ok(match parts.next() {
            Some("id") => match parts.next() {
                Some("name") => match parts.next() {
                    Some(name) => UciEvent::Id(IdEvent::Name(name.to_string())),
                    None => bail!("missing id name value"),
                },
                Some("author") => match parts.next() {
                    Some(author) => UciEvent::Id(IdEvent::Author(author.to_string())),
                    None => bail!("missing id author value"),
                },
                Some(s) => bail!("invalid id argument: {}", s),
                None => bail!("missing id argument"),
            },
            Some("uciok") => UciEvent::UciOk,
            Some("readyok") => UciEvent::ReadyOk,
            Some("bestmove") => todo!("bestmove"),
            Some("copyprotection") => todo!("copyprotection"),
            Some("registration") => todo!("registration"),
            Some("info") => todo!("info"),
            Some("option") => {
                if parts.next() != Some("name") {
                    bail!("malformed option name: {}", line);
                }
                let name = parts.next().context("missing option name")?;
                if parts.next() != Some("type") {
                    bail!("malformed option type: {}", line);
                }
                let ty = parts.next().context("missing option type")?;
                match (name, ty) {
                    ("Threads", "spin") => UciEvent::Option(OptionEvent::Threads(Spin::parse(&mut parts)?)),
                    ("Hash", "spin") => UciEvent::Option(OptionEvent::Hash(Spin::parse(&mut parts)?)),
                    ("Ponder", "check") => UciEvent::Option(OptionEvent::Ponder(Check::parse(&mut parts)?)),
                    ("MultiPV", "spin") => UciEvent::Option(OptionEvent::MultiPV(Spin::parse(&mut parts)?)),
                    ("UciAnalyseMode", "check") => UciEvent::Option(OptionEvent::UciAnalyseMode(Check::parse(&mut parts)?)),
                    _ => bail!("malformed option: {}", line),
                }
            },
            Some(s) => bail!("unable to parse uci event: {}", s),
            None => bail!("missing uci event to parse"),
        })
    }

    pub fn write(&self) {
         let msg = match self {
            UciEvent::Id(id_event) => match id_event {
                IdEvent::Name(name) => format!("id name {}", name),
                IdEvent::Author(author) => format!("id author {}", author),
            },
            UciEvent::UciOk => "uciok".to_string(),
            UciEvent::ReadyOk => "readyok".to_string(),
            UciEvent::BestMove { best, ponder } => todo!("bestmove"),
            UciEvent::CopyProtection => todo!("copyprotection"),
            UciEvent::Registration => todo!("registration"),
            UciEvent::Info(info_event) => match info_event {
                InfoEvent::String(s) => format!("info string {}", s),
                _ => todo!(),
            },
            UciEvent::Option(opt_event) => match opt_event {
                OptionEvent::Threads(t) => format!("option name Threads type spin {t}"),
                OptionEvent::Hash(h) => format!("option name Hash type spin {h}"),
                OptionEvent::Ponder(p) => format!("option name Ponder type check {p}"),
                OptionEvent::MultiPV(m) => format!("option name MultiPV type spin {m}"),
                OptionEvent::UciAnalyseMode(a) => format!("option name UciAnalyseMode type check {a}"),
            },
        };
        println!("{}", msg)
    }
}

#[derive(Debug)]
pub enum IdEvent {
    Name(String),
    Author(String),
}

#[derive(Debug)]
pub enum InfoEvent {
    Depth(usize),
    SelDepth(usize),
    Time(usize),
    Nodes(usize),
    PV(Vec<String>),
    MultiPV(usize),
    ScoreCp(usize),
    ScoreMate(usize),
    ScoreLowerBound,
    ScoreUpperBound,
    CurrMove(String),
    CurrMoveNumber(usize),
    HashFull(usize),
    Nps(usize),
    TbHits(usize),
    SbHits(usize),
    CpuLoad(usize),
    String(String),
    Refutaion(Vec<String>),
    CurrLine {
        cpu_num: usize,
        moves: Vec<String>,
    }
}

#[derive(Debug)]
pub enum OptionEvent {
    Threads(Spin),
    Hash(Spin),
    Ponder(Check),
    MultiPV(Spin),
    UciAnalyseMode(Check),
}

#[derive(Debug)]
pub struct Spin {
    pub default: usize,
    pub min: usize,
    pub max: usize,
}

impl Spin {
    fn parse(parts: &mut SplitAsciiWhitespace) -> anyhow::Result<Self> {
        if parts.next() != Some("default") {
            bail!("malformed spin default");
        }
        let default = parts.next().context("missing spin default")?.parse::<usize>().context("failed to parse spin default")?;
        if parts.next() != Some("min") {
            bail!("malformed spin min");
        }
        let min = parts.next().context("missing spin min")?.parse::<usize>().context("failed to parse spin min")?;
        if parts.next() != Some("max") {
            bail!("malformed spin max");
        }
        let max = parts.next().context("missing spin max")?.parse::<usize>().context("failed to parse spin max")?;
        Ok(Spin { default, min , max })
    }
}

impl Display for Spin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "default {} min {} max {}", self.default, self.min, self.max)
    }
}

#[derive(Debug)]
pub struct Check {
    pub default: bool,
}

impl Check {
    fn parse(parts: &mut SplitAsciiWhitespace) -> anyhow::Result<Self> {
        if parts.next() != Some("default") {
            bail!("malformed check default");
        }
        Ok(Check { default: parts.next().context("missing check default")?.parse::<bool>().context("failed to parse check")? })
    }
}

impl Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "default {}", self.default)
    }
}
