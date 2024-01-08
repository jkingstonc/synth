use std::{time::Instant, vec};

use log::debug;

use crate::{
    ast::{Binary, Block, Decl, If, Number, ParsedAST, Program},
    compiler::CompilerOptions,
    ir::{Instruction, InstructionData, Ref},
    token::Token,
};

pub struct IRParser<'a> {
    pub compiler_options: &'a CompilerOptions,
    pub counter: usize,
    pub block_counter: usize,
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

impl IRParser<'_> {
    pub fn parse(&mut self, mut ast: Box<ParsedAST>) -> Instruction {
        let mut instructions: Box<Vec<Instruction>> = Box::new(vec![]);

        let now = Instant::now();

        let mut main_block_instructions: Box<Vec<Instruction>> = Box::new(vec![]);

        let (instruction, data) = self.gen_ast(ast.as_mut(), &mut main_block_instructions);
        // if let Some(instruction_unwrapped) = instruction {
        //     self.write_instruction_to_block(instruction_unwrapped, &mut main_block_instructions);
        // } else {
        //     panic!("expected instruction");
        // }
        let elapsed = now.elapsed();
        debug!(
            "ir parsing time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
        // let main_block = Instruction {
        //     instruction_type: InstructionType::BLOCK,
        //     data: Some(InstructionData::INSTRUCTIONS(instructions)),
        //     assignment_name: None,
        // };
        // return Box::new(vec![main_block]);
        return Instruction::PROGRAM(main_block_instructions);
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
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<InstructionData>) {
        match ast {
            ParsedAST::PROGRAM(program) => self.gen_program(program, current_block),
            ParsedAST::STMT(stmt) => self.gen_stmt(stmt, current_block),
            ParsedAST::BINARY(binary) => self.gen_binary(binary, current_block),
            ParsedAST::NUMBER(num) => self.gen_num(num, current_block),
            ParsedAST::STRING(s) => self.gen_string(s, current_block),
            ParsedAST::DECL(decl) => self.gen_decl(decl, current_block),
            ParsedAST::IDENTIFIER(identifier) => self.gen_identifier(identifier, current_block),
            // ParsedAST::DIRECTIVE(directive) => self.type_check_directive(directive),
            // ParsedAST::PROGRAM(program) => self.type_check_program(program),
            ParsedAST::BLOCK(block) => self.gen_block(block, current_block),
            ParsedAST::IF(iff) => self.gen_if(iff, current_block),
            // ParsedAST::FOR(forr) => self.type_check_for(forr),
            // ParsedAST::RET(ret) => self.type_check_ret(ret),
            // ParsedAST::DECL(decl) => self.type_check_decl(decl),
            // ParsedAST::ASSIGN(assign) => self.type_check_assign(assign),
            // ParsedAST::FN(func) => self.type_check_func(func),
            // ParsedAST::NUMBER(num) => self.type_check_num(num),
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
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<InstructionData>) {
        for item in program.body.iter_mut() {
            let (instruction, _) = self.gen_ast(item, current_block);
            if let Some(instruction_unwrapped) = instruction {
                self.write_instruction_to_block(instruction_unwrapped, current_block);
            } else {
                panic!("expected instruction");
            }
        }
        (None, None)
    }

    fn gen_stmt(
        &mut self,
        stmt: &mut Box<ParsedAST>,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<InstructionData>) {
        self.gen_ast(stmt, current_block)
    }

    fn gen_binary(
        &mut self,
        binary: &mut Binary,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<InstructionData>) {
        let (_, left_address) = self.gen_ast(&mut binary.left, current_block);
        let (_, right_address) = self.gen_ast(&mut binary.right, current_block);

        let l: InstructionData;
        match left_address {
            Some(left) => {
                l = left;
            }
            _ => panic!(),
        };
        let r: InstructionData;
        match right_address {
            Some(right) => {
                r = right;
            }
            _ => panic!(),
        };

        let locals_id = self.locals_counter;
        self.locals_counter += 1;

        match binary.op {
            Token::PLUS => {
                // todo enable optimisation

                let mut should_optimize = true;
                match l {
                    InstructionData::INT(_) => {}
                    _ => should_optimize = false,
                };
                match r {
                    InstructionData::INT(_) => {}
                    _ => should_optimize = false,
                };

                if self.compiler_options.optimization > 0 && should_optimize {
                    let mut lhs_value = 0;
                    let mut rhs_value = 0;
                    match l {
                        InstructionData::INT(i) => lhs_value = i,
                        _ => todo!("unsupported type for add optimization"),
                    };
                    match r {
                        InstructionData::INT(i) => rhs_value = i,
                        _ => todo!("unsupported type for add optimization"),
                    };

                    (None, Some(InstructionData::INT(lhs_value + rhs_value)))
                } else {
                    self.write_instruction_to_block(
                        Instruction::ADD(format!("{:?}", locals_id), l, r),
                        current_block,
                    );
                    (
                        // Some(Instruction::ADD(format!("{:?}", locals_id), l, r)),
                        None,
                        Some(InstructionData::REF(Ref {
                            value: format!("{:?}", locals_id),
                        })),
                    )
                }
            }
            _ => panic!(),
        }
    }

    fn gen_num(
        &mut self,
        num: &mut Number,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<InstructionData>) {
        self.counter += 1;
        match num {
            Number::INTEGER(i) => (None, Some(InstructionData::INT(i.clone()))),
            Number::FLOAT(f) => (None, Some(InstructionData::FLOAT(f.clone()))),
        }
    }

    fn gen_string(
        &mut self,
        s: &mut String,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<InstructionData>) {
        self.counter += 1;
        (None, Some(InstructionData::STRING(s.to_string())))
    }

    fn gen_decl(
        &mut self,
        decl: &mut Decl,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<InstructionData>) {
        // first generate the decl value
        let mut instruction_data = None;
        if let Some(value) = decl.value.as_mut() {
            let (_, data) = self.gen_ast(value, current_block);
            instruction_data = data;
        }

        // match current_block
        // let Instruction::BLOCK(_, instructions) = current_block;
        // current_block.push(Instruction::STACK_VAR(
        //     decl.identifier.clone(),
        //     instruction_data,
        // ));
        // instructions.push(Instruction {
        //     instruction_type: InstructionType::STACK_VAR,
        //     data: instruction_data,
        //     assignment_name: Some(decl.identifier.clone()),
        // });
        self.counter += 1;
        (
            Some(Instruction::STACK_VAR(
                decl.identifier.clone(),
                instruction_data,
            )),
            None,
        )
    }

    fn gen_block(
        &mut self,
        block: &mut Block,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<InstructionData>) {
        let block_id = self.block_counter;
        self.block_counter += 1;
        let mut new_block_instructions: Box<Vec<Instruction>> = Box::new(vec![]);
        for mut instruction in &mut block.body {
            let (instruction, _) = self.gen_ast(&mut instruction, &mut new_block_instructions);
            if let Some(instruction_unwrapped) = instruction {
                new_block_instructions.push(instruction_unwrapped);
            }
        }
        let mut new_block = Instruction::BLOCK(format!("{:?}", block_id), new_block_instructions);
        // self.write_instruction_to_block(new_block, current_block);
        (Some(new_block), None)
    }

    fn gen_if(
        &mut self,
        iff: &mut If,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<InstructionData>) {
        let (_, condition_data) = self.gen_ast(&mut iff.condition, current_block);
        if let Some(condition_data_unwrapped) = condition_data {
            // todo uhhh
            // todo bug: we have an issue here because the statement bit doesn't return teh i
            let (body_instruction, _) = self.gen_ast(&mut iff.body, current_block);
            let body_instruction_unwrapped = body_instruction.expect("expected body instruction");
            // todo do the else
            if let Some(mut else_body) = iff.else_body.as_mut() {
                let (else_body_instruction, _) = self.gen_ast(&mut else_body, current_block);
                let e = else_body_instruction.expect("expected body");
                return (
                    Some(Instruction::COND_BR(
                        condition_data_unwrapped,
                        Box::new(body_instruction_unwrapped),
                        Some(Box::new(e)),
                    )),
                    None,
                );
            } else {
                return (
                    Some(Instruction::COND_BR(
                        condition_data_unwrapped,
                        Box::new(body_instruction_unwrapped),
                        None,
                    )),
                    None,
                );
            }
        } else {
            panic!("conditional branch requires condition");
        }
        // todo
        (None, None)
    }

    fn gen_identifier(
        &mut self,
        identifier: &mut std::string::String,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<InstructionData>) {
        let locals_id = self.locals_counter;
        self.locals_counter += 1;

        // todo do we need to do a load here?
        self.write_instruction_to_block(
            Instruction::LOAD(
                format!("{:?}", locals_id),
                Ref {
                    value: identifier.to_string(),
                },
            ),
            current_block,
        );
        self.counter += 1;
        (
            // Some(Instruction::LOAD(
            //     format!("{:?}", locals_id),
            //     Ref {
            //         value: identifier.to_string(),
            //     },
            // )),
            None,
            Some(InstructionData::REF(Ref {
                value: format!("{:?}", locals_id),
            })),
        )
    }
}
