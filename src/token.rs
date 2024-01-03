// todo add position to tokens

#[derive(Debug, Eq, PartialEq, Clone)]
#[warn(dead_code)]
pub enum Token {
    END,

    DOLLAR,

    AT,
    HASH,

    PLUS,
    MINUS,
    STAR,
    DIV,

    LCURLY,
    RCURLY,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,

    DOT,
    COMMA,
    COLON,
    SEMICOLON,

    EQUAL,

    NUMBER(std::string::String),
    STRING(std::string::String),
    IDENTIFIER(std::string::String),

    VAR,

    MUT,
    CONST,

    PUB,
    PRIV,

    U32,
    I32,
    F32,
    BOOL,
    FN,
    TYPE,

    TRUE,
    FALSE,

    IF,
    ELSE,
    FOR,

    RET,
}

pub struct Position {
    col_start: u32,
    col_end: u32,
    line_start: u32,
    line_end: u32,
}
