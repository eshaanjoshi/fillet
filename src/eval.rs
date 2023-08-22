use crate::expres::Expr;
use crate::token_enums::LiteralData;
use std::process::exit;
use crate::accepter;
pub fn evaluate(expr:Expr)->LiteralData{
    println!("Eval: {:?}", expr);
    return accepter::accept(expr);
}
pub fn literal(expr:Expr) -> LiteralData
{
    if let Expr::Literal(test) = expr {
        return test;
    }
    else{
        exit(1);
    }
}

pub fn grouping(expr:Expr)->LiteralData{
    if let Expr::Grouping(test) = expr{
        return evaluate(*test);
    }
    else{
        exit(1);
    }
}

pub fn interpret(expr:Expr) -> LiteralData
{
    return evaluate(expr);
}