use chess_game::App;
use dioxus::prelude::*;

fn main() {
    #[cfg(debug_assertions)]
    dioxus_logger::init(tracing::Level::DEBUG).expect("failed to init logger");
    #[cfg(not(debug_assertions))]
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");

    launch(App)
}
