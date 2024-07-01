#![allow(non_snake_case)]

use chess_core::board::{file::{File, NUM_BOARD_FILES}, piece::{Piece, Side}, rank::{Rank, NUM_BOARD_RANKS}, square::{Square, FILE_NAMES, RANK_NAMES}, Bitboard, Board};
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
    let mut targeted: Signal<Bitboard> = use_signal(|| 0);
    rsx! {
        div {
            class: "chessboard",
            for rank in (0..NUM_BOARD_RANKS).rev() {
                for file in 0..NUM_BOARD_FILES {
                    div {
                        onclick: move |_| *active.write() = Some(Square::from_coord(Rank::from_index(rank), File::from_index(file))),
                        if let Some((side, piece)) = board.read().square(Square::from_coord(Rank::from_index(rank), File::from_index(file))) {
                            if active.read().is_some_and(|s| s == Square::from_coord(Rank::from_index(rank), File::from_index(file))) {
                                // TODO: properly render active
                                PieceSprite { side, piece, active: true}
                            } else {
                                PieceSprite { side, piece, active: false }
                            }
                        }
                        // TODO: add targeted
                    }
                }
            }
        }
    }
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
    debug!("render {:?} {:?} active: {}", side, piece, active);
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
    let style = match active {
        false => "width: 100%; height: auto; background-color: transparent;",
        true =>  "width: 100%; height: auto; background-color: transparent; opacity: 50%;"
    };
    rsx! { 
        img { 
            class: "center",
            "style" : style,
            onclick: move |event| tracing::debug!("click piece event: {event:?}"),
            src,
        }
    }
}
