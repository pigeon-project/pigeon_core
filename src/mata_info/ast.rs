

enum Literal {
    Bool(bool),
    I64(i64),
    U64(u64),
    Float(f64),
}

enum Ast {
    Literal(Literal),
    If(Box<Ast>, Box<Ast>, Box<Ast>),
}