use colored::Color;
use std::{
    fs::{FileType, Permissions, read_dir},
    os::unix::fs::PermissionsExt,
};

fn _decimal_to_octal(octal_number: u32) -> String {
    let mut cloned_octal = octal_number;
    let mut reminders = String::new();

    while cloned_octal > 0 {
        reminders = reminders + &(cloned_octal % 8).to_string();
        cloned_octal /= 8;
    }

    reminders.chars().rev().collect()
}

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

pub fn get_formatted_permissions(mode: u32, file_type: FileType) -> String {
    let file_char = if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else {
        '-'
    };

    let perms = [
        if mode & 0o400 != 0 { 'r' } else { '-' },
        if mode & 0o200 != 0 { 'w' } else { '-' },
        if mode & 0o100 != 0 { 'x' } else { '-' },
        if mode & 0o040 != 0 { 'r' } else { '-' },
        if mode & 0o020 != 0 { 'w' } else { '-' },
        if mode & 0o010 != 0 { 'x' } else { '-' },
        if mode & 0o004 != 0 { 'r' } else { '-' },
        if mode & 0o002 != 0 { 'w' } else { '-' },
        if mode & 0o001 != 0 { 'x' } else { '-' },
    ];

    format!("{}{}", file_char, perms.iter().collect::<String>())
}
