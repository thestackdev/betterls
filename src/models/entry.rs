use crate::utils;
use std::cmp::Ordering;
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
    pub path: String,
    pub file_type: FileType,
    pub size: f64,
}

impl Entry {
    pub fn display(&self) {
        let color = utils::get_color_for_file_type(self.file_type);
        println!("{}", self.path.color(color));
    }

    pub fn display_long(self) {
        let color = utils::get_color_for_file_type(self.file_type);
        println!("{:.1}k {}", self.size, self.path.color(color));
    }
}
