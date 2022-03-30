#![allow(dead_code)]

// use std::env::Args;
use clap::Parser;


/// Simple program to calculate a directory-tree checksum
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    starting_dir: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}
use walkdir::{DirEntry, WalkDir};
fn main() {
    let args = Args::parse();
    // println!("\nInitial directory is: {}", args.starting_dir);

    WalkDir::new(args.starting_dir)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display()));
}