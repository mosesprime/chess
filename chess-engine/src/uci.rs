//! UCI Protocol Reference: https://backscattering.de/chess/uci/

use std::{fmt::Display, str::SplitAsciiWhitespace};

use chess_core::board::fen::NUM_FEN_FIELDS;

#[derive(Debug)]
pub enum UciError {
    ParseError,
    EmptyEvent,
    EmptyCommand,
    MissingValue,
    InvalidParameter,
    MissingParameter,
    UnknownCommandArg,
    MissingCommandArgs,
    ExcessiveCommandArgs,
}

impl Display for UciError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for UciError {}

/// Inputs to the engine.
#[derive(Debug, PartialEq)]
pub enum UciCommand {
    /// Tell engine to use UCI. Engine must identify itself with [UciEvent::Id] and send [UciEvent::Option]
    /// to inform the GUI what settings the engine supports. Finally the engine should send
    /// [UciEvent::UciOk] to acknoledge UCI mode.
    Uci,
    /// Enable/disable debug info being sent to the GUI via [UciEvent::Info].
    Debug(bool),
    /// Syncronize the engine and GUI. Engine should respond with [UciEvent::ReadyOk].
    IsReady,
    /// Change an engine setting.
    SetOption(UciOption),
    ///
    Register(RegisterCommand),
    /// Engine should prepare to evaluate a different game than the current one.
    UciNewGame,
    /// Set the engine to evaluate the given position. Could be a ply down from the currently held
    /// position, or an entirely different game. In the later case, a "ucinewgame" is prefered but
    /// not always given by the GUI.
    Position(PositionCommand),
    /// Start calculating on the current position.
    Go(GoCommand),
    /// Stop current search ASAP. Engine should send "bestmove" and "ponder" if able.
    Stop,
    /// User has played the expected "ponder" move. Continue ponder search as a normal search.
    PonderHit,
    /// Terminate the engine ASAP.
    Quit,
}

impl Display for UciCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cmd = match self {
            UciCommand::Uci => "uci".to_string(),
            UciCommand::Debug(b) => format!("debug {b}"),
            UciCommand::IsReady => "isready".to_string(),
            UciCommand::SetOption(opts) => match opts {
                UciOption::Threads(t) => format!("setoption name Threads value {t}"),
                UciOption::Hash(h) => format!("setoption name Hash value {h}"),
                UciOption::Ponder(b) => format!("setoption name Ponder value {b}"),
                UciOption::MultiPV(n) => format!("setoption name MultiPV value {n}"),
                UciOption::UciAnalyseMode(b) => format!("setoption name UciAnalyseMode value {b}"),
            },
            UciCommand::Register(_) => todo!(),
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
            UciCommand::Go(go_cmd) => format!("go {go_cmd}"),
            UciCommand::Stop => "stop".to_string(),
            UciCommand::PonderHit => "ponderhit".to_string(),
            UciCommand::Quit => "quit".to_string(),
        };
        write!(f, "{}", cmd)
    }
}

impl UciCommand {
    pub fn parse(s: &str) -> Result<Self, UciError> {
        let mut parts = s.split_ascii_whitespace();
        Ok(match parts.next() {
            Some("uci") => UciCommand::Uci,
            Some("debug") => UciCommand::Debug(parts.next().ok_or(UciError::MissingValue)?.parse().map_err(|_| UciError::ParseError)?),
            Some("isready") => UciCommand::IsReady,
            Some("setoption") => {
                match parts.next() {
                    Some("name") => {},
                    Some(_) => return Err(UciError::InvalidParameter),
                    None => return Err(UciError::MissingParameter),
                }
                let name = parts.next().ok_or(UciError::MissingParameter)?;
                let value = match parts.next() {
                    Some("value") => Some(parts.next().ok_or(UciError::MissingValue)?),
                    Some(_) => return Err(UciError::InvalidParameter),
                    None => None,
                };
                match (name, value) {
                    ("Threads", Some(v)) => UciCommand::SetOption(UciOption::Threads(v.parse::<usize>().map_err(|_| UciError::ParseError)?)),
                    ("Hash", Some(v)) => UciCommand::SetOption(UciOption::Hash(v.parse::<usize>().map_err(|_| UciError::ParseError)?)),
                    _ => return Err(UciError::InvalidParameter), 
                }
            },
            Some("register") => todo!("register"), // TODO: uci command register
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
                Some(_) => return Err(UciError::InvalidParameter),
                None => return Err(UciError::MissingParameter),
            },
            Some("go") => UciCommand::Go(GoCommand::parse(&mut parts)?),
            Some("stop") => UciCommand::Stop,
            Some("ponderhit") => UciCommand::PonderHit,
            Some("quit") => UciCommand::Quit,
            Some(_) => return Err(UciError::InvalidParameter),
            None => return Err(UciError::EmptyCommand),
        })

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
pub struct GoCommand {
    pub kind: GoKind,
    pub params: Vec<GoParam>,
}

impl Display for GoCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)?;
        for param in self.params.iter() {
            write!(f, " {}", param)?;
        }
        Ok(())
    }
}

