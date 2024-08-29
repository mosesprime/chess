pub mod board;
pub mod game;
pub mod magic;
pub mod moves;
pub mod uci;

include!(concat!(env!("OUT_DIR"), "/tables.rs"));
