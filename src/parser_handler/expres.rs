#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod token_handler;

#[derive(Debug, Clone)]
///enum containing all expressions our parser looks for
pub enum Expr {
    Useless,
    Literal(token_handler::built_in::token_enums::LiteralData),
    This(SourceLocation),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Call(Box<Expr>, SourceLocation, Vec<Expr>),
    Get(Box<Expr>, Symbol),
    Grouping(Box<Expr>),
    Variable(Symbol),
    Assign(Symbol, Box<Expr>),
    Logical(Box<Expr>, LogicalOp, Box<Expr>),
    Set(Box<Expr>, Symbol, Box<Expr>),
    Super(SourceLocation, Symbol),
    List(Vec<Expr>),
    Subscript {
        value: Box<Expr>,
        slice: Box<Expr>,
        source_location: SourceLocation,
    },
    SetItem {
        lhs: Box<Expr>,
        slice: Box<Expr>,
        rhs: Box<Expr>,
        source_location: SourceLocation,
    },
    Lambda(LambdaDecl),
}

#[derive(Debug, Clone, Copy)]
///location struct, currently unused.
pub struct SourceLocation {
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Clone)]
///logical operation enums
pub enum LogicalOp {
    Or,
    And,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
///struct for symbols
pub struct Symbol {
    pub name: String,
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Clone)]
///Struct for function declarations
pub struct FunDecl {
    pub name: Symbol,
    pub params: Vec<Symbol>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
///Struct for lambda declarations
pub struct LambdaDecl {
    pub params: Vec<Symbol>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
///struct for class declarations
pub struct ClassDecl {
    pub name: Symbol,
    pub superclass: Option<Symbol>,
    pub methods: Vec<FunDecl>,
}

#[derive(Debug, Clone)]
///enum of all possible statements our parser can find
pub enum Stmt {
    Expr(Expr),
    FunDecl(FunDecl),
    ClassDecl(ClassDecl),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    VarDecl(Symbol, Option<Expr>),
    Block(Vec<Stmt>),
    Return(SourceLocation, Option<Expr>),
    While(Expr, Box<Stmt>),
}

#[derive(Debug, Copy, Clone)]
///Unary operator type
pub enum UnaryOpTy {
    Minus,
    Bang,
}

#[derive(Debug, Clone)]
///unary operator
pub struct UnaryOp {
    pub ty: UnaryOpTy,
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Copy, Clone)]
///Binary operator type
pub enum BinaryOpTy {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
///Binary operator
pub struct BinaryOp {
    pub ty: BinaryOpTy,
    //pub ty: token_handler::token,
    pub line: usize,
    pub col: i64,
}

#[derive(Debug, Clone)]
///enum of all literal types(all primitives)
pub enum Literal {
    Number(i64),
    Float(f64),
    String(String),
    True,
    False,
    Nil,
}
