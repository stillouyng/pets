
use std::{fs, path};
use clap::{Parser};


/// Program for shows files in directory
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Properties {
    /// Shows internal files in directories
    #[arg(short, long, default_value = "None")]
    flags: String,
}

fn remove_first_chars(str: String) -> String {
    let mut chars = str.chars();
    chars.next();
    chars.next();
    return chars.as_str().to_string()
}

fn print_path(filepath: path::PathBuf, with_iternal_files: bool) {
    let mut extend: &str = "file";
    let filename: String = remove_first_chars(filepath.display().to_string());
    if filepath.clone().is_dir() {
        extend = "dir";

    }
    println!("{0: <35} | {1: <4} | ", filename, extend);
    if filepath.clone().is_dir() && with_iternal_files {
        print_internal_files(filepath);
    }
}

fn print_internal_files(filepath: path::PathBuf) {
    let files: fs::ReadDir = fs::read_dir(filepath).unwrap();
    for file in files {
        let sub_filepath: path::PathBuf = file.unwrap().path();
        print_sub_file(sub_filepath);
    }
}

fn print_sub_file(filepath: path::PathBuf) {
    let mut extend: &str = "file";
    let filename = remove_first_chars(filepath.display().to_string());
    if filepath.is_dir() {
        extend = "dir";
    }
    println!("– {0: <33} | {1: <4} | ", filename, extend);
}

fn main() {
    let files: fs::ReadDir = fs::read_dir(".\\").unwrap();
    let args: Properties = Properties::parse();
    for file in files {
        let filepath: path::PathBuf = file.unwrap().path();
        if args.flags == "R" {
            print_path(filepath, true);
        } else {
            print_path(filepath, false);
        }
    }
}
