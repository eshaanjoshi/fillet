#[allow(non_camel_case_types)]
pub mod expres;

use std::process::exit;

use expres::token_handler;
use expres::Expr;
use token_handler::built_in::token_enums::LiteralData;
use token_handler::built_in::token_enums::Tokentype;

use self::expres::token_handler::error_handler;
///Parser struct
pub struct Parser {
    token_list: Vec<token_handler::token>,
    current: usize,
}
///Gets BinaryOpTy from TokenType
fn binary_parser(op: token_handler::token) -> expres::BinaryOpTy {
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
fn unary_parser(op: token_handler::token) -> expres::UnaryOpTy {
    match op.t_type {
        Tokentype::BANG => return expres::UnaryOpTy::Bang,
        Tokentype::MIN => return expres::UnaryOpTy::Minus,
        _ => exit(1),
    }
}
///Parser implementation. Uses a hierarchical organization to convert from a list of tokens to an asbtract syntax tree
impl Parser {
    ///generates new parser type
    fn new(tok_list: Vec<token_handler::token>) -> Parser {
        return Parser {
            token_list: tok_list,
            current: 0,
        };
    }
    ///an expression collapses to equality
    fn expression(&mut self) -> Expr {
        return self.equality();
    }
    ///generates binary expression from a left and right expression and an operator
    fn gen_binary(&self, expr: Expr, op: token_handler::token, right: Expr) -> Expr {
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
            let op: token_handler::token = self.previous();
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
        return Expr::Useless;
    }
    ///consumes current token in tokenlist. Used to match parens and brackets
    fn consume(&mut self, ty: Tokentype, message: String) -> token_handler::token {
        if self.check(ty) {
            return self.advance();
        }
        error_handler::error(0, message);
        return token_handler::useless_token();
    }
    ///look at tokenlist without consuming
    fn peek(&self) -> token_handler::token {
        return (*self.token_list.get(self.current).unwrap()).clone();
    }
    ///get previous token from token list
    fn previous(&mut self) -> token_handler::token {
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
    fn advance(&mut self) -> token_handler::token {
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
    fn synchronize(&mut self) {
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
}
///Publiicizes token_list parser
pub fn parse_token_list(token_list: &mut Vec<token_handler::token>) -> Expr {
    let mut p = Parser::new(token_list.clone());
    return p.expression();
}
