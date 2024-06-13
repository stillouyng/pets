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

fn make_size_human_readable(mut size: u64) -> String {
    return if size == 0 {
        "".to_string()
    } else {
        let mut word: &str = "KB";
        size = size / 1024;
        if size > 1024 {
            size = size / 1024;
            word = "MB";
        }
        format!("{0: <4} {1}", size, word).to_string()
    }
}

fn get_file_size(filepath: path::PathBuf) -> Result<String, &'static str> {
    let size_result = filepath.symlink_metadata();
    return match size_result {
        Ok(size) => {
            Ok(make_size_human_readable(size.len()))
        },
        Err(_) => {
            Ok("".parse().unwrap())
        }
    }

}

fn print_path(filepath: path::PathBuf, with_iternal_files: bool) {
    let filename = remove_backslash_from_path(filepath.clone());
    let extend;
    if filepath.clone().is_dir() {
        extend = "dir".yellow().bold();
    } else {
        extend = "file".white().bold();
    }
    let size_result = get_file_size(filepath.clone());
    match size_result {
        Ok(size) => {
            println!("{0: <35} | {1: <4} | {2: <7} |", filename, extend, size);
            if filepath.clone().is_dir() && with_iternal_files {
                print_internal_files(filepath);
            }
        },
        Err(none_size) => {
            println!("{0: <35} | {1: <4} | {2: <7} |", filename, extend, none_size);
            if filepath.clone().is_dir() && with_iternal_files {
                print_internal_files(filepath);
            }
        }
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
    let size_result = get_file_size(filepath);
    match size_result {
        Ok(size) => {
            println!("{0: <35} | {1: <4} | {2: <7} |", filename, extend, size);
        }
        Err(none_size) => {
            println!("{0: <35} | {1: <4} | {2: <7} |", filename, extend, none_size);
        }
    }
}

fn print_underline() {
    let mut i = 0;
    while i < 54 {
        print!("-");
        i += 1;
    }
    println!();
}

fn print_error<T: std::error::Error>(error: T) {
    println!(" - {}", error);
}

fn print_header() {
    println!("{0: <35} | {1: <4} | {2: <7} |", "FILEPATH", "TYPE", "SIZE");
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
