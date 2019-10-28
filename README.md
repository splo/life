# Life

A simple [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) written in Rust.

## Features

- [x] Beautifully display grids in the console!
- [x] Start with a new randomized grid.

## Usage

- Download & unzip the [latest release](https://github.com/splo/life/releases/latest).
- Then just run the single binary file:

```bash
./bin/life
```

Output:

```
/--------------------\
|   o   o            |
|oo     o  o   o  o o|
| o o o oooo ooo  oo |
| o       o     o    |
|oo  o   o  o    oo  |
|  oo   oo       oooo|
| o oo      o o    o |
| o o  o o o  o o   o|
|o          oo   o   |
|    o oo o o ooo    |
\--------------------/
```

- Press `Ctrl-C` to stop the game.

## Development

### Requirements

- [Rust](https://rustup.rs/) 2018 edition.

### Building, Testing and Running

Just run standard [cargo](https://doc.rust-lang.org/cargo/) commands.

```bash
# Build and test
cargo test
# Build and run the debug version
cargo run
# Build and run the release version
cargo run --release
```
