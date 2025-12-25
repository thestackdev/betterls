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
