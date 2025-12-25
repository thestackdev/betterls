use betterls::{Cli, Entry};
use clap::Parser;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let path = args.path.unwrap_or_else(|| ".".to_string());
    let show_hidden = args.all;
    let show_additional_contents = args.long;

    let mut entries: Vec<Entry> = fs::read_dir(path)?
        .map(|entry| entry.unwrap())
        .filter_map(|filtered_entry| {
            let file_type = filtered_entry.file_type().ok()?;
            let file_size = filtered_entry.metadata().ok()?;
            Some(Entry {
                path: filtered_entry.file_name().to_string_lossy().to_string(),
                file_type,
                size: (file_size.len() as f64) / 1024.0,
            })
        })
        .collect();

    if !show_hidden {
        entries.retain(|entry| !entry.path.starts_with("."));
    }

    entries.sort();

    if show_additional_contents {
        for entry in entries {
            entry.display_long();
        }
    } else {
        for entry in entries {
            entry.display();
        }
    }

    Ok(())
}
