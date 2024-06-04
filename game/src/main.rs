fn main() {
    let mut board = core::board::Board::new();
    board.load_fen(core::board::fen::DEFAULT_FEN_START).expect("failed fen");
    println!("{}", board)
}
