pub mod bishop;
pub mod king;
pub mod knight;
pub mod magic;
pub mod pawn;
pub mod queen;
pub mod rook;

pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn new() -> Self {
        Self {}
    }
}
