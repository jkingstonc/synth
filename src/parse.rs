use std::time::Instant;

use log::debug;

use crate::ast::{ParsedAST, Program};
use crate::lex::Token;

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
            // body.push(self.statement(&mut current));
            current += 1;
        }

        return ParsedAST::PROGRAM(Program { body: body });
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
