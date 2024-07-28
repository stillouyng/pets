use std::{fs};
use std::path::Path;
use clap::Parser;
use filetime::{set_file_mtime, FileTime};
use std::process::Command;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Properties {
    /// Filename of the file
    #[arg(short, long, action)]
    filepath: String,

}

#[derive(Debug)]
struct FileData {
    filename: String,
    extension: String
}

fn parse_filepath(filepath: &String) -> FileData {
    let mut filename_split = "";
    if filepath.contains("/") {
        filename_split = filepath.rsplit_once("/").unwrap().1;
    } else {
        filename_split = filepath;
    }
    let ext_splitter = filepath.rsplit_once(".").unwrap().1;

    let file: FileData = FileData {
        filename: filename_split.to_string(),
        extension: ext_splitter.to_string(),
    };
    return file;
}

fn update_last_modified(filepath: &String) -> bool {
    set_file_mtime(filepath, FileTime::now()).unwrap();
    return true;
}

fn create_file(filepath: &String) -> std::io::Result<()> {
    let file = fs::File::create(filepath)?;
    Ok(())
}

fn open_file(filepath: &String) {
    let command = format!("start {}", filepath);
    println!("{}", command);
    let output = Command::new("cmd")
        .args(["/C", &*command])
        .output()
        .expect("failed to execute command");
    output.stdout;
}

fn main() {
    let args: Properties = Properties::parse();
    let file: FileData = parse_filepath(&args.filepath);
    if fs::metadata(&args.filepath).is_ok() {
        update_last_modified(&args.filepath);
    } else {
        create_file(&args.filepath).expect("Error: create file");
    }
    if file.extension == "txt" {
        open_file(&args.filepath);
    }
}
