use std::collections::HashMap;
use std::process::id;
use std::time::Instant;

use log::debug;

use crate::ast::{
    Assign, Binary, Block, Call, Decl, ExpressionInstruction, ExpressionInstructionEnum, Fun, If,
    LeftUnary, LhsAccess, Number, ParsedAST, Program, Qualifier, Typ,
};
use crate::token::Token;
use crate::types::Type;

pub struct Parser<'a> {
    pub tokens: &'a Box<Vec<Token>>,
}

impl Parser<'_> {
    pub fn parse(&mut self) -> Box<ParsedAST> {
        let now = Instant::now();
        let ast = Box::new(self.parse_program());
        let elapsed = now.elapsed();
        debug!(
            "parse time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
        ast
    }

    fn parse_program(&mut self) -> ParsedAST {
        let mut current: usize = 0;
        let mut body: Vec<ParsedAST> = vec![];

        while !self.end(&current) {
            body.push(self.statement(&mut current));
        }

        return ParsedAST::PROGRAM(Program { body: body });
    }

    fn statement(&self, current: &mut usize) -> ParsedAST {
        match self.peek(&current) {
            // Token::LCURLY => self.block(current),
            Token::IF => self.if_stmt(current),
            // Token::FOR => self.for_stmt(current),
            // Token::RET => self.ret(current),
            _ => ParsedAST::STMT(Box::new(self.expression(current))),
        }
    }

    fn if_stmt(&self, current: &mut usize) -> ParsedAST {
        self.consume(current); // consume the if
        let condition = Box::new(self.expression(current));
        let body = Box::new(self.statement(current));
        let mut else_body: Option<Box<ParsedAST>> = None;

        if !self.end(current) && self.expecting(Token::ELSE, current) {
            self.consume(current); // consume the else
            else_body = Some(Box::new(self.statement(current)));
        }

        return ParsedAST::IF(If {
            condition,
            body,
            else_body,
        });
    }

    fn expression(&self, current: &mut usize) -> ParsedAST {
        match self.peek(&current) {
            _ => self.comparison(current),
        }
    }

    fn comparison(&self, current: &mut usize) -> ParsedAST {
        // todo!
        self.decl_or_assign(current)
    }

    fn parse_type(&self, current: &mut usize) -> Type {
        match self.consume(current) {
            Token::I32 => Type::I32,
            Token::TYPE => Type::TYPE,
            Token::IDENTIFIER(i) => Type::STRUCT(i.to_string()),
            _ => panic!(),
        }
    }

    fn decl_or_assign(&self, current: &mut usize) -> ParsedAST {
        // self.assign(current)
        // todo
        let first = self.peek(current);

        if self.end_ahead(current, 1) {
            return self.assign(current);
        }

        // let second = self.peek_ahead(current, 1);

        match first {
            Token::CONST => {
                self.consume(current);
                let identifier: String;
                match self.consume(current) {
                    Token::IDENTIFIER(i) => identifier = i.to_string(),
                    _ => panic!(),
                }

                let mut typ: Option<Type> = None;
                if self.expecting(Token::COLON, current) {
                    // get the type
                    self.consume(current);
                    typ = Some(self.parse_type(current));
                }

                self.consume(current); // consume the =
                let value = self.expression(current);

                debug!("parsing decl type! {:?}", typ);
                return ParsedAST::DECL(Decl {
                    identifier,
                    qualifier: Qualifier::CONST,
                    requires_infering: true,
                    typ: typ,
                    value: Some(Box::new(value)),
                });
            }
            Token::VAR => {
                self.consume(current);
                let identifier: String;
                match self.consume(current) {
                    Token::IDENTIFIER(i) => identifier = i.to_string(),
                    _ => panic!(),
                }

                let mut typ: Option<Type> = None;
                if self.expecting(Token::COLON, current) {
                    // get the type
                    self.consume(current);
                    typ = Some(self.parse_type(current));
                }

                self.consume(current); // consume the =
                let value = self.expression(current);

                return ParsedAST::DECL(Decl {
                    identifier,
                    qualifier: Qualifier::VAR,
                    typ: typ,
                    requires_infering: true,
                    value: Some(Box::new(value)),
                });
            }
            Token::IDENTIFIER(identifier) => {
                match self.peek_ahead(current, 1) {
                    Token::EQUAL => {
                        // doing assign
                        self.consume(current);
                        // consume the =
                        self.consume(current);
                        let rhs = self.expression(current);
                        return ParsedAST::ASSIGN(Assign {
                            lhs: Box::new(ParsedAST::IDENTIFIER(identifier.to_string())),
                            rhs: Box::new(rhs),
                        });
                    }
                    _ => return self.assign(current),
                }
            }
            _ => return self.assign(current),
        }

        // match first {
        //     Token::IDENTIFIER(_) => {
        //         match second {
        //             Token::TYPE => {
        //                 let identifier = self.consume(current);
        //                 let typ = self.parse_type(current);

        //                 let mut value: Option<Box<ParsedAST>> = None;

        //                 // constant
        //                 match self.peek(current) {
        //                     Token::LCURLY => {
        //                         value = Some(Box::new(self.struct_types_list(current)));
        //                     },
        //                     _ => {}
        //                 };

        //                 let mut ident: String;
        //                 match identifier {
        //                     Token::IDENTIFIER(i) => ident = i.to_string(),
        //                     _ => panic!()
        //                 }
        //                 return ParsedAST::DECL(Decl{identifier: ident, typ, requires_infering: false, value})
        //             },
        //             // todo
        //             //  instead of parsing the fn type, just call self.fn() ?
        //             Token::FN => {
        //                 let identifier = self.consume(current);
        //                 let typ = Type { mutability: Mutability::CONSTANT, primative: Primative::INCOMPLETE, reference: false };
        //                 let value = Some(Box::new(self.function(current)));
        //                 let mut ident: String;
        //                 match identifier {
        //                     Token::IDENTIFIER(i) => ident = i.to_string(),
        //                     _ => panic!()
        //                 }
        //                 return ParsedAST::DECL(Decl{identifier: ident, typ, requires_infering: true, value});

        //                 // let typ = self.parse_type(current);

        //                 // let mut value: Option<Box<ParsedAST>> = None;

        //                 // // constant
        //                 // match self.peek(current) {
        //                 //     Token::LCURLY => {
        //                 //         value = Some(Box::new(self.block(current)));
        //                 //     },
        //                 //     _ => {}
        //                 // };

        //                 // return ParsedAST::DECL(Decl{identifier, typ, requires_infering: false, value})
        //             },
        //             // todo we need to match for a type here instead of identifier
        //             Token::AT | Token::VAR | Token::DOLLAR | Token::MUT | Token::CONST | Token::PUB | Token::PRIV | Token::U32 | Token::I32 | Token::F32 | Token::BOOL | Token::IDENTIFIER(_) => {

        //                 let identifier = self.consume(current);
        //                 let typ = self.parse_type(current);

        //                 let mut value: Option<Box<ParsedAST>> = None;

        //                 let mut ident: String;
        //                 match identifier {
        //                     Token::IDENTIFIER(i) => ident = i.to_string(),
        //                     _ => panic!()
        //                 }

        //                 // constant
        //                 match self.peek(current) {
        //                     Token::EQUAL => {
        //                         self.consume(current); // consume the =
        //                         value = Some(Box::new(self.expression(current)))
        //                     },
        //                     _ => {}
        //                 };

        //                 match typ.primative {
        //                     Primative::INCOMPLETE => return ParsedAST::DECL(Decl{identifier: ident, typ, requires_infering: true, value}),
        //                     _ => return ParsedAST::DECL(Decl{identifier: ident, typ, requires_infering: false, value})
        //                 }
        //             },
        //             Token::EQUAL => {

        //                 // todo assign!

        //                 // let identifier = self.consume(current);
        //                 // let typ = Type{mutability: Mutability::CONSTANT, primative: Primative::NONE};
        //                 // self.consume(current); // consume the =
        //                 // let value = self.expression(current);
        //                 // return ParsedAST::DECL(Decl{identifier, typ, requires_infering: true, value: Some(Box::new(value))})
        //                 return self.assign(current);

        //             },
        //             _ => return self.assign(current)
        //         }
        //     },
        //     _ => return self.assign(current)
        // }
    }

    fn assign(&self, current: &mut usize) -> ParsedAST {
        let higher_precedence = self.expression_instructions(current);
        if !self.end(current) {
            if self.expecting(Token::EQUAL, current) {
                self.consume(current);
                let rhs = self.expression_instructions(current);
                return ParsedAST::ASSIGN(Assign {
                    lhs: Box::new(higher_precedence),
                    rhs: Box::new(rhs),
                });
            }
        }
        higher_precedence
    }

    // e.g. comp 1+2
    fn expression_instructions(&self, current: &mut usize) -> ParsedAST {
        // if !self.end(&current) {
        //     if self.expecting(Token::COMP, current) {
        //         self.consume(current);
        //         let rhs = self.plus_or_minus(current);
        //         return ParsedAST::EXPRESSION_INSTRUCTION(ExpressionInstruction {
        //             instr: ExpressionInstructionEnum::COMP,
        //             rhs: Box::new(rhs),
        //         });
        //     }
        // }
        self.plus_or_minus(current)
    }

    fn plus_or_minus(&self, current: &mut usize) -> ParsedAST {
        let higher_precedence = self.mul_or_div(current);

        if !self.end(current) {
            match self.peek(current) {
                Token::PLUS | Token::MINUS => {
                    let token = self.consume(current);
                    let right = self.expression(current);
                    return ParsedAST::BINARY(Binary {
                        left: Box::new(higher_precedence),
                        op: token,
                        right: Box::new(right),
                    });
                }
                _ => return higher_precedence,
            }
        }
        higher_precedence
    }

    fn mul_or_div(&self, current: &mut usize) -> ParsedAST {
        let higher_precedence = self.unary(current);

        if !self.end(current) {
            match self.peek(current) {
                Token::STAR | Token::DIV => {
                    let token = self.consume(current);
                    let right = self.expression(current);
                    return ParsedAST::BINARY(Binary {
                        left: Box::new(higher_precedence),
                        op: token,
                        right: Box::new(right),
                    });
                }
                _ => return higher_precedence,
            }
        }
        higher_precedence
    }

    fn unary(&self, current: &mut usize) -> ParsedAST {
        // todo &some_var
        if self.expecting(Token::COMP, current) {
            self.consume(current);
            let rhs = self.expression(current);
            return ParsedAST::LEFT_UNARY(LeftUnary::COMP(Box::new(rhs)));
        }

        self.call(current)
    }

    fn call(&self, current: &mut usize) -> ParsedAST {
        let higher_presedence = self.struct_access(current);
        if !self.end(current) {
            match self.peek_ahead(current, -1) {
                Token::IDENTIFIER(_) => {
                    if !self.end_ahead(current, 1) {
                        // todo peak_ahead could fail :(
                        match self.peek(current) {
                            Token::LPAREN => {
                                self.consume(current);
                                let mut args: Vec<ParsedAST> = vec![];
                                while !self.expecting(Token::RPAREN, current) {
                                    args.push(self.expression(current));
                                    if !self.expecting(Token::RPAREN, current) {
                                        self.consume(current); // consume the ,
                                    }
                                }
                                self.consume(current); // consume the )
                                return ParsedAST::CALL(Call {
                                    callee: Box::new(higher_presedence),
                                    args,
                                });
                            }
                            _ => return higher_presedence,
                        }
                    }
                }
                _ => return higher_presedence,
            }
        }

        higher_presedence
    }

    fn struct_access(&self, current: &mut usize) -> ParsedAST {
        let higher_precedence = self.single(current);
        if !self.end(current) {
            match self.peek(current) {
                Token::DOT => {
                    self.consume(current); // consume the dot
                    let rhs = self.expression(current);
                    return ParsedAST::LHS_ACCESS(LhsAccess {
                        left: Box::new(higher_precedence),
                        right: Box::new(rhs),
                    });
                }
                _ => return higher_precedence,
            }
        }
        higher_precedence
    }

    fn single(&self, current: &mut usize) -> ParsedAST {
        match self.peek(current) {
            Token::FN => {
                self.consume(current);
                let identifier = self.consume(current);

                // do params

                let mut params: Vec<Decl<'_>> = vec![];
                if self.expecting(Token::LPAREN, current) {
                    self.consume(current);

                    loop {
                        if self.expecting(Token::RPAREN, current) {
                            self.consume(current);
                            break;
                        }
                        // do a decl
                        let identifier = self.consume(current);
                        self.consume(current);
                        let typ = self.parse_type(current);

                        let Token::IDENTIFIER(i) = identifier else {
                            // todo do this more:)
                            panic!("expected identifier");
                        };

                        params.push(Decl {
                            identifier: i.to_string(),
                            qualifier: Qualifier::CONST,
                            typ: Some(typ),
                            requires_infering: false,
                            value: None,
                        });

                        if !self.expecting(Token::RPAREN, current) {
                            // todo we need to verify were consuming the right thing
                            self.consume(current); // consume the ,
                        }
                    }
                }

                if let Token::IDENTIFIER(i) = identifier {
                    return ParsedAST::FN(Fun {
                        identifier: Some(i.to_string()),
                        params: params,
                        body: Box::new(self.statement(current)),
                    });
                }
                panic!("expected identifier");
            }
            Token::TYPE => {
                // we have a type definition!
                // consume the type
                self.consume(current);
                // consume the {
                self.consume(current);
                let mut fields: HashMap<String, Type> = HashMap::new();

                while !self.expecting(Token::RCURLY, current) {
                    let identifier = self.consume(current);

                    match identifier {
                        Token::IDENTIFIER(i) => {
                            // consume the :
                            // todo we need a method to consume a token and crash if we dont find it
                            self.consume_expected(current, Token::COLON);
                            let typ = self.type_from_token(self.consume(current));
                            fields.insert(i.to_string(), typ);
                        }
                        _ => panic!("expected identifier"),
                    }
                }

                // consume the rbracket
                self.consume(current);

                ParsedAST::TYPE(Typ {
                    fields,
                    anon_name: None,
                })
            }
            // Token::HASH => {
            //     self.consume(current);
            //     let value = self.consume(current);
            //     let mut args: Vec<ParsedAST> = vec![];
            //     if (self.expecting(Token::LPAREN, current)) {
            //         self.consume(current);

            //         while !self.expecting(Token::RPAREN, current) {
            //             args.push(self.expression(current));
            //             if !self.expecting(Token::RPAREN, current) {
            //                 self.consume(current); // consume the ,
            //             }
            //         }

            //         self.consume(current); // consume the rparen
            //     }
            //     // todo directives don't need bodies!
            //     //let body = self.statement(current);
            //     ParsedAST::DIRECTIVE(Directive {
            //         value: value.clone(),
            //         args,
            //         body: None,
            //     })
            // }
            // todo do we want true/false to be numbers?
            Token::TRUE => {
                self.consume(current);
                ParsedAST::NUMBER(Number::INTEGER(1))
            }
            Token::FALSE => {
                self.consume(current);
                ParsedAST::NUMBER(Number::INTEGER(0))
            }
            Token::IDENTIFIER(identifier) => {
                self.consume(current);
                ParsedAST::IDENTIFIER(identifier.to_string())
            }
            Token::STRING(string) => {
                self.consume(current);
                ParsedAST::STRING(string.to_string())
            }
            Token::NUMBER(number) => {
                let num = self.consume(current);
                if number.parse::<i32>().is_ok() {
                    return ParsedAST::NUMBER(Number::INTEGER(number.parse::<i32>().unwrap()));
                } else if number.parse::<f32>().is_ok() {
                    return ParsedAST::NUMBER(Number::FLOAT(number.parse::<f32>().unwrap()));
                }
                panic!("failed to parse number {:?}", num);
            }
            // todo
            // Token::LPAREN => {
            //     self.consume(current);
            //     let expression = self.expression(current);
            //     self.consume(current);
            //     ParsedAST::GROUP(Group {
            //         expression: Box::new(expression),
            //     })
            // }
            // todo
            Token::LCURLY => self.block(current),
            _ => panic!(),
        }
    }

    fn block(&self, current: &mut usize) -> ParsedAST {
        self.consume(current);
        let mut body: Vec<ParsedAST> = vec![];
        while !self.end(current) && !self.expecting(Token::RCURLY, current) {
            body.push(self.statement(current));
        }
        self.consume(current);
        return ParsedAST::BLOCK(Block {
            new_scope: true,
            body,
        });
    }

    fn peek(&self, current: &usize) -> &Token {
        match self.tokens.get(*current) {
            std::option::Option::Some(t) => return t,
            _ => panic!("there are no tokens at the current index {}", *current),
        }
    }

    fn peek_ahead(&self, current: &usize, amount: i32) -> &Token {
        match self.tokens.get((*current as i32 + amount) as usize) {
            std::option::Option::Some(t) => return t,
            _ => panic!("there are no tokens at the current index {}", *current),
        }
    }

    fn end(&self, current: &usize) -> bool {
        *current >= self.tokens.len()
    }

    fn end_ahead(&self, current: &usize, amount: i32) -> bool {
        (*current as i32 + amount) as usize >= self.tokens.len()
    }

    fn expecting(&self, token: Token, current: &usize) -> bool {
        let next = self.peek(current);
        return token.eq(&next);
    }

    fn consume(&self, current: &mut usize) -> &Token {
        match self.tokens.get(*current) {
            std::option::Option::Some(t) => {
                *current += 1;
                return t;
            }
            _ => panic!("there are no tokens at the current index {}", *current),
        }
    }

    fn consume_expected(&self, current: &mut usize, token_expected: Token) -> &Token {
        match self.tokens.get(*current) {
            std::option::Option::Some(t) => {
                if *t != token_expected {
                    panic!("expected {:?} found {:?}", token_expected, t);
                }
                *current += 1;
                return t;
            }
            _ => panic!("there are no tokens at the current index {}", *current),
        }
    }

    fn type_from_token(&self, token: &Token) -> Type {
        match token {
            Token::U32 => Type::U32,
            Token::I32 => Type::I32,
            _ => panic!("expected type"),
        }
    }
}
