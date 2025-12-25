# betterls

A fast, colorized `ls` replacement built in Rust.

## Features

- Colorized output - directories, symlinks, and files have distinct colors
- Alphabetically sorted entries
- Hidden files filtered by default
- Long format with permissions, human-readable sizes, and modified dates

## Installation

```bash
git clone https://github.com/yourusername/betterls.git
cd betterls
cargo build --release
```

Add to your path or create an alias:

```bash
alias bls="./target/release/betterls"
```

## Usage

```bash
betterls                  # list current directory
betterls /path/to/dir     # list specific directory
betterls -a               # include hidden files
betterls -l               # long format with sizes
betterls -la ~/Downloads  # combine flags
```

## Options

```
-a, --all      Show hidden files
-l, --long     Display permissions, sizes, and dates
-h, --help     Print help
-V, --version  Print version
```

## Built With

- [clap](https://crates.io/crates/clap) - CLI argument parsing
- [colored](https://crates.io/crates/colored) - Terminal colors
- [chrono](https://crates.io/crates/chrono) - Date and time formatting

## License

MIT
