use crate::accepter;
use crate::error_handler;
use crate::expres::Expr;
use crate::expres::Stmt;
use crate::token_enums::LiteralData;
use crate::environ::EnvDefinitions;
use crate::typecheck;

pub struct TaggedStmt{
    stmt: Stmt,
    valid: bool,
}

pub fn interpret(expr: Expr, environ:&mut EnvDefinitions) -> LiteralData {
    return accepter::evaluate(expr, environ);
}


pub fn eval_stmt_list(stmt_list:Vec<Stmt>, environ:&mut EnvDefinitions){
    for stmt in stmt_list{
        accepter::execute(stmt, environ);
    }
}

pub fn eval_typechecked_list(stmt_list:Vec<TaggedStmt>, environ:&mut EnvDefinitions){
    for stmt in stmt_list{
        if stmt.valid
        {
            accepter::execute(stmt.stmt, environ);
        }
        else{
            eprintln!("Skipping statement {:?} due to a reported error.", stmt.stmt);
        }
    }  
}

pub fn typechecker(stmt_list:Vec<Stmt>)->Vec<TaggedStmt>{
    let mut tagged: Vec<TaggedStmt> = Vec::new();
    for stmt in &stmt_list{
        match typecheck::typecheck_stmt(stmt.clone()){
            None => {{
                error_handler::type_error();
                //exit(1);
            }; tagged.push(TaggedStmt { stmt: stmt.clone(), valid: false })}
            Some(_)=>{
                tagged.push(TaggedStmt { stmt: stmt.clone(), valid: true })
            }
        }
    }
    return tagged;
}