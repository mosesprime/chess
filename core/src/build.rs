//! Generates the giant lookup tables.
//! Since tables can be large enough to cause stack overflows on some targets (*cough* Windows *cough*)
//! ensure that the table is not stored in its entirety. (ie  NOT ```let table = ROOK_MAGIC_TABLE;```)
//! Use iterators or indexing instead.

use std::{env, fs, io::{self, BufWriter, Write}, path::Path};

mod board;
mod magic;
mod tables;

use board::{piece::NUM_PIECE_SIDES, square::NUM_BOARD_SQUARES};
use magic::{BISHOP_MAGIC_TABLE_SIZE, ROOK_MAGIC_TABLE_SIZE};


fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = env::var_os("OUT_DIR").expect("env var OUT_DIR should have been set by compiler");
    let tables_path = Path::new(&out_dir).join("tables.rs");
    let mut tables_writer = std::io::BufWriter::new(fs::File::create(&tables_path).expect("failed to create tables_file"));
    write_prelude(&mut tables_writer).expect("failed to write prelude to tables");
    write_king_moves(&mut tables_writer).expect("failed to write king moves to tables");
    write_pawn_moves(&mut tables_writer).expect("failed to write pawn moves to tables");
    write_pawn_attacks(&mut tables_writer).expect("failed to write pawn attacks to tables");
    write_knight_moves(&mut tables_writer).expect("failed to write knight moves to tables");
    write_bishop_moves(&mut tables_writer).expect("failed to write bishop moves to tables");
    write_bishop_magics(&mut tables_writer).expect("failed to write bishop magics to tables");
    write_rook_moves(&mut tables_writer).expect("failed to write rook moves to tables");
    write_rook_magics(&mut tables_writer).expect("failed to write rook magics to tables");
    write_queen_moves(&mut tables_writer).expect("failed to write queen moves to tables");
}

fn write_prelude(w: &mut BufWriter<fs::File>) -> io::Result<()> {
    writeln!(w, "use board::square::NUM_BOARD_SQUARES;")?;
    writeln!(w, "use board::piece::NUM_PIECE_SIDES;")?;
    writeln!(w, "use magic::BISHOP_MAGIC_TABLE_SIZE;")?;
    writeln!(w, "use magic::ROOK_MAGIC_TABLE_SIZE;")?;
    writeln!(w, "use magic::Magic;")?;
    writeln!(w, "use board::Bitboard;")?;
    Ok(())
}

fn write_pawn_moves(w: &mut BufWriter<fs::File>) -> io::Result<()> {
    let pawn_moves = tables::pawn::gen_pawn_moves();
    write!(w, "pub const PAWN_MOVE_TABLE: [[Bitboard; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES] = [[")?;
    for i in 0..NUM_PIECE_SIDES {
        for j in 0..NUM_BOARD_SQUARES {
            write!(w, "{},", pawn_moves[i][j])?;
        }
        if i < NUM_PIECE_SIDES - 1 { write!(w, "],[")?; }
    }
    write!(w, "]];\n")?;
    Ok(())
}

fn write_pawn_attacks(w: &mut BufWriter<fs::File>) -> io::Result<()> {
    let pawn_attacks = tables::pawn::gen_pawn_attacks();
    write!(w, "pub const PAWN_ATTACK_TABLE: [[Bitboard; NUM_BOARD_SQUARES]; NUM_PIECE_SIDES] = [[")?;
    for i in 0..NUM_PIECE_SIDES {
        for j in 0..NUM_BOARD_SQUARES {
            write!(w, "{},", pawn_attacks[i][j])?;
        }
        if i < NUM_PIECE_SIDES - 1 { write!(w, "],[")?; }
    }
    write!(w, "]];\n")?;
    Ok(())
}

fn write_king_moves(w: &mut BufWriter<fs::File>) -> io::Result<()> {
    let king_moves = tables::king::gen_king_moves();
    write!(w, "pub const KING_MOVE_TABLE: [Bitboard; NUM_BOARD_SQUARES] = [")?;
    for i in 0..NUM_BOARD_SQUARES {
        write!(w, "{},", king_moves[i])?;
    }
    write!(w, "];\n")?;
    Ok(()) 
}

fn write_knight_moves(w: &mut BufWriter<fs::File>) -> io::Result<()> {
    let knight_moves = tables::knight::gen_knight_moves();
    write!(w, "pub const KNIGHT_MOVE_TABLE: [Bitboard; NUM_BOARD_SQUARES] = [")?;
    for i in 0..NUM_BOARD_SQUARES {
        write!(w, "{},", knight_moves[i])?;
    }
    write!(w, "];\n")?;
    Ok(())
}

fn write_bishop_moves(w: &mut BufWriter<fs::File>) -> io::Result<()> {
    let bishop_moves = tables::bishop::gen_bishop_moves();
    write!(w, "pub const BISHOP_MOVE_TABLE: [Bitboard; NUM_BOARD_SQUARES] = [")?;
    for i in 0..NUM_BOARD_SQUARES {
        write!(w, "{},", bishop_moves[i])?;
    }
    write!(w, "];\n")?;
    Ok(())
}

fn write_bishop_magics(w: &mut BufWriter<fs::File>) -> io::Result<()> {
    let (bishop_attacks, bishop_magics) = tables::bishop::gen_bishop_magics();
    write!(w, "pub const BISHOP_MAGIC_TABLE: [Magic; NUM_BOARD_SQUARES] = [")?;
    for i in 0..NUM_BOARD_SQUARES {
        write!(w, "{:?},", bishop_magics[i])?;
    }
    write!(w, "];\n")?;
    write!(w, "pub const BISHOP_ATTACK_TABLE: [Bitboard; BISHOP_MAGIC_TABLE_SIZE] = [")?;
    for i in 0..BISHOP_MAGIC_TABLE_SIZE {
        write!(w, "{},", bishop_attacks[i])?;
    }
    write!(w, "];\n")?;
    Ok(())
}

fn write_rook_moves(w: &mut BufWriter<fs::File>) -> io::Result<()> {
    let rook_moves = tables::rook::gen_rook_moves();
    write!(w, "pub const ROOK_MOVE_TABLE: [Bitboard; NUM_BOARD_SQUARES] = [")?;
    for i in 0..NUM_BOARD_SQUARES {
        write!(w, "{},", rook_moves[i])?;
    }
    write!(w, "];\n")?;
    Ok(())
}

fn write_rook_magics(w: &mut BufWriter<fs::File>) -> io::Result<()> {
    let (rook_attacks, rook_magics) = tables::rook::gen_rook_magics();
    write!(w, "pub const ROOK_MAGIC_TABLE: [Magic; NUM_BOARD_SQUARES] = [")?;
    for i in 0..NUM_BOARD_SQUARES {
        write!(w, "{:?},", rook_magics[i])?;
    }
    write!(w, "];\n")?;
    write!(w, "pub const ROOK_ATTACK_TABLE: [Bitboard; ROOK_MAGIC_TABLE_SIZE] = [")?;
    for i in 0..ROOK_MAGIC_TABLE_SIZE {
        write!(w, "{},", rook_attacks[i])?;
    }
    write!(w, "];\n")?;
    Ok(())
}

fn write_queen_moves(w: &mut BufWriter<fs::File>) -> io::Result<()> {
    let queen_moves = tables::queen::gen_queen_moves();
    write!(w, "pub const QUEEN_MOVE_TABLE: [Bitboard; NUM_BOARD_SQUARES] = [")?;
    for i in 0..NUM_BOARD_SQUARES {
        write!(w, "{},", queen_moves[i])?;
    }
    write!(w, "];\n")?;
    Ok(())
}
