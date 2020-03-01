use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

pub type Id = u64;
pub type Level = u64;
pub type Name = String;

// Editor Level

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Path(pub Vec<u64>);

pub type RErrorCode = &'static str;
pub type RErrorMessage = &'static str;

pub type RErrorInfo = (RErrorCode, RErrorMessage);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorInfo(String, String);

impl From<RErrorInfo> for ErrorInfo {
    fn from(input: RErrorInfo) -> Self {
        ErrorInfo(input.0.to_string(), input.1.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompileError(pub Path, pub ErrorInfo);

pub type CompileResult<T> = Result<T, CompileError>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TypeVar {
    Unbound(Id, Level),
    Link(Box<TypeExpr>),
    Generic(Id),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TypeExpr {
    BaseType(BaseType),               // built-in type
    TConst(Name),                     // type name
    NamedItem(String, Box<TypeExpr>), // record item
    Sum(String, Vec<TypeExpr>),       // enum type
    Product(Vec<TypeExpr>),           // struct type
    TypeArrow(Vec<TypeExpr>),         // function type
    TypeApply(String, Vec<TypeExpr>), // generic apply
    TypeVar(Box<TypeVar>),            // generic type
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConstraintContext(pub RefCell<HashMap<Name, TypeExpr>>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Context(pub ConstraintContext, pub ConstraintContext); // type context, variable context

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Pattern {
    Ignore,
    Lit(Literal),
    Variable(Name),
    MatchVariable(String),
    TypeAssert(Box<Pattern>, Box<TypeExpr>),
    Union(String, Box<TypeExpr>),
    Tuple(Vec<Pattern>, bool),
    // feature
    // list, etc.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FunctionFlag {
    // Extern,
    Const,
    Inline,
    Rec,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypeConstructor(
    pub String,               // type name
    pub Vec<String>,          // type variable name
    pub Vec<TypeExpr>,        // type body
    pub Vec<TypeConstructor>, // rec type
    pub Vec<Name>,            // type dervings
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionBody(
    pub HashMap<String, TypeExpr>, // type where
    pub Vec<Expr>,                 // function statement
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionConstructor(
    pub Option<FunctionFlag>,
    pub String,
    pub Vec<(Box<Pattern>, Box<TypeExpr>)>, // parameter
    pub Box<TypeExpr>,                      // return type
    pub Vec<FunctionBody>,
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Expr {
    Unknown,
    Literal(Literal),
    Variable(Name),
    TypeAssert(Box<Expr>, Box<TypeExpr>),
    Call(Box<Expr>, Vec<Expr>),
    Lambda(Vec<Pattern>, Vec<Expr>),
    NamedFun(Box<FunctionConstructor>),
    Let(Box<Pattern>, Box<Expr>),
    LetIn(Box<Pattern>, Box<Expr>, Vec<Expr>),
    UseIn(Box<Pattern>, Box<Expr>, Vec<Expr>),
    IfThenElseIfElse(Vec<(Expr, Expr)>, Box<Expr>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TopLevel {
    Comment(String),
    TypeDef(Box<TypeConstructor>),
    FuncDef(Box<FunctionConstructor>),
    DoBlock(Vec<Expr>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReplTopLevel {
    Comment(String),
    TypeDef(Box<TypeConstructor>),
    FuncDef(Box<FunctionConstructor>),
    Expression(Box<Expr>),
}

// Compile Level

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expr1(pub Expr);

impl Expr1 {
    fn unpack(self) -> Expr {
        self.0
    }
}
