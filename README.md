# Life

A simple [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) written in Rust.

## Features

- [x] Beautifully display grids in the console!
- [x] Start with a new randomized grid.
- [x] Customize starting grid width, height and amount of living cells.

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

You can specify the following options:

```
USAGE:
    life [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --alive <alive-ratio>             Starting grid ratio of alive cells, between 0 and 1 [default: 0.3]
    -h, --height <grid-height>            Starting grid height, at least 3 [default: 10]
    -w, --width <grid-width>              Starting grid width, at least 3 [default: 20]
```

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
