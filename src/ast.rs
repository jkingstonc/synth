use std::collections::HashMap;

use crate::{token::Token, types::Type};

#[derive(Debug)]
pub enum Qualifier {
    CONST,
    VAR,
}

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
pub struct Decl<'a> {
    pub identifier: String,
    pub qualifier: Qualifier,
    pub typ: Option<Type>,
    // todo types
    // pub typ: Type,
    pub requires_infering: bool,
    pub value: Option<Box<ParsedAST<'a>>>,
}

#[derive(Debug)]
pub struct If<'a> {
    pub condition: Box<ParsedAST<'a>>,
    pub body: Box<ParsedAST<'a>>,
    pub else_body: Option<Box<ParsedAST<'a>>>,
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
pub enum ExpressionInstructionEnum {
    COMP,
}

#[derive(Debug)]
pub struct ExpressionInstruction<'a> {
    pub instr: ExpressionInstructionEnum,
    pub rhs: Box<ParsedAST<'a>>,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub new_scope: bool,
    pub body: Vec<ParsedAST<'a>>,
}

#[derive(Debug)]
pub struct Fun<'a> {
    // pub typ: Type,
    pub identifier: Option<String>,
    pub params: Vec<Decl<'a>>,
    pub body: Box<ParsedAST<'a>>,
}

#[derive(Debug)]
pub enum LeftUnary<'a> {
    COMP(Box<ParsedAST<'a>>),
}

#[derive(Debug)]
pub struct Typ {
    pub fields: HashMap<String, Type>,
    pub anon_name: Option<String>,
}

// todo this should be a struct so we get positional information
#[derive(Debug)]
pub enum ParsedAST<'a> {
    PROGRAM(Program<'a>),
    STMT(Box<ParsedAST<'a>>),
    EXPRESSION_INSTRUCTION(ExpressionInstruction<'a>),
    BLOCK(Block<'a>),
    IF(If<'a>),
    // FOR(For<'a>),
    // RET(Option<Box<ParsedAST<'a>>>),
    DECL(Decl<'a>),
    ASSIGN(Assign<'a>),
    IDENTIFIER(String),
    STRING(String),
    FN(Fun<'a>),
    NUMBER(Number),
    LEFT_UNARY(LeftUnary<'a>),
    BINARY(Binary<'a>),
    // GROUP(Group<'a>),
    CALL(Call<'a>),
    TYPE(Typ),
    // STRUCT_TYPES_LIST(StructTypesList<'a>),
    LHS_ACCESS(LhsAccess<'a>),
    // DIRECTIVE(Directive<'a>),
}
