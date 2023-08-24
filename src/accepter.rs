use std::process::exit;
use std::sync::Mutex;

use crate::environ::EnvDefinitions;
use crate::error_handler;
use crate::error_handler::error;
use crate::error_handler::fatal_error;
use crate::expres::BinaryOpTy;
use crate::expres::Expr;
use crate::expres::Stmt;
use crate::expres::Symbol;
use crate::expres::UnaryOpTy;
use crate::token_enums::LiteralData;
use crate::environ;

lazy_static! {
    ///global keyword dictionary
    static ref ENVIR: Mutex<environ::EnvDefinitions> = Mutex::new(environ::EnvDefinitions::new());
}


fn mathint(x: LiteralData, y: LiteralData, f: fn(i32, i32) -> i32) -> LiteralData {
    let x_int;
    let y_int;
    if let LiteralData::NUM(_x_int) = x {
        x_int = _x_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    if let LiteralData::NUM(_y_int) = y {
        y_int = _y_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    return LiteralData::NUM(f(x_int, y_int));
}

fn mathfloat(x: LiteralData, y: LiteralData, f: fn(f32, f32) -> f32) -> LiteralData {
    let x_int;
    let y_int;
    if let LiteralData::FLOAT(_x_int) = x {
        x_int = _x_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    if let LiteralData::FLOAT(_y_int) = y {
        y_int = _y_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    return LiteralData::FLOAT(f(x_int, y_int));
}
fn strcat(x: LiteralData, y: LiteralData, f: fn(String, String) -> String) -> LiteralData {
    let x_int;
    let y_int;
    if let LiteralData::STR(_x_int) = x {
        x_int = _x_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    if let LiteralData::STR(_y_int) = y {
        y_int = _y_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    return LiteralData::STR(f(x_int, y_int));
}
fn compint(left: LiteralData, right: LiteralData, f: fn(a: i32, b: i32) -> bool) -> LiteralData {
    let x_int;
    let y_int;
    if let LiteralData::NUM(_x_int) = left {
        x_int = _x_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    if let LiteralData::NUM(_y_int) = right {
        y_int = _y_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    return LiteralData::BOOL(f(x_int, y_int));
}

fn compfloat(left: LiteralData, right: LiteralData, f: fn(a: f32, b: f32) -> bool) -> LiteralData {
    let x_int;
    let y_int;
    if let LiteralData::FLOAT(_x_int) = left {
        x_int = _x_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    if let LiteralData::FLOAT(_y_int) = right {
        y_int = _y_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    return LiteralData::BOOL(f(x_int, y_int));
}

fn compbool(left: LiteralData, right: LiteralData, f: fn(a: bool, b: bool) -> bool) -> LiteralData {
    let x_int;
    let y_int;
    if let LiteralData::BOOL(_x_int) = left {
        x_int = _x_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    if let LiteralData::BOOL(_y_int) = right {
        y_int = _y_int
    } else {
        fatal_error("INTERPRETER".to_string(), "Typechecking failed".to_string(), 0);
        exit(1);
    }
    return LiteralData::BOOL(f(x_int, y_int));
}

fn comp_op(left: LiteralData, right: LiteralData, op: BinaryOpTy) -> LiteralData {
    match op {
        BinaryOpTy::Greater => match left {
            LiteralData::NUM(_) => return compint(left, right, |a: i32, b: i32| a > b),
            LiteralData::FLOAT(_) => return compfloat(left, right, |a: f32, b: f32| a > b),
            _ => exit(1),
        },
        BinaryOpTy::GreaterEqual => match left {
            LiteralData::NUM(_) => return compint(left, right, |a: i32, b: i32| a >= b),
            LiteralData::FLOAT(_) => return compfloat(left, right, |a: f32, b: f32| a > b),
            _ => exit(1),
        },
        BinaryOpTy::Less => match left {
            LiteralData::NUM(_) => return compint(left, right, |a: i32, b: i32| a < b),
            LiteralData::FLOAT(_) => return compfloat(left, right, |a: f32, b: f32| a > b),
            _ => exit(1),
        },
        BinaryOpTy::LessEqual => match left {
            LiteralData::NUM(_) => return compint(left, right, |a: i32, b: i32| a <= b),
            LiteralData::FLOAT(_) => return compfloat(left, right, |a: f32, b: f32| a > b),
            _ => exit(1),
        },
        BinaryOpTy::NotEqual => match left {
            LiteralData::NUM(_) => return compint(left, right, |a: i32, b: i32| a != b),
            LiteralData::FLOAT(_) => return compfloat(left, right, |a: f32, b: f32| a != b),
            LiteralData::BOOL(_) => return compbool(left, right, |a: bool, b: bool| a != b),
            _ => exit(1),
        },
        BinaryOpTy::EqualEqual => match left {
            LiteralData::NUM(_) => return compint(left, right, |a: i32, b: i32| a == b),
            LiteralData::FLOAT(_) => return compfloat(left, right, |a: f32, b: f32| a == b),
            LiteralData::BOOL(_) => return compbool(left, right, |a: bool, b: bool| a == b),
            _ => exit(1),
        },
        _ => exit(1),
    }
}

fn binary(expr: Expr, environ:&mut EnvDefinitions) -> LiteralData {
    if let Expr::Binary(left, op, right) = expr {
        let left_data = evaluate(*left, environ);
        let right_data = evaluate(*right, environ);
        match op.ty {
            BinaryOpTy::Minus => match left_data {
                LiteralData::NUM(_) => {
                    return mathint(left_data, right_data, |a: i32, b: i32| a - b)
                }
                LiteralData::FLOAT(_) => {
                    return mathfloat(left_data, right_data, |a: f32, b: f32| a - b)
                }
                _ => exit(1),
            },
            BinaryOpTy::Slash => match left_data {
                LiteralData::NUM(_) => {
                    return mathint(left_data, right_data, |a: i32, b: i32| a / b)
                }
                LiteralData::FLOAT(_) => {
                    return mathfloat(left_data, right_data, |a: f32, b: f32| a / b)
                }
                _ => exit(1),
            },
            BinaryOpTy::Star => match left_data {
                LiteralData::NUM(_) => {
                    return mathint(left_data, right_data, |a: i32, b: i32| a * b)
                }
                LiteralData::FLOAT(_) => {
                    return mathfloat(left_data, right_data, |a: f32, b: f32| a * b)
                }
                _ => exit(1),
            },
            BinaryOpTy::Plus => match left_data {
                LiteralData::NUM(_) => {
                    return mathint(left_data, right_data, |a: i32, b: i32| a + b)
                }
                LiteralData::FLOAT(_) => {
                    return mathfloat(left_data, right_data, |a: f32, b: f32| a + b)
                }
                LiteralData::STR(_) => {
                    return strcat(left_data, right_data, |a: String, b: String| {
                        format!("{}{}", a, b)
                    })
                }
                _ => exit(1),
            },
            BinaryOpTy::EqualEqual
            | BinaryOpTy::NotEqual
            | BinaryOpTy::Greater
            | BinaryOpTy::GreaterEqual
            | BinaryOpTy::Less
            | BinaryOpTy::LessEqual => return comp_op(left_data, right_data, op.ty),
            //_ => return LiteralData::NONE,
        }
    } else {
        exit(1);
    }
}

fn truthy(a: LiteralData) -> bool {
    match a {
        LiteralData::BOOL(f) => return f,
        LiteralData::NONE => return false,
        _ => return true,
    }
}

fn unary(expr: Expr, environ:&mut EnvDefinitions) -> LiteralData {
    if let Expr::Unary(op, expr) = expr {
        let val = evaluate(*expr, environ);
        match op.ty {
            UnaryOpTy::Minus => match val {
                LiteralData::FLOAT(f) => return LiteralData::FLOAT(-f),
                LiteralData::NUM(f) => return LiteralData::NUM(-f),
                _ => exit(1),
            },
            UnaryOpTy::Bang => match val {
                LiteralData::BOOL(_) => return LiteralData::BOOL(!truthy(val)),
                _ => exit(1),
            },
        }
    } else {
        exit(1);
    }
}
pub fn evaluate(expr: Expr, environ:&mut EnvDefinitions) -> LiteralData {
    println!("Eval: {:?}", expr);
    return accept(expr, environ);
}
pub fn literal(expr: Expr) -> LiteralData {
    if let Expr::Literal(test) = expr {
        return test;
    } else {
        exit(1);
    }
}

pub fn grouping(expr: Expr, environ:&mut EnvDefinitions) -> LiteralData {
    if let Expr::Grouping(test) = expr {
        return evaluate(*test, environ);
    } else {
        exit(1);
    }
}

pub fn assign(sym:Symbol, expr:Expr, environ:&mut EnvDefinitions)->LiteralData{
    let value = evaluate(expr, environ);
    let mut hashmap: std::sync::MutexGuard<'_, environ::EnvDefinitions> = ENVIR.lock().unwrap();
    hashmap.define(sym.name, value.clone());
    return value;
}



pub fn accept(expr: Expr, environ:&mut EnvDefinitions) -> LiteralData {
    // -> impl Fn(Expr) -> token_enums::LiteralData{
    match expr {
        Expr::Binary(_, _, _) => return binary(expr, environ),
        Expr::Unary(_, _) => return unary(expr, environ),
        Expr::Literal(_) => return literal(expr),
        Expr::Grouping(_) => return grouping(expr, environ),
        Expr::Variable(_) => return var_expr(expr, environ),
        Expr::Assign(sym, exp) => return assign(sym.clone(), *exp, environ),
        _ => exit(1),
    }
}

pub fn print(expr: Expr, environ:&mut EnvDefinitions){
    let value = evaluate(expr, environ);
    println!("{}", value.string_rep());
}

pub fn var(val: Symbol, expr: Option<Expr>, environ:&mut EnvDefinitions){
    let mut value = LiteralData::NONE;
    match expr{
        Some(expres) => {
            value = evaluate(expres, environ);
            
        }
        None => {},
    }
    //let mut hashmap = ENVIR.lock().unwrap();
    //hashmap.define(val.name, value);
    environ.define(val.name, value);
}

pub fn var_expr(expr:Expr, environ:&mut EnvDefinitions) -> LiteralData{
    let symb: Symbol;
    match expr {
        Expr::Variable(sym) => {symb = sym;}
        _=>exit(1),
    }
    //let mut hashmap = ENVIR.lock().unwrap();
    //let val = hashmap.get_from_string(symb.name);
    let val = environ.get_from_string(symb.name);
    match val{
        None => {error_handler::fatal_error("RUNTIME".to_string(), "Uninitialized Variable".to_string(), symb.line); return LiteralData::NONE},
        _=>{},
    }
    return val.unwrap();
}

pub fn block(stmts:Vec<Stmt>, environ:&mut EnvDefinitions){
    let mut new_env = EnvDefinitions::new_enclosed(environ);
    for stmt in stmts{
        execute(stmt, &mut new_env);
    }
}

pub fn expression(expr:Expr, environ:&mut EnvDefinitions){
    evaluate(expr, environ);
    //assignment(expr);
}


pub fn execute(stmt:Stmt, environ:&mut EnvDefinitions) {
    match stmt{
        Stmt::Print(expr) => print(expr, environ),
        Stmt::Expr(expr)=>expression(expr, environ),
        Stmt::VarDecl(sym, expr) => var(sym, expr, environ),
        Stmt::Block(stmts) => block(stmts, environ),
        _=>exit(1),
    }
}
