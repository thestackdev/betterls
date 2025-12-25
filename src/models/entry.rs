use crate::utils;
use std::cmp::Ordering;
use std::fs;
use std::fs::FileType;

use colored::Colorize;

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for Entry {}

pub struct Entry {
    path: String,
    file_type: FileType,
    size: u64,
}

impl Entry {
    fn display(&self) {
        let color = utils::get_color_for_file_type(self.file_type);
        println!("{}", self.path.color(color));
    }

    fn display_long(self) {
        let color = utils::get_color_for_file_type(self.file_type);
        let human_readable_size = utils::get_human_readable_size(self.size);
        println!("{} {}", human_readable_size, self.path.color(color));
    }
}

pub struct Entries {
    pub entries: Vec<Entry>,
}

impl Entries {
    pub fn init(path: String, show_hidden: bool) -> Self {
        let mut entries: Vec<Entry> = fs::read_dir(path)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|filtered_entry| {
                let file_type = filtered_entry.file_type().ok()?;
                let file_size = filtered_entry.metadata().ok()?;
                Some(Entry {
                    path: filtered_entry.file_name().to_string_lossy().to_string(),
                    file_type,
                    size: file_size.len(),
                })
            })
            .collect();

        if !show_hidden {
            entries.retain(|entry| !entry.path.starts_with("."));
        }

        entries.sort();

        Self { entries }
    }

    pub fn display(self) {
        for entry in self.entries {
            entry.display();
        }
    }

    pub fn display_long(self) {
        for entry in self.entries {
            entry.display_long();
        }
    }
}
