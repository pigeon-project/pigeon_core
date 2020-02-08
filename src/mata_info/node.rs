use serde::{Deserialize, Serialize};

use super::ast::Expr::*;
use super::ast::Literal::*;
use crate::mata_info::ast;
use crate::mata_info::ast::Expr;
use crate::mata_info::ast::Expr::Literal;

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature(pub String, pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub typeis: String,
    pub features: Vec<Feature>,
    pub value: String,
    pub subtree: Vec<Node>,
}

impl Node {
    fn tree_node_assert(&self) -> bool {
        self.subtree.len() != 0
    }

    fn lit2ast<T: std::str::FromStr>(&self, fun: fn(T) -> ast::Literal) -> Result<Expr, ()> {
        if self.tree_node_assert() {
            match self.value.parse::<T>() {
                Ok(r) => Ok(Literal(fun(r))),
                Err(_) => Err(()),
            }
        } else {
            Err(())
        }
    }

    fn to_ast(&self) -> Result<Expr, ()> {
        match &self.typeis[..] {
            "unknown" => {
                if self.tree_node_assert() && self.value.len() == 0 {
                    Ok(Unknown)
                } else {
                    Err(())
                }
            }
            "bool" => self.lit2ast(Bool),
            "char" => self.lit2ast(Char),
            "i64" => self.lit2ast(I64),
            "u64" => self.lit2ast(U64),
            "f32" => self.lit2ast(F32),
            "f64" => self.lit2ast(F64),
            "str" => self.lit2ast(PString),

            _ => Err(()),
        }
    }
}
