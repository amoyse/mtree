

use clap::Parser;
use walkdir::{DirEntry, WalkDir};


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


fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn cut_first_two_chars(path: &str) -> &str {
    let mut path_string = path;
    path_string = &path_string[2..];
    path_string
}


fn main() {
    let args = Args::parse();
    let ignore_dir = args.ignore;
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        let full_path = entry.path().to_str().unwrap();
        if full_path.chars().count() > 2 {
            let new_path = cut_first_two_chars(full_path);
            println!("{}", new_path);
        }
    }
}
