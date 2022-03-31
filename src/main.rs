#![allow(dead_code)]
extern crate sha1;
extern crate digest;

use clap::Parser;


/// Simple program to calculate a directory-tree checksum
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    starting_dir: String,

}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn cksum_as_string(fname: &String) -> String {
    let data = fname.as_bytes();
    let mut m = sha1::Sha1::new();
    m.update(data);
    m.digest().to_string()
}
use walkdir::{DirEntry, WalkDir};
fn main() {
    // let args: Vec<String> = env::args().collect();
    let _args = Args::parse();
    let starting_dir = std::env::args().nth(2).expect("no starting path given");

    let dir= format!("{}", &starting_dir);
    // if args.len()>1 {
        WalkDir::new(format!("{}",&dir))
            .into_iter()
            .filter_entry(|e| is_not_hidden(e))
            .filter_map(|v| v.ok())
            .for_each(|x| println!("\n{}: {}", x.path().display(), cksum_as_string(&dir)));

    // }
    println!("\n\n");
}