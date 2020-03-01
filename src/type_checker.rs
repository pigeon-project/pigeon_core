use crate::mata_info::ast::*;
use std::borrow::{Borrow, BorrowMut};

const TYPE_ASSERT_ERR: RErrorInfo = ("T0", "Expression type assertion error");
const CALLEE_IS_NOT_FUNCTION_TYPE: RErrorInfo = ("T1", "Callee is not a function type");
const VARIABLE_IS_NOT_DEFINED: RErrorInfo = ("T2", "Variable is not defined");

fn new_path_level(path: &Path, pos: u64) -> Path {
    let mut res = path.0.clone();
    res.push(pos);
    return Path(res);
}

fn unifier(ty1: &TypeExpr, ty2: &TypeExpr) -> bool {
    match (ty1, ty2) {
        (TypeExpr::BaseType(x), TypeExpr::BaseType(y)) => x == y,
        // (TypeExpr::)
        _ => unimplemented!(),
    }
}

fn literal_infer(lit: Literal) -> TypeExpr {
    match lit {
        Literal::Unit => TypeExpr::BaseType(BaseType::Unit),
        Literal::Bool(_) => TypeExpr::BaseType(BaseType::Bool),
        Literal::Char(_) => TypeExpr::BaseType(BaseType::Char),
        Literal::I64(_) => TypeExpr::BaseType(BaseType::I64),
        Literal::U64(_) => TypeExpr::BaseType(BaseType::U64),
        Literal::F32(_) => TypeExpr::BaseType(BaseType::F32),
        Literal::F64(_) => TypeExpr::BaseType(BaseType::F64),
        Literal::PString(_) => TypeExpr::BaseType(BaseType::PString),
        Literal::Tuple(x) => TypeExpr::Product(x.into_iter().map(literal_infer).rev().collect()),
    }
}

fn append_error_info(
    callee: Box<dyn FnOnce(Expr, Path) -> CompileResult<TypeExpr>>,
    infoee: Box<dyn FnOnce(CompileError) -> CompileError>,
) -> Box<dyn FnOnce(Expr, Path) -> CompileResult<TypeExpr>> {
    Box::new(|expr, path| match callee(expr, path) {
        Ok(res) => Ok(res),
        Err(err) => Err(infoee(err)),
    })
}

fn expr_infer(context: &Context, expr: Expr, path: Path) -> CompileResult<TypeExpr> {
    match expr {
        Expr::Unknown => Ok(TypeExpr::BaseType(BaseType::Bottom)),
        Expr::Literal(x) => Ok(literal_infer(x)),
        Expr::Variable(name) => match (context.1).0.borrow().get(&name) {
            Some(x) => unimplemented!(),
            None => Err(CompileError(path, ErrorInfo::from(TYPE_ASSERT_ERR))),
        },
        Expr::TypeAssert(e, t) => {
            let ty1 = expr_infer(context, *e, new_path_level(&path, 0))?;
            let ty2 = *t;
            if unifier(&ty1, &ty2) {
                Ok(ty2)
            } else {
                Err(CompileError(path, ErrorInfo::from(VARIABLE_IS_NOT_DEFINED)))
            }
        }
        Expr::Call(callee, parameter) => {
            let funtp = expr_infer(context, *callee, new_path_level(&path, 0))?;
            match funtp {
                TypeExpr::TypeArrow(arrow) => unimplemented!(),
                TypeExpr::TConst(_) => unimplemented!(),
                TypeExpr::TypeVar(_) => unimplemented!(),
                _ => Err(CompileError(
                    path,
                    ErrorInfo::from(CALLEE_IS_NOT_FUNCTION_TYPE),
                )),
            }
        }
        Expr::Lambda(_, _) => unimplemented!(),
        Expr::NamedFun(_) => unimplemented!(),
        Expr::Let(_, _) => unimplemented!(),
        Expr::LetIn(_, _, _) => unimplemented!(),
        Expr::UseIn(_, _, _) => unimplemented!(),
        Expr::IfThenElseIfElse(_, _) => unimplemented!(),
    }
}
