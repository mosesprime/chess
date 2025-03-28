pub mod board;
pub mod game;
pub mod magic;
pub mod moves;

include!(concat!(env!("OUT_DIR"), "/tables.rs"));
