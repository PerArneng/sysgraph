#![allow(unused_variables)]
#[allow(dead_code)]


mod sysgraph;

use sysgraph::args::{Args};



fn main() {

    let args = Args::parse().unwrap();



    println!("args => {:?}", args);
}