impl GoCommand {
    fn parse(parts: &mut SplitAsciiWhitespace) -> Result<Self, UciError> {
        let kind = match parts.next().ok_or(UciError::MissingCommandArgs)? {
            "infinite" => GoKind::Infinite,
            "depth" => GoKind::Depth(parts.next().ok_or(UciError::MissingParameter)?.parse().map_err(|_| UciError::ParseError)?),
            "nodes" => GoKind::Nodes(parts.next().ok_or(UciError::MissingParameter)?.parse().map_err(|_| UciError::ParseError)?),
            "mate" => GoKind::Mate(parts.next().ok_or(UciError::MissingParameter)?.parse().map_err(|_| UciError::ParseError)?),
            _ => return Err(UciError::UnknownCommandArg),
        };
        let mut params = vec![];
        while let Some(part) = parts.next() {
            let param = match part {
                "ponder" => GoParam::Ponder,
                "wtime" => GoParam::WTime(parts.next().ok_or(UciError::MissingParameter)?.parse().map_err(|_| UciError::ParseError)?),
                "btime" => GoParam::BTime(parts.next().ok_or(UciError::MissingParameter)?.parse().map_err(|_| UciError::ParseError)?),
                "winc" => GoParam::WInc(parts.next().ok_or(UciError::MissingParameter)?.parse().map_err(|_| UciError::ParseError)?),
                "binc" => GoParam::BInc(parts.next().ok_or(UciError::MissingParameter)?.parse().map_err(|_| UciError::ParseError)?),
                "movestogo" => GoParam::MovesToGo(parts.next().ok_or(UciError::MissingParameter)?.parse().map_err(|_| UciError::ParseError)?),
                "movetime" => GoParam::MoveTime(parts.next().ok_or(UciError::MissingParameter)?.parse().map_err(|_| UciError::ParseError)?),
                "searchmoves" => todo!(), //GoParam::SearchMoves(parts.next().ok_or(UciError::MissingParameter)?.parse().map_err(|_| UciError::ParseError)?),
                _ => return Err(UciError::UnknownCommandArg),
            };
            params.push(param);
        }
        Ok(GoCommand { kind, params })
    }
}

#[derive(Debug, PartialEq)]
pub enum GoKind {
    Depth(usize),
    Nodes(usize),
    Mate(usize),
    Infinite,
}

impl Display for GoKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoKind::Depth(n) => write!(f, "depth {n}"),
            GoKind::Nodes(n) => write!(f, "nodes {n}"),
            GoKind::Mate(n) => write!(f, "mate {n}"),
            GoKind::Infinite => write!(f, "infinite"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GoParam {
    SearchMoves(Vec<String>),
    WTime(usize),
    BTime(usize),
    WInc(usize),
    BInc(usize),
    MovesToGo(usize),
    MoveTime(usize),
    Ponder,
}

impl Display for GoParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoParam::SearchMoves(m) => write!(f, "searchmoves {}", m.join(" ")),
            GoParam::WTime(n) => write!(f, "wtime {n}"),
            GoParam::BTime(n) => write!(f, "btime {n}"),
            GoParam::WInc(n) => write!(f, "winc {n}"),
            GoParam::BInc(n) => write!(f, "binc {n}"),
            GoParam::MovesToGo(n) => write!(f, "movestogo {n}"),
            GoParam::MoveTime(n) => write!(f, "movetime {n}"),
            GoParam::Ponder => write!(f, "ponder"),
        }
    }
}

