use std::{env, io::Result, path::PathBuf};
use clap::{Parser};


/// Program for print current path
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Properties {}


fn main() -> Result<()> {
    Properties::parse();
    let path: PathBuf = env::current_dir()?;
    println!("{}", path.to_string_lossy());
    Ok(())
}
