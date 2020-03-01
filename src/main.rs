extern crate serde;

mod mata_info;
mod runtime;
mod type_checker;

use std::fs::File;
use std::io::Read;
// use std::sync::mpsc;

use serde_json;
//use serde_lexpr;

use mata_info::ast;
use mata_info::ast::Expr;
use mata_info::ast::Literal::Bool;
use std::cell::RefCell;
use std::mem::size_of;
//use mata_info::{node, node::Node};

fn main() {
    println!("Hello, world!");
}

#[test]
fn write_expr() {
    let expr = Expr::Unknown;
    println!("write out: {}", serde_json::to_string(&expr).unwrap());
    let expr = Expr::Literal(Bool(true));
    println!("write out: {}", serde_json::to_string(&expr).unwrap());
    // let expr = Expr::IfThenElse(
    //     Box::new(Expr::Unknown),
    //     Box::new(Expr::Unknown),
    //     Box::new(Expr::Unknown),
    // );
    println!("write out: {}", serde_json::to_string(&expr).unwrap());
    // let expr = //Expr::Literal(Bool(true));
    //     Expr::IfElseIf(
    //         vec![(Expr::Unknown, Expr::Unknown)],
    //         Box::new(Expr::Unknown)
    //     );
    println!("write out: {}", serde_json::to_string(&expr).unwrap());
}

/*
#[test]
fn fuck_ser() {
    let mut src = String::new();
    let _ = File::open("examples/test1.pst")
        .expect("file open failed")
        .read_to_string(&mut src);
    let result: Node = serde_json::from_str(&src).unwrap();
    println!("output: {:?}", result);
}
*/