/// Outputs from the engine.
#[derive(Debug)]
pub enum UciEvent {
    /// Sent to the GUI after receiving a [UciCommand::Uci] to self-identify the engine.
    Id(IdEvent),
    /// Acknoledge [UciCommand::Uci].
    UciOk,
    /// Acknoledge [UciCommand::IsReady] as soon as engine is ready to accept new commands.
    ReadyOk,
    /// Report the results of a search. Does not start pondering automatically.
    BestMove {
        /// Most advantageous move to make.
        best: String,
        /// Next move that the engine would like to continue working on.
        ponder: Option<String>,
    },
    /// 
    CopyProtection,
    ///
    Registration,
    /// Report some information to the GUI.
    Info(InfoEvent),
    /// Inform the GUI of what engine settings can be changed.
    Option(OptionEvent),
}

impl Display for UciEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            UciEvent::Id(id_event) => match id_event {
                IdEvent::Name(name) => format!("id name {}", name),
                IdEvent::Author(author) => format!("id author {}", author),
            },
            UciEvent::UciOk => "uciok".to_string(),
            UciEvent::ReadyOk => "readyok".to_string(),
            UciEvent::BestMove { best, ponder } => match ponder {
                None => format!("bestmove {best}"),
                Some(ponder) => format!("bestmove {best} ponder {ponder}")
            },
            UciEvent::CopyProtection => todo!("copyprotection"), // TODO: uci event copyprotection
            UciEvent::Registration => todo!("registration"), // TODO: uci event registration
            UciEvent::Info(info_event) => format!("info {info_event}"),
            UciEvent::Option(opt_event) => match opt_event {
                OptionEvent::Threads(t) => format!("option name Threads type spin {t}"),
                OptionEvent::Hash(h) => format!("option name Hash type spin {h}"),
                OptionEvent::Ponder(p) => format!("option name Ponder type check {p}"),
                OptionEvent::MultiPV(m) => format!("option name MultiPV type spin {m}"),
                OptionEvent::UciAnalyseMode(a) => format!("option name UciAnalyseMode type check {a}"),
            },
        };
        write!(f, "{}", msg)
    }
}

impl UciEvent {
    pub fn parse(line: String) -> Result<Self, UciError> {
        let mut parts = line.split_ascii_whitespace();
        Ok(match parts.next() {
            Some("id") => match parts.next() {
                Some("name") => match parts.next() {
                    Some(name) => UciEvent::Id(IdEvent::Name(name.to_string())),
                    None => return Err(UciError::MissingValue),
                },
                Some("author") => match parts.next() {
                    Some(author) => UciEvent::Id(IdEvent::Author(author.to_string())),
                    None => return Err(UciError::MissingValue),
                },
                Some(_) => return Err(UciError::InvalidParameter),
                None => return Err(UciError::MissingParameter),
            },
            Some("uciok") => UciEvent::UciOk,
            Some("readyok") => UciEvent::ReadyOk,
            Some("bestmove") => todo!("bestmove"), // TODO: uci event bestmove
            Some("copyprotection") => todo!("copyprotection"), // TODO: uci event copyprotection
            Some("registration") => todo!("registration"), // TODO: uci event registration
            Some("info") => todo!("info"), // TODO: uci event info
            Some("option") => {
                match parts.next() {
                    Some("name") => {},
                    Some(_) => return Err(UciError::InvalidParameter),
                    None => return Err(UciError::MissingParameter),
                }
                let name = parts.next().ok_or(UciError::MissingValue)?;
                match parts.next() {
                    Some("type") => {},
                    Some(_) => return Err(UciError::InvalidParameter),
                    None => return Err(UciError::MissingParameter),
                }
                let ty = parts.next().ok_or(UciError::MissingValue)?;
                match (name, ty) {
                    ("Threads", "spin") => UciEvent::Option(OptionEvent::Threads(Spin::parse(&mut parts)?)),
                    ("Hash", "spin") => UciEvent::Option(OptionEvent::Hash(Spin::parse(&mut parts)?)),
                    ("Ponder", "check") => UciEvent::Option(OptionEvent::Ponder(Check::parse(&mut parts)?)),
                    ("MultiPV", "spin") => UciEvent::Option(OptionEvent::MultiPV(Spin::parse(&mut parts)?)),
                    ("UciAnalyseMode", "check") => UciEvent::Option(OptionEvent::UciAnalyseMode(Check::parse(&mut parts)?)),
                    _ => return Err(UciError::InvalidParameter),
                }
            },
            Some(_) => return Err(UciError::InvalidParameter),
            None => return Err(UciError::EmptyEvent),
        })
    }
}

