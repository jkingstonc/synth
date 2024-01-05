use std::time::Instant;

use log::debug;

use crate::{
    ast::{Binary, Decl, Number, ParsedAST, Program},
    ir::{Instruction, InstructionData, InstructionType, Ref},
    token::Token,
};

pub struct IRParser {
    pub counter: usize,
    pub locals_counter: usize,
}

// the following instructions
//
// const my_var = 2
// my_var+4+5
//
// would be compiled to
//

// 0 :: %my_var = int 2
// 2 :: %0 = ADDI %my_var 4
// 3 :: ADDI %0 5
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

    // todo we need to return the InstructionData with the resolved value!!!
    fn gen_ast(
        &mut self,
        ast: &mut ParsedAST,
        instructions: &mut Box<Vec<Instruction>>,
    ) -> Option<InstructionData> {
        match ast {
            ParsedAST::PROGRAM(program) => self.gen_program(program, instructions),
            ParsedAST::STMT(stmt) => self.gen_stmt(stmt, instructions),
            ParsedAST::BINARY(binary) => self.gen_binary(binary, instructions),
            ParsedAST::NUMBER(num) => self.gen_num(num, instructions),
            ParsedAST::DECL(decl) => self.gen_decl(decl, instructions),
            ParsedAST::IDENTIFIER(identifier) => self.gen_identifier(identifier, instructions),
            // ParsedAST::DIRECTIVE(directive) => self.type_check_directive(directive),
            // ParsedAST::PROGRAM(program) => self.type_check_program(program),
            // ParsedAST::BLOCK(block) => self.type_check_block(block),
            // ParsedAST::IF(iff) => self.type_check_if(iff),
            // ParsedAST::FOR(forr) => self.type_check_for(forr),
            // ParsedAST::RET(ret) => self.type_check_ret(ret),
            // ParsedAST::DECL(decl) => self.type_check_decl(decl),
            // ParsedAST::ASSIGN(assign) => self.type_check_assign(assign),
            // ParsedAST::FN(func) => self.type_check_func(func),
            // ParsedAST::NUMBER(num) => self.type_check_num(num),
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

    fn gen_program(
        &mut self,
        program: &mut Program,
        instructions: &mut Box<Vec<Instruction>>,
    ) -> Option<InstructionData> {
        for item in program.body.iter_mut() {
            self.gen_ast(item, instructions);
        }
        None
    }

    fn gen_stmt(
        &mut self,
        stmt: &mut Box<ParsedAST>,
        instructions: &mut Box<Vec<Instruction>>,
    ) -> Option<InstructionData> {
        self.gen_ast(stmt, instructions)
    }

    fn gen_binary(
        &mut self,
        binary: &mut Binary,
        instructions: &mut Box<Vec<Instruction>>,
    ) -> Option<InstructionData> {
        let left_address = self.gen_ast(&mut binary.left, instructions);
        let right_address = self.gen_ast(&mut binary.right, instructions);

        /*
        TODO: this needs reworking, currently were assuming that the left & right are references
        when in reality they can be immediate values. we either need to have seperate instructions
        for mixed refs & immediates or just wack them in the same one. idk.
         */
        let mut left_ref: Ref = Ref {
            value: "".to_string(),
        };
        let mut right_ref: Ref = Ref {
            value: "".to_string(),
        };
        if let Some(left_address_value) = left_address {
            if let InstructionData::REF(left_address_value_as_ref) = left_address_value {
                left_ref = left_address_value_as_ref;
            }
        }
        if let Some(right_address_value) = right_address {
            if let InstructionData::REF(right_address_value_as_ref) = right_address_value {
                right_ref = right_address_value_as_ref;
            }
        }

        let locals_id = self.locals_counter;
        self.locals_counter += 1;

        match binary.op {
            Token::PLUS => self.write_instruction_to_block(
                Instruction {
                    instruction_type: InstructionType::ADD,
                    // todo maybe this should be instruction data not a ref
                    data: Some(InstructionData::DOUBLE_REF(left_ref, right_ref)),
                    assignment_name: Some(format!("{:?}", locals_id)),
                },
                instructions,
            ),
            _ => panic!(),
        };
        self.counter += 1;
        Some(InstructionData::REF(Ref {
            value: format!("{:?}", locals_id),
        }))
    }

    fn gen_num(
        &mut self,
        num: &mut Number,
        instructions: &mut Box<Vec<Instruction>>,
    ) -> Option<InstructionData> {
        self.counter += 1;
        match num {
            Number::INTEGER(i) => Some(InstructionData::INT(i.clone())),
            Number::FLOAT(f) => Some(InstructionData::FLOAT(f.clone())),
            // Number::INTEGER(i) => self.write_instruction_to_block(
            //     Instruction {
            //         instruction_type: InstructionType::INT,
            //         data: Some(InstructionData::INT(*i)),
            //         assignment_name: None,
            //     },
            //     instructions,
            // ),
            // Number::FLOAT(f) => self.write_instruction_to_block(
            //     Instruction {
            //         instruction_type: InstructionType::INT,
            //         data: Some(InstructionData::FLOAT(*f)),
            //         assignment_name: None,
            //     },
            //     instructions,
            // ),
        }
    }

    fn gen_decl(
        &mut self,
        decl: &mut Decl,
        instructions: &mut Box<Vec<Instruction>>,
    ) -> Option<InstructionData> {
        // first generate the decl value
        let mut instruction_data = None;
        if let Some(value) = decl.value.as_mut() {
            instruction_data = self.gen_ast(value, instructions);
        }
        instructions.push(Instruction {
            instruction_type: InstructionType::STACK_VAR,
            data: instruction_data,
            assignment_name: Some(decl.identifier.clone()),
        });
        self.counter += 1;
        None
    }

    fn gen_identifier(
        &mut self,
        identifier: &mut std::string::String,
        instructions: &mut Box<Vec<Instruction>>,
    ) -> Option<InstructionData> {
        let locals_id = self.locals_counter;
        self.locals_counter += 1;

        // do a load
        instructions.push(Instruction {
            instruction_type: InstructionType::LOAD,
            data: Some(InstructionData::REF(Ref {
                value: identifier.to_string(),
            })),
            // todo keep track of locals
            assignment_name: Some(format!("{:?}", locals_id)),
        });

        self.counter += 1;
        Some(InstructionData::REF(Ref {
            value: format!("{:?}", locals_id),
        }))
    }
}
