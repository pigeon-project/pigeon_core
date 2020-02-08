extern crate serde;

mod mata_info;

use mata_info::{node, ast};
use serde_json;
use std::fs::File;
use std::io::Read;
use crate::mata_info::node::Node;

fn main() {
    println!("Hello, world!");
}


#[test]
fn fuck_ser() {
    let mut src = String::new();
    File::open("examples/test1.pst")
        .expect("file open failed")
        .read_to_string(&mut src);
    let result: Node = serde_json::from_str(&src).unwrap();
    println!("output: {:?}", result);
}