# Life

A simple [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) written in Rust.

## Features

- [x] Continuously update the grid of cells.
- [x] Beautifully display grids in the console!
- [x] Serve a web page that displays the current grid in HTML.
- [x] Start with a new randomized grid.
- [x] Customize starting grid width, height and amount of living cells.
- [x] Customize grid update frequency.

## Usage

- Download & unzip the [latest release](https://github.com/splo/life/releases/latest).
- Then just run the single binary file:

```bash
./bin/life
```

Example output:

```
Server running at http://localhost:8090/
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
    -p, --port <port>                     Web server port to listen to [default: 8090]
    -f, --frequency <update-frequency>    Cell generation update frequency in Hz [default: 1.0]
```

## Development

### Requirements

- [Rust](https://rustup.rs/) 2018 edition.
- [`git-chglog`](https://github.com/git-chglog/git-chglog) to generate a changelog.

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

### Committing

This project adheres to the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specifications.

All commit messages should follow the following template:

```
[optional type: ]<description>

[optional body]
```

- No need for a `scope` part.
- Only the `feat` and `fix` types are used.
- Description should start with an upper case character and should **not** finish with a period.

### Generating Changelog

This project follows the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) principle.

In order to generate a changelog, after committing your latest changes, run the following commands.

```bash
git-chglog -c .chglog/changelog.yml > ./CHANGELOG.md
```
