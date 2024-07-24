#![allow(non_snake_case)]

use chess_core::{board::{file::{File, NUM_BOARD_FILES}, piece::{Piece, Side}, rank::{Rank, NUM_BOARD_RANKS}, square::{Square, FILE_NAMES, RANK_NAMES}, Bitboard, Board}, moves::{generate_moves, ShortMove, MoveList}};
use dioxus::prelude::*;
use tracing::debug;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/game/:id")]
    Game { id: i32 },
}

pub fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "../global.css" }
        Router::<Route> {}
    }
}

#[component]
fn ChessBoard() -> Element {
    let board = use_context::<Signal<Board>>();
    let mut active: Signal<Option<Square>> = use_signal(|| None);
    let mut targets: Signal<Vec<Square>> = use_signal(|| vec![]);
    rsx! {
        div {
            class: "chessboard",
            for rank in (0..NUM_BOARD_RANKS).rev() {
                for file in 0..NUM_BOARD_FILES {
                    div {
                        // TODO: clean up the Square::from_coord() calls
                        class: "tile",
                        onmousedown: move |_| async move {
                            *active.write() = Some(Square::from_coord(Rank::from(rank), File::from(file)));
                            if let Ok(tars) = generate_targets(board.read().as_fen(), ((rank * 8) + file) as u8).await {
                                debug!("targets {:?}", tars);
                                *targets.write() = tars.iter().map(|&t| Square::from(t)).collect();
                            }
                        },
                        ondrop: move |_| async move {
                            // debug!("drop: {}x{} {:?}", rank, file, e);
                            if let Ok(m) = attempt_move(board.read().as_fen(), *active.read().unwrap(), ((rank * 8) + file) as u8).await {
                                let m = ShortMove::from(m);
                                if m.is_valid() {
                                    debug!("move {} -> {}", m.src().name(), m.dest().name());
                                    if m.is_capturing() {
                                        if let Some((s, p)) = board.read().square(m.dest()) {
                                            // TODO: here
                                        }
                                    }
                                } else {
                                    tracing::warn!("invalid move");
                                }
                            } else {
                                tracing::error!("attempt move server fn failed");
                            }
                        },
                        prevent_default: "ondragover ondrop ondragstart",
                        ondragover: move |_| {
                            // debug!("dragover");
                        },
                        if let Some((side, piece)) = board.read().square(Square::from_coord(Rank::from(rank), File::from(file))) {
                            if active.read().is_some_and(|s| s == Square::from_coord(Rank::from(rank), File::from(file))) {
                                // TODO: properly render active
                                PieceSprite { side, piece, active: true}
                            } else {
                                PieceSprite { side, piece, active: false }
                            }
                        }
                        if let Ok(i) = targets.read().binary_search(&Square::from_coord(Rank::from(rank), File::from(file))) {
                            img {
                                class: "center piece",
                                style: "opacity: 50%;",
                                src: "../dot.svg",
                            }
                        }
                        
                    }
                }
            }
        }
    }
}

#[server]
async fn generate_targets(fen: String, sq: u8) -> Result<Vec<u8>, ServerFnError> {
    let mut board = Board::new();
    board.load_fen(&fen);
    let square = Square::from(sq);
    debug!("generating targets from {} for {}", square.name(), fen);
    let mut targets = vec![];
    for m in generate_moves(&board).as_slice() {
        if m.src() == square {
            targets.push(*m.dest())
        }
    }
    Ok(targets)
}

#[server]
async fn attempt_move(fen: String, src: u8, dest: u8) -> Result<u16, ServerFnError> {
    let src_sq = Square::from(src);
    let dest_sq = Square::from(dest);
    let mut board = Board::new();
    board.load_fen(&fen);
    debug!("attempting move from {} to {} on {}", src_sq.name(), dest_sq.name(), fen);
    let mut flags = 0;
    if let Some((side, _piece)) = board.square(dest_sq) {
        // prohibit capture of friendly pieces
        if side == board.active_side() {
            return Ok(*Move::INVALID);
        }
        // TODO: add enpassant
        // flags |= Move::CAPTURE;
    }
    // TODO: add castling
    // TODO: add promotion
    let legal_moves = generate_moves(&board);
    Ok(*Move::new(src_sq, dest_sq, flags))
}

#[component]
fn Game(id: i32) -> Element {
    let board = Board::default();
    use_context_provider(|| Signal::new(board));
    rsx! { 
        ChessBoard {  }
        h1 { "Game: {id}" }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        Link {
            to: Route::Game {
                id: count()
            },
            "Go to game"
        }
        div {
            h1 { "counter: {count}" }
            button { onclick: move |_| count += 1, "Up" }
            button { onclick: move |_| count -= 1, "Down" }
        }
    }
}

#[component]
fn PieceSprite(side: Side, piece: Piece, active: bool) -> Element {
    let src = match (side, piece) {
        (Side::White, Piece::Pawn) => "../white_pawn.svg",
        (Side::White, Piece::Knight) => "../white_knight.svg",
        (Side::White, Piece::Bishop) => "../white_bishop.svg",
        (Side::White, Piece::Rook) => "../white_rook.svg",
        (Side::White, Piece::Queen) => "../white_queen.svg",
        (Side::White, Piece::King) => "../white_king.svg",
        (Side::Black, Piece::Pawn) => "../black_pawn.svg",
        (Side::Black, Piece::Knight) => "../black_knight.svg",
        (Side::Black, Piece::Bishop) => "../black_bishop.svg",
        (Side::Black, Piece::Rook) => "../black_rook.svg",
        (Side::Black, Piece::Queen) => "../black_queen.svg",
        (Side::Black, Piece::King) => "../black_king.svg",
    };
    rsx! { 
        img { 
            class: "piece center",
            style : if active { 
                "opacity: 50%;"
            } else {
                "opacity: 100%;"
            },
            src,
        }
    }
}
