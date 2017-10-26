#![allow(unused_variables)]
#[allow(dead_code)]


mod sysgraph;

use sysgraph::args::{Args};
use std::process;


fn main() {

    let args = Args::parse();

    if args.is_err() {
        eprintln!("error: {}", args.unwrap_err());
        process::exit(1);
    }

    println!("args => {:?}", args);
}
