use colored::Color;
use std::fs::FileType;

pub fn get_color_for_file_type(file_type: FileType) -> Color {
    if file_type.is_dir() {
        Color::Blue
    } else if file_type.is_symlink() {
        Color::Cyan
    } else {
        Color::White
    }
}

pub fn get_human_readable_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{:.1} K", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.1} M", bytes as f64 / MB as f64)
    } else {
        format!("{:.1} G", bytes as f64 / GB as f64)
    }
}
