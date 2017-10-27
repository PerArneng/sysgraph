#![allow(unused_variables)]
#[allow(dead_code)]


mod sysgraph;
mod utils;

use sysgraph::args::{Args};
use std::process;


fn main() {

    let args_result = Args::parse();
    if args_result.is_err() {
        eprintln!("error: {}", args_result.unwrap_err());
        process::exit(1);
    }

    let args = args_result.unwrap();

    utils::ensure_file_exists(&args.spec_file);



    println!("args => {:?}", args);
}
