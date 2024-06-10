use std::{fs, path};
use clap::{Parser};


#[derive(Parser)]
struct Properties {
    #[arg(default_value = "None")]
    param: String
}

fn print_path(filepath: path::PathBuf, with_iternal_files: bool) {
    let mut extend: &str = "file";
    if filepath.clone().is_dir() {
        extend = "dir";
    }
    println!("{0: <15} | {1: <4} | ", filepath.display(), extend);
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
    if filepath.is_dir() {
        extend = "dir";
    }
    println!("– {0: <25} | {1: <4} | ", filepath.display(), extend);
}

fn print_help() {
    let available_params = [
        ("help", "Shows help"),
        ("R", "Shows internal files in folders")
    ];
    println!("Use \"ls -- *param*\"");
    for params in available_params {
        println!("{0: <5} | {1}", params.0, params.1);
    }
}

fn main() {
    let files: fs::ReadDir = fs::read_dir(".\\").unwrap();
    let args: Properties = Properties::parse();
    if args.param == "help" {
        print_help();
    } else if args.param == "None" || args.param == "R" {
        for file in files {
            let filepath: path::PathBuf = file.unwrap().path();
            if args.param == "R" {
                print_path(filepath, true);
            } else {
                print_path(filepath, false);
            }
        }
    } else {
        panic!("Use ls -- help")
    }
}
