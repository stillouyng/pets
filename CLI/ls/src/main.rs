use std::{fs, path};
use clap::{Parser};
use colored::Colorize;


/// Program for print files in directory
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Properties {
    /// Print internal files in directories
    #[arg(short = 'R', action)]
    r: bool,

    /// Print files in the specified directory
    #[arg(short, long, default_value = ".\\")]
    path: String
}

fn remove_first_chars(str: String) -> String {
    let mut chars = str.chars();
    chars.next();
    chars.next();
    return chars.as_str().to_string()
}

fn remove_backslash_from_path(filepath: path::PathBuf) -> String{
    let filename: String;
    if filepath.display().to_string().contains(".\\") {
        filename = remove_first_chars(filepath.display().to_string());
    } else {
        filename = filepath.display().to_string();
    }
    return filename
}

fn print_path(filepath: path::PathBuf, with_iternal_files: bool) {
    let filename = remove_backslash_from_path(filepath.clone());
    let extend;
    if filepath.clone().is_dir() {
        extend = "dir".yellow().bold();
    } else {
        extend = "file".white().bold();
    }
    println!("{0: <35} | {1: <4} | ", filename, extend);
    if filepath.clone().is_dir() && with_iternal_files {
        print_internal_files(filepath);
    }
}

fn print_internal_files(filepath: path::PathBuf) {
    let files_result = fs::read_dir(filepath.clone());
    match files_result {
        Ok(files) =>
            for file in files {
                let sub_filepath: path::PathBuf = file.unwrap().path();
                print_sub_file(sub_filepath);
            }
        Err(error) => print_error(error)
    }
}

fn print_sub_file(filepath: path::PathBuf) {
    let filename = remove_backslash_from_path(filepath.clone());
    let extend;
    if filepath.is_dir() {
        extend = "dir".yellow().bold();
    } else {
        extend = "file".white().bold();
    }
    println!("– {0: <33} | {1: <4} | ", filename, extend);
}

fn print_underline() {
    let mut i = 0;
    while i < 44 {
        print!("-");
        i += 1;
    }
    println!();
}

fn print_error<T: std::error::Error>(error: T) {
    println!(" - {}", error);
}

fn print_header() {
    println!("{0: <35} | {1: <4} |", "FILEPATH", "TYPE");
    print_underline();
}

fn main() {
    let args: Properties = Properties::parse();
    let files_result = fs::read_dir(args.path);
    match files_result {
        Ok(files) => {
            print_header();
            for file in files {
                let filepath: path::PathBuf = file.unwrap().path();
                if args.r {
                    print_path(filepath, true);
                } else {
                    print_path(filepath, false);
                }
            }
            print_underline();
        },
        Err(error) => print_error(error)
    }
}
