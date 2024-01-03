use std::time::Instant;

use log::debug;

use crate::ast::{Assign, Binary, Call, LhsAccess, Number, ParsedAST, Program};
use crate::token::Token;

pub struct Parser<'a> {
    pub tokens: &'a Box<Vec<Token>>,
}

impl Parser<'_> {
    pub fn parse(&mut self) -> Box<ParsedAST> {
        let now = Instant::now();
        let elapsed = now.elapsed();
        debug!(
            "parse time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
        Box::new(self.parse_program())
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
            // Token::IF => self.if_stmt(current),
            // Token::FOR => self.for_stmt(current),
            // Token::RET => self.ret(current),
            _ => ParsedAST::STMT(Box::new(self.expression(current))),
        }
    }

    fn expression(&self, current: &mut usize) -> ParsedAST {
        debug!("doing expression!");
        match self.peek(&current) {
            _ => self.comparison(current),
        }
    }

    fn comparison(&self, current: &mut usize) -> ParsedAST {
        // todo!
        self.decl_or_assign(current)
    }

    fn decl_or_assign(&self, current: &mut usize) -> ParsedAST {
        self.assign(current)
        // todo
        // let first = self.peek(current);

        // if self.end_ahead(current, 1) {
        //     return self.assign(current);
        // }

        // let second = self.peek_ahead(current, 1);

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

        //                 let mut ident: std::string::String;
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
        //                 let mut ident: std::string::String;
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

        //                 let mut ident: std::string::String;
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
        let higher_precedence = self.plus_or_minus(current);
        if !self.end(current) {
            if self.expecting(Token::EQUAL, current) {
                self.consume(current);
                let rhs = self.plus_or_minus(current);
                return ParsedAST::ASSIGN(Assign {
                    lhs: Box::new(higher_precedence),
                    rhs: Box::new(rhs),
                });
            }
        }
        higher_precedence
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
        // if self.expecting(Token::AT, current) {
        //     self.consume(current);
        //     let rhs = self.call(current);
        //     let rhs_type = Type {
        //         mutability: Mutability::CONSTANT,
        //         primative: Primative::INCOMPLETE,
        //         reference: false,
        //     };
        //     return ParsedAST::LEFT_UNARY(LeftUnary::TAKE_REFERENCE(TakeReference {
        //         rhs: Box::new(rhs),
        //         rhs_type,
        //         is_heap_alloc: false,
        //     }));
        // }

        self.call(current)
    }

    fn call(&self, current: &mut usize) -> ParsedAST {
        let higher_presedence = self.struct_access(current);
        if !self.end(current) {
            match self.peek_ahead(current, -1) {
                Token::IDENTIFIER(_) => {
                    if !self.end_ahead(current, 1) {
                        // todo
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
        debug!("doing single!");
        match self.peek(current) {
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
                debug!("doing number!");
                let num = self.consume(current);
                if number.parse::<i32>().is_ok() {
                    debug!("doing i32!");
                    return ParsedAST::NUMBER(Number::INTEGER(number.parse::<i32>().unwrap()));
                } else if number.parse::<f32>().is_ok() {
                    debug!("doing f32!");
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
            // Token::LCURLY => self.block(current),
            _ => panic!(),
        }
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
}
