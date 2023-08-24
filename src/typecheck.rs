use crate::{expres::{Stmt, Expr, UnaryOp, BinaryOp, UnaryOpTy, BinaryOpTy}, token_enums::LiteralData, error_handler::error};

fn unary(op:UnaryOp, expr:Expr)->Option<LiteralData>{
    let val = typecheck_expr(expr);
    match val{
        None => None,
        Some(data) => check_unary(op, data),
    }
}

fn check_unary(op:UnaryOp, data:LiteralData)-> Option<LiteralData>{
    match op.ty{
        UnaryOpTy::Minus =>{
            match data{
                LiteralData::FLOAT(_) | LiteralData::NUM(_) => return Some(data),
                _=>{
                    error(op.line, "Mismatched Types".to_string());
                    return None
                },
            }
        }
        UnaryOpTy::Bang =>{
            match data{
                LiteralData::BOOL(_) => return Some(data),
                _=>{
                    error(op.line, "Mismatched Types".to_string());
                    return None
                },
            }
        }
    }
}

fn check_binary(op:BinaryOp, left:LiteralData, right:LiteralData)->Option<LiteralData>{
    match op.ty{
        BinaryOpTy::Minus | BinaryOpTy::Slash | BinaryOpTy::Star | BinaryOpTy::Greater
        | BinaryOpTy::GreaterEqual
        | BinaryOpTy::Less
        | BinaryOpTy::LessEqual =>{
            match (left, right){
                (LiteralData::NUM(_), LiteralData::NUM(_)) => Some(LiteralData::NUM(0)),
                (LiteralData::FLOAT(_), LiteralData::FLOAT(_)) => Some(LiteralData::FLOAT(0.0)),
                _=>{
                    error(op.line, "Mismatched Types".to_string());
                    return None
                },
            }
        }
        BinaryOpTy::Plus =>{
            match (left, right){
                (LiteralData::NUM(_), LiteralData::NUM(_)) => Some(LiteralData::NUM(0)),
                (LiteralData::FLOAT(_), LiteralData::FLOAT(_)) => Some(LiteralData::FLOAT(0.0)),
                (LiteralData::STR(_), LiteralData::STR(_)) => Some(LiteralData::STR("".to_string())),
                _=>{
                    error(op.line, "Mismatched Types".to_string());
                    return None
                },
            }
        }
        BinaryOpTy::EqualEqual
        | BinaryOpTy::NotEqual =>{
                match (left, right){
                    (LiteralData::NUM(_), LiteralData::NUM(_)) => Some(LiteralData::NUM(0)),
                    (LiteralData::FLOAT(_), LiteralData::FLOAT(_)) => Some(LiteralData::FLOAT(0.0)),
                    (LiteralData::BOOL(_), LiteralData::BOOL(_)) => Some(LiteralData::BOOL(false)),
                    _=>{
                        error(op.line, "Mismatched Types".to_string());
                        return None
                    },
                }
            }
        
    }
}

fn binary(left:Expr, op:BinaryOp, right:Expr)->Option<LiteralData>{
    let left_val = typecheck_expr(left);
    let right_val = typecheck_expr(right);
    match (left_val, right_val){
        (Some(l), Some(r)) => check_binary(op, l, r),
        _=>{
            error(op.line, "Mismatched Types".to_string());
            return None
        },
    }
}


pub fn typecheck_expr(expr:Expr)->Option<LiteralData>{
    match expr{
        Expr::Literal(expr) => return Some(expr),
        Expr::Unary(op, expr) => {return unary(op, *expr);},
        Expr::Binary(left, op, right) => {return binary(*left, op, *right);}
        Expr::Grouping(expr) => typecheck_expr(*expr),
        //Expr::Variable()
        _=>None,
    }
}

pub fn typecheck_stmt(stmt:Stmt) -> Option<LiteralData>{
    match stmt{
        Stmt::Print(expr) => return typecheck_expr(expr),
        Stmt::Expr(expr) => return typecheck_expr(expr),
        _=>return None,
    }
}

