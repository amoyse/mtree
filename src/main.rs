

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


fn walk_dir(start_dir: &str, prefix: &str, all: bool, ignore_dir: &str) -> io::Result<()> {

    // Sort list of paths
    let mut paths: Vec<_> = fs::read_dir(start_dir).unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    let mut it = paths.iter().peekable();

    while let Some(path) = it.next() {
        let entity = path.file_name().into_string().unwrap();

        if (entity.starts_with(".") && !all) || entity == ignore_dir {
            continue;
        }

        if it.peek().is_none() {
            println!("{}└── {}", prefix, entity);
            if path.path().is_dir() {
                walk_dir(&format!("{}/{}", start_dir, entity), &format!("{}│   ", prefix), all, ignore_dir)?;
            }
        } else {
            println!("{}├── {}", prefix, entity);
            if path.path().is_dir() {
                walk_dir(&format!("{}/{}", start_dir, entity), &format!("{}│   ", prefix), all, ignore_dir)?;
            }
        }
    }

    Ok(())
}


fn main() -> io::Result<()> {
    let args = Args::parse();
    let ignore_dir = args.ignore;
    let all = args.all;

    let start_path = ".";

    println!(".");
    walk_dir(start_path, "", all, &ignore_dir)
}
