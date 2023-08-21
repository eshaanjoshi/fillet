#[derive(Debug, Clone, PartialEq)]
pub enum LiteralData {
    NUM(i32),
    FLOAT(f32),
    STR(String),
    BOOL(bool),
    NONE,
}

#[derive(PartialEq, Debug, Clone, Copy)]
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
