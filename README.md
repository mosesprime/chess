# Chess

> [!WARNING]
> Work in progress!

[Lichess Profile](https://lichess.org/@/MonteCristoBot)

## Components

- `engine/`: custom chess engine
- `core/`: common chess library
- `game/`: local game server & chess GUI

## Getting Started

Requires the [Rust](https://www.rust-lang.org/) toolchain.

### Chess Engine
```
cargo install --bin chess-engine
```

### Chess GUI
```
# install Dioxus CLI
cargo install dioxus-cli

# run in release mode
cd game
dx serve --release
```
