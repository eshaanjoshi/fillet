#[allow(non_camel_case_types)]
use crate::expres;
use crate::expres::Stmt;
use crate::expres::Symbol;
use crate::token_handler;
use std::process::exit;
//use expres::token_handler;
use expres::Expr;
//use token_handler::built_in::token_enums::LiteralData;
//use token_handler::built_in::token_enums::Tokentype;
use crate::token_enums::LiteralData;
use crate::token_enums::Tokentype;
//use self::expres::token_handler::error_handler;
//use token_handler::error_handler;
use crate::error_handler;
///Parser struct
pub struct Parser {
    token_list: Vec<token_handler::Token>,
    current: usize,
}
///Gets BinaryOpTy from TokenType
fn binary_parser(op: token_handler::Token) -> expres::BinaryOpTy {
    match op.t_type {
        Tokentype::EQUAL_EQUAL => return expres::BinaryOpTy::EqualEqual,
        Tokentype::BANG_EQUAL => return expres::BinaryOpTy::NotEqual,
        Tokentype::LESS => return expres::BinaryOpTy::Less,
        Tokentype::LESS_EQUAL => return expres::BinaryOpTy::LessEqual,
        Tokentype::GREATER => return expres::BinaryOpTy::Greater,
        Tokentype::GREATER_EQUAL => return expres::BinaryOpTy::GreaterEqual,
        Tokentype::PLUS => return expres::BinaryOpTy::Plus,
        Tokentype::MIN => return expres::BinaryOpTy::Minus,
        Tokentype::STAR => return expres::BinaryOpTy::Star,
        Tokentype::FSLASH => return expres::BinaryOpTy::Slash,
        _ => exit(1),
    }
}
///Gets UnaryOpTy from Tokentype
fn unary_parser(op: token_handler::Token) -> expres::UnaryOpTy {
    match op.t_type {
        Tokentype::BANG => return expres::UnaryOpTy::Bang,
        Tokentype::MIN => return expres::UnaryOpTy::Minus,
        _ => exit(1),
    }
}
///Parser implementation. Uses a hierarchical organization to convert from a list of tokens to an asbtract syntax tree
impl Parser {
    ///generates new parser type
    fn new(tok_list: Vec<token_handler::Token>) -> Parser {
        return Parser {
            token_list: tok_list,
            current: 0,
        };
    }