#[derive(Debug)]
pub enum IdEvent {
    Name(String),
    Author(String),
}

#[derive(Debug)]
pub struct InfoEvent {
    depth: Option<usize>,
    sel_depth: Option<usize>,
    time: Option<usize>,
    nodes: Option<usize>,
    pv: Vec<Box<str>>,
    multi_pv: Option<usize>,
    score: Option<ScoreEvent>,
    //curr_move: String,
    //curr_move_num: Option<usize>,
    //hash_full: Option<usize>,
    nps: Option<usize>,
    //tb_hits: Option<usize>,
    //sb_hits: Option<usize>,
    //cpu_load: Option<usize>,
    string: Box<str>,
    //refutation: Vec<String>,
    // (cpu_num, moves)
    //curr_line: Option<(usize, Vec<String>)>,
}

impl Display for InfoEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(depth) = self.depth {
            write!(f, "depth {} ", depth)?
        }
        if let Some(sel_depth) = self.sel_depth {
            write!(f, "seldepth {} ", sel_depth)?
        }
        if let Some(time) = self.time {
            write!(f, "time {} ", time)?
        }
        if let Some(nodes) = self.nodes {
            write!(f, "nodes {} ", nodes)?
        }
        if self.pv.len() > 0 {
            write!(f, "pv {} ", self.pv.join(" "))?
        }
        if let Some(multi_pv) = self.multi_pv {
            write!(f, "multipv {} ", multi_pv)?
        }
        if let Some(score) = &self.score {
            write!(f, "score {}", score)?
        }
        if let Some(nps) = self.nps {
            write!(f, "nps {} ", nps)?
        }
        if self.string.len() > 0 {
            write!(f, "string {} ", self.string)?
        }
        // TODO: additional InfoEvent here?
        Ok(())
    }
}

#[derive(Debug)]
pub struct ScoreEvent {
    cp: Option<usize>,
    mate: Option<usize>,
    bound: ScoreEventBound,
}

impl Display for ScoreEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(cp) = self.cp {
            write!(f, "cp {} ", cp)?
        }
        if let Some(mate) = self.mate {
            write!(f, "mate {} ", mate)?
        }
        match self.bound {
            ScoreEventBound::None => Ok(()),
            ScoreEventBound::Upper => write!(f, "upperbound "),
            ScoreEventBound::Lower => write!(f, "lowerbound "),
        }
    }
}

#[derive(Debug)]
pub enum ScoreEventBound {
    None,
    Upper,
    Lower,
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
    fn parse(parts: &mut SplitAsciiWhitespace) -> Result<Self, UciError> {
        if parts.next().ok_or(UciError::MissingParameter)? != "default" {
            return Err(UciError::InvalidParameter);
        }
        let default = parts.next().ok_or(UciError::MissingValue)?.parse::<usize>().map_err(|_| UciError::ParseError)?;
        if parts.next().ok_or(UciError::MissingParameter)? != "min" {
            return Err(UciError::InvalidParameter);
        }
        let min = parts.next().ok_or(UciError::MissingValue)?.parse::<usize>().map_err(|_| UciError::ParseError)?;
        if parts.next().ok_or(UciError::MissingParameter)? != "max" {
            return Err(UciError::InvalidParameter);
        }
        let max = parts.next().ok_or(UciError::MissingValue)?.parse::<usize>().map_err(|_| UciError::ParseError)?;
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
    fn parse(parts: &mut SplitAsciiWhitespace) -> Result<Self, UciError> {
        if parts.next().ok_or(UciError::MissingParameter)? != "default" {
            return Err(UciError::InvalidParameter);
        }
        Ok(Check { default: parts.next().ok_or(UciError::MissingValue)?.parse::<bool>().map_err(|_| UciError::ParseError)? })
    }
}

impl Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "default {}", self.default)
    }
}
