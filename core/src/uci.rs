//! UCI Protocol Reference: https://backscattering.de/chess/uci/

use std::io;

use anyhow::bail;

pub fn parse_command(line: String) -> anyhow::Result<UciCommand> {
    let mut parts = line.split_ascii_whitespace();
    Ok(match parts.next() {
        Some("uci") => UciCommand::Uci,
        Some("debug") => match parts.next() {
            Some("on") => UciCommand::Debug(true),
            Some("off") => UciCommand::Debug(false),
            _ => bail!("failed to parse debug command"),
        },
        Some("isready") => UciCommand::IsReady,
        Some("setoption") => todo!("set option"),
        Some("register") => todo!("register"),
        Some("ucinewgame") => UciCommand::UciNewGame,
        Some("position") => todo!("position"),
        Some("go") => todo!("go"),
        Some("stop") => UciCommand::Stop,
        Some("ponderhit") => UciCommand::PonderHit,
        Some("quit") => UciCommand::Quit,
        Some(s) => bail!("unable to parse uci command: {}", s),
        None => bail!("missing uci command to parse"),
    })
}

pub fn output_command(cmd: UciCommand) {
    let msg = match cmd {
        UciCommand::Uci => "uci".to_string(),
        UciCommand::Debug(check) => match check {
            true => "debug on".to_string(),
            false => "debug off".to_string(),
        },
        UciCommand::IsReady => "isready".to_string(),
        UciCommand::SetOption { name, value } => match value {
            Some(v) => format!("setoption name {} value {}", name, v),
            None => format!("setoption name {}", name),
        },
        UciCommand::Register(reg_cmd) => todo!("register"),
        UciCommand::UciNewGame => "ucinewgame".to_string(),
        UciCommand::Position(pos_cmd) => todo!("position"),
        UciCommand::Go(go_cmd) => todo!("go"),
        UciCommand::Stop => "stop".to_string(),
        UciCommand::PonderHit => "ponderhit".to_string(),
        UciCommand::Quit => "quit".to_string(),
    };
    println!("{}", msg);
}

pub fn parse_event(line: String) -> anyhow::Result<UciEvent> {
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
        Some("option") => todo!("option"),
        Some(s) => bail!("unable to parse uci event: {}", s),
        None => bail!("missing uci event to parse"),
    })
}

pub fn output_event(out: &mut impl io::Write, event: UciEvent) -> io::Result<()> {
    let msg = match event {
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
        UciEvent::Option(opt_event) => todo!("option"),
    };
    Ok(writeln!(out, "{}", msg)?)
}

#[derive(Debug, PartialEq)]
pub enum UciCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption { name: String, value: Option<String>, },
    Register(RegisterCommand),
    UciNewGame,
    Position(PositionCommand),
    Go(GoCommand),
    Stop,
    PonderHit,
    Quit,
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
    Hash(Spin),
    NalimovPath(String),
    NalimovCache(Spin),
    Ponder(bool),
    OwnBook(bool),
    MultiPV(Spin),
    UciShowCurrLine(bool),
    UciShowRefutations(bool),
    UciLimitStrength(bool),
    UciElo(Spin),
    UciAnalyseMode(bool),
    UciOpponent {
        title: Option<String>,
        elo: Option<usize>,
        name: String,
    },
    UciEngineAbout(String), 
}

#[derive(Debug)]
pub struct Spin {
    pub default: usize,
    pub min: usize,
    pub max: usize,
}
