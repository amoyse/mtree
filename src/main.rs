

use clap::Parser;


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
    #[arg(short, value_name = "directory")]
    ignore: String,
}


fn main() {
    let args = Args::parse();
    println!("{:?}", args.ignore);
}
