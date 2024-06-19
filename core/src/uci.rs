//! UCI Protocol Reference: https://backscattering.de/chess/uci/

use std::io;

use anyhow::bail;

pub fn parse_command(line: String) -> anyhow::Result<UciCommmand> {
    let mut parts = line.split_ascii_whitespace();
    Ok(match parts.next() {
        Some("uci") => UciCommmand::Uci,
        Some("debug") => match parts.next() {
            Some("on") => UciCommmand::Debug(true),
            Some("off") => UciCommmand::Debug(false),
            _ => bail!("failed to parse debug command"),
        },
        Some("isready") => UciCommmand::IsReady,
        Some("setoption") => todo!("set option"),
        Some("register") => todo!("register"),
        Some("ucinewgame") => UciCommmand::UciNewGame,
        Some("position") => todo!("position"),
        Some("go") => todo!("go"),
        Some("stop") => UciCommmand::Stop,
        Some("ponderhit") => UciCommmand::PonderHit,
        Some("quit") => UciCommmand::Quit,
        Some(s) => bail!("unable to parse uci command: {}", s),
        None => bail!("missing uci command to parse"),
    })
}

pub fn output_command(out: &mut impl io::Write, cmd: UciCommmand) -> io::Result<()> {
    let msg = match cmd {
        UciCommmand::Uci => "uci",
        UciCommmand::Debug(check) => match check {
            true => "debug on",
            false => "debug off",
        },
        UciCommmand::IsReady => "isready",
        UciCommmand::SetOption { name, value } => match value {
            Some(v) => return Ok(writeln!(out, "setoption name {} value {}", name, v)?),
            None => return Ok(writeln!(out, "setoption name {}", name)?),
        },
        UciCommmand::Register(reg_cmd) => todo!("register"),
        UciCommmand::UciNewGame => "ucinewgame",
        UciCommmand::Position(pos_cmd) => todo!("position"),
        UciCommmand::Go(go_cmd) => todo!("go"),
        UciCommmand::Stop => "stop",
        UciCommmand::PonderHit => "ponderhit",
        UciCommmand::Quit => "quit",
    };
    Ok(writeln!(out, "{}", msg)?)
}

pub fn parse_event(line: String) -> anyhow::Result<UciEvent> {
    let mut parts = line.split_ascii_whitespace();
    Ok(match parts.next() {
        Some("id") => todo!("id"),
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
        UciEvent::Id(id_event) => todo!("id"),
        UciEvent::UciOk => "uciok",
        UciEvent::ReadyOk => "readyok",
        UciEvent::BestMove { best, ponder } => todo!("bestmove"),
        UciEvent::CopyProtection => todo!("copyprotection"),
        UciEvent::Registration => todo!("registration"),
        UciEvent::Info(info_event) => todo!("info"),
        UciEvent::Option(opt_event) => todo!("option"),
    };
    Ok(writeln!(out, "{}", msg)?)
}

#[derive(Debug)]
pub enum UciCommmand {
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

#[derive(Debug)]
pub enum RegisterCommand {
    Later,
    Name(String),
    Code(String),
}

#[derive(Debug)]
pub enum PositionCommand {
    StartPos,
    Fen {
        fen: String,
        moves: Vec<String>,
    },
}

#[derive(Debug)]
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
