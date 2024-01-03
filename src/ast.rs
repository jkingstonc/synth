use crate::lex::Token;

#[derive(Debug)]
pub struct Program<'a> {
    // todo this should probably be an array of Box<ParsedAST>
    pub body: Vec<ParsedAST<'a>>,
}

#[derive(Debug)]
pub struct Assign<'a> {
    pub lhs: Box<ParsedAST<'a>>,
    pub rhs: Box<ParsedAST<'a>>,
}

// todo turn this into an enum
#[derive(Debug)]
pub struct Binary<'a> {
    pub left: Box<ParsedAST<'a>>,
    pub op: &'a Token, // todo this should probably be a ref
    pub right: Box<ParsedAST<'a>>,
}

#[derive(Debug)]
pub struct Call<'a> {
    pub callee: Box<ParsedAST<'a>>,
    pub args: Vec<ParsedAST<'a>>,
}

#[derive(Debug)]
pub struct LhsAccess<'a> {
    pub left: Box<ParsedAST<'a>>,
    // todo this should probably be an identifier?
    pub right: Box<ParsedAST<'a>>,
}

#[derive(Debug)]
pub enum Number {
    INTEGER(i32),
    FLOAT(f32),
}

#[derive(Debug)]
pub enum ParsedAST<'a> {
    PROGRAM(Program<'a>),
    STMT(Box<ParsedAST<'a>>),
    // BLOCK(Block<'a>),
    // IF(If<'a>),
    // FOR(For<'a>),
    // RET(Option<Box<ParsedAST<'a>>>),
    // DECL(Decl<'a>),
    ASSIGN(Assign<'a>),
    IDENTIFIER(std::string::String),
    STRING(std::string::String),
    // FN(Fn<'a>),
    NUMBER(Number),
    // LEFT_UNARY(LeftUnary<'a>),
    BINARY(Binary<'a>),
    // GROUP(Group<'a>),
    CALL(Call<'a>),
    // STRUCT_TYPES_LIST(StructTypesList<'a>),
    LHS_ACCESS(LhsAccess<'a>),
    // DIRECTIVE(Directive<'a>),
}