    fn assignment(&mut self)->Expr{
        let expr = self.equality();
        if self.match_type(&vec![Tokentype::EQUAL]){
            //self.advance();
            let value = self.assignment();
            match expr {
                Expr::Variable(sym) =>{
                    return Expr::Assign(sym, Box::new(value));
                },
                _=>{
                    
                    //error(equals.line, "Invalid assignmen target.".to_string());
                },
            }
            
        }
        return expr;
    }
    ///an expression collapses to equality
    fn expression(&mut self) -> Expr {
        //return self.equality();
        return self.assignment();
    }
    ///generates binary expression from a left and right expression and an operator
    fn gen_binary(&self, expr: Expr, op: token_handler::Token, right: Expr) -> Expr {
        let val: &usize = &op.line;
        return Expr::Binary(
            Box::new(expr),
            expres::BinaryOp {
                ty: binary_parser(op.clone()),
                line: *val,
                col: 0,
            },
            Box::new(right),
        );
    }
    ///Collapses equality operators into an expression
    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();
        let mut vec: Vec<Tokentype> = Vec::new();
        vec.push(Tokentype::BANG_EQUAL);
        vec.push(Tokentype::EQUAL_EQUAL);
        while self.match_type(&vec) {
            let op: token_handler::Token = self.previous();
            let right: Expr = self.comparison();
            expr = self.gen_binary(expr, op, right);
        }
        return expr;
    }
    ///collapses comparison operators into an expression
    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();
        let vec = vec![
            Tokentype::GREATER,
            Tokentype::GREATER_EQUAL,
            Tokentype::LESS,
            Tokentype::LESS_EQUAL,
        ];
        while self.match_type(&vec) {
            let op = self.previous();
            let right = self.term();
            expr = self.gen_binary(expr, op, right);
        }
        return expr;
    }
    ///collapses terminal operators into an expression
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        let vec = vec![Tokentype::MIN, Tokentype::PLUS];
        while self.match_type(&vec) {
            let op = self.previous();
            let right = self.factor();
            expr = self.gen_binary(expr, op, right);
        }
        return expr;
    }
    ///collapses factor operators into an expression
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        let vec = vec![Tokentype::FSLASH, Tokentype::STAR];
        while self.match_type(&vec) {
            let op = self.previous();
            let right = self.unary();
            expr = self.gen_binary(expr, op, right);
        }
        return expr;
    }
    ///collapses unary operators into an expression
    fn unary(&mut self) -> Expr {
        let vec = vec![Tokentype::BANG, Tokentype::MIN];
        if self.match_type(&vec) {
            let op = self.previous();
            let right = self.unary();
            let val: &usize = &op.line;
            return Expr::Unary(
                expres::UnaryOp {
                    ty: unary_parser(op.clone()),
                    line: *val,
                    col: 0,
                },
                Box::new(right),
            );
        }
        return self.primary();
    }
    ///collapses primitives into primitive expressions
    fn primary(&mut self) -> Expr {
        if self.match_type(&vec![Tokentype::FALSE]) {
            return Expr::Literal(LiteralData::BOOL(false));
        }
        if self.match_type(&vec![Tokentype::TRUE]) {
            return Expr::Literal(LiteralData::BOOL(true));
        }
        if self.match_type(&vec![Tokentype::NIL]) {
            return Expr::Literal(LiteralData::NONE);
        }
        if self.match_type(&vec![Tokentype::NUMBER, Tokentype::STRING]) {
            return Expr::Literal(self.previous().literal);
        }
        if self.match_type(&vec![Tokentype::LEFTP]) {
            let expr = self.expression();
            self.consume(Tokentype::RIGHTP, "Missing )!!!".to_string());
            return Expr::Grouping(Box::new(expr));
        }
        if self.match_type(&vec![Tokentype::IDENTIFIER]){
            let val = self.previous();
            return Expr::Variable(Symbol { name: val.lexeme, line: val.line, col: 0 });
        }
        return Expr::Useless;
    }
    ///consumes current token in tokenlist. Used to match parens and brackets
    fn consume(&mut self, ty: Tokentype, message: String) -> token_handler::Token {
        if self.check(ty) {
            return self.advance();
        }
        error_handler::error(0, message);
        return token_handler::useless_token();
    }
    ///look at tokenlist without consuming
    fn peek(&self) -> token_handler::Token {
        return (*self.token_list.get(self.current).unwrap()).clone();
    }
    ///get previous token from token list
    fn previous(&mut self) -> token_handler::Token {
        return (*self.token_list.get(self.current - 1).unwrap()).clone();
    }
    ///Check if token is expected
    fn check(&mut self, tok: Tokentype) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().t_type == tok;
    }
    ///advances to the next token in the tokenlist
    fn advance(&mut self) -> token_handler::Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    ///checks if end of tokenlist
    fn is_at_end(&self) -> bool {
        return self.peek().t_type == Tokentype::EOF;
    }
    ///match current token with a list of tokens
    fn match_type(&mut self, vec: &Vec<Tokentype>) -> bool {
        for tok in (*vec).iter() {
            if self.check(*tok) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    ///synchronize upon finishing an expression. Looking for semicolon
    fn _synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().t_type == Tokentype::SEMI {
                return;
            }
            match self.peek().t_type {
                Tokentype::CLASS
                | Tokentype::FUN
                | Tokentype::VAR
                | Tokentype::FOR
                | Tokentype::IF
                | Tokentype::WHILE
                | Tokentype::PRINT
                | Tokentype::RETURN => return,
                _ => (),
            }
            self.advance();
        }
    }
    fn print(&mut self)->Stmt{
        self.advance();
        let value = self.expression();

        let err = self.consume(Tokentype::SEMI, "Expect ; at end of expression.".to_string());
        match err.t_type{
            Tokentype::USELESS => exit(1),
            _=>(),
        }
        return Stmt::Print(value);
    }

    fn block(&mut self)->Stmt{
        return Stmt::Block(self.block_stmt());
    }

    fn block_stmt(&mut self)->Vec<Stmt>{
        let mut values: Vec<Stmt> = Vec::new();
        self.advance();
        while !self.check(Tokentype::RIGHTB)&&!self.is_at_end(){
            values.push(self.declaration());
        }
        self.consume(Tokentype::RIGHTB, "Expect } after block.".to_string());
        println!("hello??");
        println!("{:?}", values);
        return values;
    }
    fn statement(&mut self)->Stmt{
        println!("stmt");
        match self.peek().t_type{
            Tokentype::PRINT => return self.print(),
            Tokentype::LEFTB => return self.block(),
            _=>self.expression_stmt(),
        }
    }
    fn declaration(&mut self) -> Stmt{
       match self.peek().t_type{
            Tokentype::VAR => {return self.var_declaration();},
            _=>{();},

        }
        return self.statement();

    }
    fn var_declaration(&mut self) -> Stmt{
        self.advance();
        let val = self.consume(Tokentype::IDENTIFIER, "Expecting Variable Name.".to_string());
        let tok = Symbol{name: val.lexeme, line:val.line, col:0};
        let mut init = Expr::Useless;

        if self.match_type(&vec![Tokentype::EQUAL]){
            init = self.expression();
        }
        self.consume(Tokentype::SEMI, "Expect ; at end of expression".to_string());
        return Stmt::VarDecl(tok, Some(init));
    }
    fn expression_stmt(&mut self)->Stmt{
        //self.advance();
        let value = self.expression();
        self.consume(Tokentype::SEMI, "Expect ; at end of expression.".to_string());
        return Stmt::Expr(value);
    }
    fn parse_statement(&mut self) -> Vec<Stmt>{
        let mut stmt_list:Vec<Stmt> = Vec::new();
        while !self.is_at_end(){
            stmt_list.push(self.declaration());
        }


        return stmt_list;
    }
}
///Publiicizes token_list parser
pub fn parse_token_list(token_list: &mut Vec<token_handler::Token>) -> Expr {
    let mut p = Parser::new(token_list.clone());
    return p.expression();
}
pub fn parse_stmt(token_list: &mut Vec<token_handler::Token>) -> Vec<Stmt> {
    let mut p = Parser::new(token_list.clone());
    return p.parse_statement();
}




