use crate::accepter;
use crate::expres::Expr;
use crate::expres::Stmt;
use crate::token_enums::LiteralData;


pub fn interpret(expr: Expr) -> LiteralData {
    return accepter::evaluate(expr);
}


pub fn eval_stmt_list(stmt_list:Vec<Stmt>){
    for stmt in stmt_list{
        accepter::execute(stmt);
    }
}