use clap::Parser;
use std::fs;
use std::io;
use std::process;

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
 
    /// Supply a maximum depth to be travelled 
    #[arg(short, long, value_name = "depth", default_value = "2")]
    max_depth: i32,
}

struct Counts {
    dirs: i32,
    files: i32,
    max_depth:i32
}


fn walk_dir(start_dir: &str, prefix: &str, all: bool, only_directories: bool, ignore_dir: &str, counts: &mut Counts, depth: i32) -> io::Result<()> {

    // Sort list of paths
    let mut paths: Vec<_> = fs::read_dir(start_dir).unwrap().map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    let mut it = paths.iter().peekable();

    while let Some(path) = it.next() {

        // if counts.depth > counts.max_depth {
        //     continue;
        // }

        if only_directories && !path.path().is_dir() {
            continue;
        }

        let entity = path.file_name().into_string().unwrap();

        if (entity.starts_with(".") && !all) || entity == ignore_dir {
            continue;
        }
        if path.path().is_dir() {
            counts.dirs += 1;
        } else {
            counts.files += 1;
        }

        if counts.dirs > 5000 || counts.files > 5000 {
            eprintln!("Error: too many elements. The tree is too big, don't run so high up the filesystem!");
            process::exit(1);
        }

        if it.peek().is_none() {
            println!("{}└── {}", prefix, entity);
            if path.path().is_dir() {
                if depth < counts.max_depth {
                    walk_dir(&format!("{}/{}", start_dir, entity), &format!("{}│   ", prefix), all, only_directories, ignore_dir, counts, depth + 1)?;
                }
            }
        } else {
            println!("{}├── {}", prefix, entity);
            if path.path().is_dir() {
                if depth < counts.max_depth {
                    walk_dir(&format!("{}/{}", start_dir, entity), &format!("{}│   ", prefix), all, only_directories, ignore_dir, counts, depth + 1)?;
                }
            }
        }
    }
    Ok(())
}


fn main() -> io::Result<()> {
    let args = Args::parse();
    let ignore_dir = args.ignore;
    let all = args.all;
    let only_directories = args.directories;
    let max_depth = args.max_depth;

    let start_path = ".";

    let mut counts = Counts { dirs : 0, files : 0, max_depth};

    println!(".");
    walk_dir(start_path, "", all, only_directories, &ignore_dir, &mut counts, 0)?;
    println!("\n{} directories, {} files", counts.dirs, counts.files);
    Ok(())
}
