use crate::mata_info::ast::*;

const TYPE_ASSERT_ERR: RErrorInfo = ("T0", "Expression type assertion error");
const CALLEE_IS_NOT_FUNCTION_TYPE: RErrorInfo = ("T1", "Callee is not a function type");
const VARIABLE_IS_NOT_DEFINED: RErrorInfo = ("T2", "Variable is not defined");
const UNIFICATION_ERROR: &'static str = "T3";

fn new_path_level(path: &Path, pos: u64) -> Path {
    let mut res = path.0.clone();
    res.push(pos);
    return Path(res);
}

fn make_unification_error(i: String) -> Result<(), ErrorInfo> {
    Err(ErrorInfo(UNIFICATION_ERROR.to_string(), i))
}

fn unifier(ty1: &TypeExpr, ty2: &TypeExpr) -> Result<(), ErrorInfo> {
    match (ty1, ty2) {
        (TypeExpr::BaseType(x), TypeExpr::BaseType(y)) => {
            if x == y {
                Ok(())
            } else {
                make_unification_error(format!(
                    "type {} and type {} cannot be unified",
                    x.to_string(),
                    y.to_string()
                ))
            }
        }
        (TypeExpr::TConst(x), TypeExpr::TConst(y)) => {
            if x == y {
                Ok(())
            } else {
                make_unification_error(format!("type {} and type {} cannot be unified", x, y))
            }
        }
        // (TypeExpr::)
        _ => Ok(()),
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

/*
fn append_error_info(
    callee: Box<dyn FnOnce(Expr, Path) -> CompileResult<TypeExpr>>,
    infoee: Box<dyn FnOnce(CompileError) -> CompileError>,
) -> Box<dyn FnOnce(Expr, Path) -> CompileResult<TypeExpr>> {
    Box::new(|expr, path| match callee(expr, path) {
        Ok(res) => Ok(res),
        Err(err) => Err(infoee(err)),
    })
}
*/

fn expr_infer(context: &Context, expr: Expr, path: Path) -> CompileResult<TypeExpr> {
    match expr {
        Expr::Unknown => Ok(TypeExpr::BaseType(BaseType::Bottom)),
        Expr::Literal(x) => Ok(literal_infer(x)),
        Expr::Variable(name) => match (context.1).0.borrow().get(&name) {
            Some(x) => Ok(x.clone()),
            None => Err(vec![CompileError(
                path,
                ErrorInfo::from(VARIABLE_IS_NOT_DEFINED),
            )]),
        },
        Expr::TypeAssert(e, t) => {
            let ty1 = expr_infer(context, *e, new_path_level(&path, 0))?;
            let ty2 = *t;
            if let Ok(_) = unifier(&ty1, &ty2) {
                Ok(ty2)
            } else {
                Err(vec![CompileError(path, ErrorInfo::from(TYPE_ASSERT_ERR))])
            }
        }
        Expr::Call(callee, parameter) => {
            let funtp = expr_infer(context, *callee, new_path_level(&path, 0))?;
            let argtp = parameter
                .into_iter()
                .enumerate()
                .map(|(i, e)| expr_infer(&context, e, new_path_level(&path, i as u64 + 1)));
            let infer_errlog = argtp
                .clone()
                .filter(|e| if let Err(_) = e { true } else { false })
                .map(|e| e.unwrap_err())
                .fold(vec![], |mut res, mut v| {
                    res.append(&mut v);
                    res
                });
            if infer_errlog.len() > 0 {
                return Err(infer_errlog);
            }
            let argtp = TypeExpr::TypeArrow(argtp.map(|e| e.unwrap()).collect::<Vec<_>>());
            match unifier(&funtp, &argtp) {
                Ok(res) => unimplemented!(),
                Err(e) => Err(vec![CompileError(path, e)]),
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
