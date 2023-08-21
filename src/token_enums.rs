#![allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
///This enum contains the data of the token to be read by the parser. Only contains data for the primitives
pub enum LiteralData {
    NUM(i32),
    FLOAT(f32),
    STR(String),
    BOOL(bool),
    NONE,
}

#[derive(PartialEq, Debug, Clone, Copy)]
/// This enum contains a list of all the different tokens that the parser will assign tokens to.
pub enum Tokentype {
    LEFTP,
    RIGHTP,
    LEFTB,
    RIGHTB,
    COMMA,
    PERIOD,
    MIN,
    PLUS,
    SEMI,
    FSLASH,
    STAR,

    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    IDENTIFIER,
    STRING,
    NUMBER,
    FLOAT,

    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
    USELESS,
}
