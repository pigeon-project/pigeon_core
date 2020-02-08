use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Literal {
    Bool(bool),
    Char(char),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    PString(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TypeExpr {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Pattern {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Unknown,
    Literal(Literal),
    Call(Box<Expr>, Vec<Expr>),
    Lambda(Vec<Expr>, Vec<Expr>),
    TypeAssert(Box<Expr>, Box<TypeExpr>),
    LetIn(Box<Pattern>, Box<Expr>, Vec<Expr>),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
    IfElseIf(Vec<(Expr, Expr)>, Box<Expr>),
}
