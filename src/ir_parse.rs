use std::time::Instant;

use log::debug;

use crate::{
    ast::{Binary, Number, ParsedAST, Program},
    ir::{Instruction, InstructionData, InstructionType, Ref},
    token::Token,
};

pub struct IRParser {}

// the following instructions
//
// 4+5+my_var
//
// would be compiled to
//

// %0 = int 4
// %1 = int 5
// %2 = load "my_var"
// %3 = add %0 %1
// %4 = add %3 %2
//
//

impl IRParser {
    pub fn parse(&mut self, mut ast: Box<ParsedAST>) -> Box<Vec<Instruction>> {
        let mut instructions: Box<Vec<Instruction>> = Box::new(vec![]);
        let now = Instant::now();
        self.gen_ast(ast.as_mut(), &mut instructions);
        let elapsed = now.elapsed();
        debug!(
            "ir parsing time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
        return instructions;
    }

    fn write_instruction_to_block(
        &mut self,
        instruction: Instruction,
        instructions: &mut Box<Vec<Instruction>>,
    ) {
        instructions.push(instruction);
    }

    fn gen_ast(&mut self, ast: &mut ParsedAST, instructions: &mut Box<Vec<Instruction>>) {
        match ast {
            ParsedAST::PROGRAM(program) => self.gen_program(program, instructions),
            ParsedAST::STMT(stmt) => self.gen_stmt(stmt, instructions),
            ParsedAST::BINARY(binary) => self.gen_binary(binary, instructions),
            ParsedAST::NUMBER(num) => self.gen_num(num, instructions),
            // ParsedAST::DIRECTIVE(directive) => self.type_check_directive(directive),
            // ParsedAST::STMT(stmt) => self.type_check_ast(stmt),
            // ParsedAST::PROGRAM(program) => self.type_check_program(program),
            // ParsedAST::BLOCK(block) => self.type_check_block(block),
            // ParsedAST::IF(iff) => self.type_check_if(iff),
            // ParsedAST::FOR(forr) => self.type_check_for(forr),
            // ParsedAST::RET(ret) => self.type_check_ret(ret),
            // ParsedAST::DECL(decl) => self.type_check_decl(decl),
            // ParsedAST::ASSIGN(assign) => self.type_check_assign(assign),
            // ParsedAST::FN(func) => self.type_check_func(func),
            // ParsedAST::NUMBER(num) => self.type_check_num(num),
            // ParsedAST::IDENTIFIER(identifier) => self.type_check_identifier(identifier),
            // ParsedAST::STRING(s) => self.type_check_string(s),
            // ParsedAST::LEFT_UNARY(left_unary) => self.type_check_left_unary(left_unary),//self.type_check_binary(binary),
            // ParsedAST::BINARY(binary) => self.type_check_binary(binary),
            // ParsedAST::CALL(call) => self.type_check_call(call), // todo
            // ParsedAST::STRUCT_TYPES_LIST(s) => None, // todo
            // ParsedAST::LHS_ACCESS(lhs_access) => None, // todo
            // ParsedAST::GROUP(_) => None, // todo
            _ => panic!(),
        }
    }

    fn gen_program(&mut self, program: &mut Program, instructions: &mut Box<Vec<Instruction>>) {
        for item in program.body.iter_mut() {
            self.gen_ast(item, instructions);
        }
    }

    fn gen_stmt(&mut self, stmt: &mut Box<ParsedAST>, instructions: &mut Box<Vec<Instruction>>) {
        self.gen_ast(stmt, instructions);
    }

    fn gen_binary(&mut self, binary: &mut Binary, instructions: &mut Box<Vec<Instruction>>) {
        // todo we probably need to return the location etc
        self.gen_ast(&mut binary.left, instructions);
        self.gen_ast(&mut binary.right, instructions);
        match binary.op {
            Token::PLUS => self.write_instruction_to_block(
                Instruction {
                    instruction_type: InstructionType::INT,
                    data: InstructionData::DOUBLE_REF(Ref { value: 0 }, Ref { value: 1 }),
                },
                instructions,
            ),
            _ => panic!(),
        }
    }

    fn gen_num(&mut self, num: &mut Number, instructions: &mut Box<Vec<Instruction>>) {
        match num {
            Number::INTEGER(i) => self.write_instruction_to_block(
                Instruction {
                    instruction_type: InstructionType::INT,
                    data: InstructionData::INT(*i),
                },
                instructions,
            ),
            Number::FLOAT(f) => self.write_instruction_to_block(
                Instruction {
                    instruction_type: InstructionType::INT,
                    data: InstructionData::FLOAT(*f),
                },
                instructions,
            ),
        }
    }
}
