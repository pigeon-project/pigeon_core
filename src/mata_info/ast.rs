use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type Id = u64;
type Level = u64;

#[derive(Serialize, Deserialize, Debug)]
pub enum BaseType {
    Bottom,
    Unit,
    Bool,
    Char,
    I64,
    U64,
    F32,
    F64,
    PString,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TypeVar {
    Unbound(Id, Level),
    Link(Box<TypeExpr>),
    Generic(Id),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TypeExpr {
    BaseType(BaseType),
    NamedItem(String, Box<TypeExpr>),
    Sum(String, Box<TypeExpr>),
    Product(Box<TypeExpr>, Box<TypeExpr>),
    TypeApply(String, Vec<TypeExpr>),
    TypeVar(Box<TypeVar>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Literal {
    Unit,
    Bool(bool),
    Char(char),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    PString(String),
    Tuple(Vec<Literal>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Pattern {
    Ignore,
    Lit(Literal),
    Name(String),
    MatchName(String),
    TypeAssert(Box<Pattern>, Box<TypeExpr>),
    Union(String, Box<TypeExpr>),
    Tuple(Box<TypeExpr>, Box<TypeExpr>),
    // feature
    // list, etc.
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FunctionFlag {
    Extern,
    Const,
    Inline,
    Rec,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeConstructor(pub String, pub Vec<String>, pub Vec<TypeExpr>);

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionConstructor(
    pub Option<FunctionFlag>,
    pub String,
    pub Vec<String>,
    pub Vec<Pattern>,
    pub HashMap<String, TypeExpr>,
    pub Vec<Expr>,
    pub Vec<FunctionConstructor>,
);

#[derive(Serialize, Deserialize, Debug)]
pub enum Expr {
    Unknown,
    Literal(Literal),
    Typed(Box<Expr>, Box<TypeExpr>),
    Call(Box<Expr>, Vec<Expr>),
    Lambda(Vec<Pattern>, Vec<Expr>),
    NamedFun(Box<FunctionConstructor>),
    TypeAssert(Box<Expr>, Box<TypeExpr>),
    Let(Box<Pattern>, Box<Expr>),
    LetIn(Box<Pattern>, Box<Expr>, Vec<Expr>),
    UseIn(Box<Pattern>, Box<Expr>, Vec<Expr>),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
    IfElseIf(Vec<(Expr, Expr)>, Box<Expr>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TopLevel {
    Comment(String),
    TypeDef(Box<TypeConstructor>),
    FuncDef(Box<FunctionConstructor>),
    DoBlock(Vec<Expr>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReplTopLevel {
    Comment(String),
    TypeDef(Box<TypeConstructor>),
    FuncDef(Box<FunctionConstructor>),
    Expression(Box<Expr>),
}
