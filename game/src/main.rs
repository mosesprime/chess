use core::{board::display_bitboard, moves::{bishop::BISHOP_MOVE_TABLES, queen::QUEEN_MOVE_TABLES, rook::ROOK_MOVE_TABLES}};

fn main() {
    println!("= BISHOP MOVE TABLES =");
    for bb in BISHOP_MOVE_TABLES {
        println!("{}\n", display_bitboard(bb));
    }

    println!("= ROOK MOVE TABLES =");
    for bb in ROOK_MOVE_TABLES {
        println!("{}\n", display_bitboard(bb));
    }

    println!("= QUEEN MOVE TABLES =");
    for bb in QUEEN_MOVE_TABLES {
        println!("{}\n", display_bitboard(bb));
    }

    let board = core::board::Board::default();
    println!("{}", board);
}
