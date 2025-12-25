use betterls::{Cli, Entries};
use clap::Parser;
use std::io;

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let path = args.path.unwrap_or_else(|| ".".to_string());
    let show_hidden = args.all;
    let show_additional_contents = args.long;

    let entries = Entries::init(path, show_hidden);

    if show_additional_contents {
        entries.display_long();
    } else {
        entries.display();
    }

    Ok(())
}
