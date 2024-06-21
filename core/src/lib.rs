pub mod board;
pub mod moves;
pub mod uci;

include!(concat!(env!("OUT_DIR"), "/tables.rs"));
