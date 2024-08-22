

use clap::error::Result;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Display hidden files too
    #[arg(short)]
    all: bool,

    /// Display only directories
    #[arg(short)]
    directories: bool,

    /// Supply directory to be ignored
    #[arg(short, value_name = "directory", default_value = "-null")]
    ignore: String,
}


fn walk_dir(path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let path = entry?.path();

        println!("{}", path.display());

        if path.is_dir() {
            walk_dir(&path)?;
        }
    }

    Ok(())
}


fn main() -> io::Result<()> {
    let args = Args::parse();
    let ignore_dir = args.ignore;

    let start_path = Path::new(".");

    walk_dir(start_path)
}
