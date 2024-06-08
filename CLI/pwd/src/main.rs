use std::{env, io::Result, path::PathBuf};


fn main() -> Result<()> {
    let path: PathBuf = env::current_dir()?;
    println!("{}", path.to_string_lossy());
    Ok(())
}
