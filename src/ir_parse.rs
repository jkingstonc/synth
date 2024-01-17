use std::{borrow::BorrowMut, collections::HashMap, time::Instant, vec};

use log::debug;

use crate::{
    ast::{
        Assign, Binary, Block, Call, Decl, Fun, If, LeftUnary, Number, ParsedAST, Program,
        Qualifier, Typ,
    },
    compiler::CompilerOptions,
    ir::{IRValue, Instruction, Ref},
    ir_interpret::IRInterpreter,
    token::Token,
    types::Type,
};

pub struct IRParser<'a> {
    pub compiler_options: &'a CompilerOptions,
    pub counter: usize,
    pub lambda_counter: usize,
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
        //     data: Some(IRValue::INSTRUCTIONS(instructions)),
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

    // todo we need to return the IRValue with the resolved value!!!
    fn gen_ast(
        &mut self,
        ast: &mut ParsedAST,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
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
            ParsedAST::CALL(call) => self.gen_call(call, current_block),
            // ParsedAST::FOR(forr) => self.type_check_for(forr),
            // ParsedAST::RET(ret) => self.type_check_ret(ret),
            // ParsedAST::DECL(decl) => self.type_check_decl(decl),
            ParsedAST::ASSIGN(assign) => self.gen_assign(assign, current_block),
            ParsedAST::FN(func) => self.gen_func(func, current_block),
            ParsedAST::TYPE(typ) => self.gen_type(typ, current_block),
            // ParsedAST::NUMBER(num) => self.type_check_num(num),
            ParsedAST::LEFT_UNARY(left_unary) => self.gen_left_unary(left_unary, current_block), //self.type_check_binary(binary),
            // ParsedAST::BINARY(binary) => self.type_check_binary(binary),
            // ParsedAST::CALL(call) => self.type_check_call(call), // todo
            // ParsedAST::STRUCT_TYPES_LIST(s) => None, // todo
            // ParsedAST::LHS_ACCESS(lhs_access) => None, // todo
            // ParsedAST::GROUP(_) => None, // todo
            ParsedAST::TYPE(typ) => self.gen_typ(typ, current_block),
            _ => panic!(),
        }
    }

    fn gen_program(
        &mut self,
        program: &mut Program,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        for item in program.body.iter_mut() {
            let (instruction, _) = self.gen_ast(item, current_block);
            if let Some(instruction_unwrapped) = instruction {
                self.write_instruction_to_block(instruction_unwrapped, current_block);
            } else {
                // todo, do we need to panic here? maybe not?
                // panic!("expected instruction");
            }
        }
        (None, None)
    }

    fn gen_stmt(
        &mut self,
        stmt: &mut Box<ParsedAST>,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        self.gen_ast(stmt, current_block)
    }

    fn gen_binary(
        &mut self,
        binary: &mut Binary,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        let (_, left_address) = self.gen_ast(&mut binary.left, current_block);
        let (_, right_address) = self.gen_ast(&mut binary.right, current_block);

        let l: IRValue;
        match left_address {
            Some(left) => {
                l = left;
            }
            _ => panic!(),
        };
        let r: IRValue;
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
                    IRValue::INT(_) => {}
                    _ => should_optimize = false,
                };
                match r {
                    IRValue::INT(_) => {}
                    _ => should_optimize = false,
                };

                if self.compiler_options.optimization > 0 && should_optimize {
                    let mut lhs_value = 0;
                    let mut rhs_value = 0;
                    match l {
                        IRValue::INT(i) => lhs_value = i,
                        _ => todo!("unsupported type for add optimization"),
                    };
                    match r {
                        IRValue::INT(i) => rhs_value = i,
                        _ => todo!("unsupported type for add optimization"),
                    };

                    (None, Some(IRValue::INT(lhs_value + rhs_value)))
                } else {
                    self.write_instruction_to_block(
                        Instruction::ADD(format!("{:?}", locals_id), l, r),
                        current_block,
                    );
                    (
                        // Some(Instruction::ADD(format!("{:?}", locals_id), l, r)),
                        None,
                        Some(IRValue::REF(Ref {
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
    ) -> (Option<Instruction>, Option<IRValue>) {
        self.counter += 1;
        match num {
            Number::INTEGER(i) => (None, Some(IRValue::INT(i.clone()))),
            Number::FLOAT(f) => (None, Some(IRValue::FLOAT(f.clone()))),
        }
    }

    fn gen_string(
        &mut self,
        s: &mut String,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        (None, Some(IRValue::STRING(s.to_string())))
    }

    fn gen_decl(
        &mut self,
        decl: &mut Decl,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        // todo check if we are dealing with a struct!

        match &decl.typ {
            Some(Type::STRUCT(i)) => (
                Some(Instruction::STACK_VAR(
                    decl.identifier.clone(),
                    decl.typ.clone().unwrap(),
                    None,
                )),
                None,
            ),
            Some(Type::TYPE) => {
                // todo figure out the const type qualifier thingy here!
                match decl.qualifier {
                    Qualifier::CONST => {}
                    Qualifier::VAR => {
                        //
                    }
                }

                if let Some(value) = decl.value.as_mut() {
                    let (_, data) = self.gen_ast(value, current_block);
                }
                // we then need to stack the var & give it a value

                // todo populate this with the type information
                // todo we probably also want to specify the name of the struct or if its anonymous,
                // otherwise this wont translate well to llvm
                (
                    Some(Instruction::STACK_VAR(
                        decl.identifier.clone(),
                        // todo get the type
                        Type::STRUCT("Runtime_Type".to_string()),
                        Some(IRValue::STRUCT(vec![IRValue::INT(123)])),
                    )),
                    None,
                )
            }
            _ => {
                // first generate the decl value
                let mut instruction_data = None;
                if let Some(value) = decl.value.as_mut() {
                    let (_, data) = self.gen_ast(value, current_block);
                    instruction_data = data;
                }
                self.counter += 1;
                (
                    Some(Instruction::STACK_VAR(
                        decl.identifier.clone(),
                        // todo get the type!
                        Type::I32,
                        instruction_data,
                    )),
                    None,
                )
            }
        }
    }

    fn gen_block(
        &mut self,
        block: &mut Block,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
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

    fn gen_typ(
        &mut self,
        typ: &mut Typ,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        (None, None)
    }

    fn gen_left_unary(
        &mut self,
        left_unary: &mut LeftUnary,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        match left_unary {
            LeftUnary::COMP(expr) => {
                // todo
                let mut ir_executor = IRInterpreter {
                    compiler_options: self.compiler_options,
                    counter: 0,
                    variables_map: HashMap::new(),
                };

                // todo we need to capture this all in a new block
                let mut comptime_block: Box<Vec<Instruction>> = Box::new(vec![]);
                self.gen_ast(expr, &mut comptime_block);

                let comptime_instruction = Instruction::PROGRAM(comptime_block);
                let result = ir_executor.execute(&comptime_instruction);

                return (None, result);
                // let (rhs_instruction, data) = self.gen_ast(expr, &mut comptime_block);
                // if let Some(rhs_instruction_unpacked) = rhs_instruction {
                //     // todo we need to get the result!
                //     // todo umm this isn't working
                //     let result = ir_executor.execute(&rhs_instruction_unpacked);

                //     // debug!("got result! {:?}", result);
                //     debug!("executing... <{:?}>", result);
                //     return (None, result);
                // } else if let Some(data_unpacked) = data {
                //     debug!(
                //         "comptime already found value {:?}, inserting",
                //         data_unpacked
                //     );
                //     return (None, Some(data_unpacked));
                // }
            }
        }

        (None, None)
    }

    // todo we need to do name resolving here!!!
    fn gen_type(
        &mut self,
        typ: &mut Typ,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        // todo we need to somehow get the anonymous name of the type?
        let mut types: Vec<Type> = vec![];
        for (_, t) in typ.fields.iter_mut() {
            types.push(t.clone());
        }
        let type_instruction = Instruction::TYPE("anon_type".to_string(), types);
        self.write_instruction_to_block(type_instruction, current_block);
        (
            None,
            Some(IRValue::REF(Ref {
                value: "anon_type".to_string(),
            })),
        )
    }

    fn gen_func(
        &mut self,
        func: &mut Fun,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        let mut name: String;
        if func.identifier.is_some() {
            name = func.identifier.as_mut().unwrap().to_string();
        } else {
            name = format!("{}_lambda", self.lambda_counter);
            self.lambda_counter += 1;
        }

        // todo for some expressions such as calls we dont return the instruction, i think we should return
        // the instructions by default and let the blocks generate them?
        let (i, _) = self.gen_ast(&mut func.body, current_block);

        let mut params: Vec<Type> = vec![];
        for p in func.params.iter() {
            params.push(p.typ.clone().unwrap());
        }

        let func_instruction = Instruction::FUNC(name, params, Box::new(i.unwrap()));

        (Some(func_instruction), None)
    }

    fn gen_assign(
        &mut self,
        assign: &mut Assign,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        // new instruction?

        match assign.lhs.as_mut() {
            ParsedAST::IDENTIFIER(i) => {
                let (_, value) = self.gen_ast(&mut assign.rhs, current_block);
                self.write_instruction_to_block(
                    Instruction::STORE(
                        Ref {
                            value: i.to_string(),
                        },
                        value.unwrap(),
                    ),
                    current_block,
                );
            }
            _ => todo!(),
        }
        (None, None)
    }

    fn gen_call(
        &mut self,
        call: &mut Call,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        // todo a call should just be a string reference to a function
        let mut f: String;

        match call.callee.as_ref() {
            ParsedAST::IDENTIFIER(i) => {
                f = i.to_string();
            }
            _ => todo!(),
        }

        // // let (callee_instruction, callee_data) = self.gen_ast(&mut call.callee, current_block);
        // let mut first_arg = call.args[0].borrow_mut();
        // let (first_arg_instruction, first_arg_data) = self.gen_ast(first_arg, current_block);

        let locals_id = self.locals_counter;
        self.locals_counter += 1;

        let mut args: Vec<IRValue> = vec![];

        for arg in call.args.iter_mut() {
            let (instr, val) = self.gen_ast(arg, current_block);
            args.push(val.unwrap());
        }

        self.write_instruction_to_block(
            Instruction::CALL(
                locals_id.to_string(),
                // callee_data.expect("expected callee data"),
                f.to_string(),
                args,
            ),
            current_block,
        );
        // todo this is really annoying
        (
            None,
            Some(IRValue::REF(Ref {
                value: locals_id.to_string(),
            })),
        )
    }

    fn gen_if(
        &mut self,
        iff: &mut If,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
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
        identifier: &mut String,
        // instructions: &mut Box<Vec<Instruction>>,
        current_block: &mut Box<Vec<Instruction>>,
    ) -> (Option<Instruction>, Option<IRValue>) {
        let locals_id = self.locals_counter;
        self.locals_counter += 1;

        // // todo do we need to do a load here?
        // self.write_instruction_to_block(
        //     Instruction::LOAD(
        //         format!("{:?}", locals_id),
        //         Ref {
        //             value: identifier.to_string(),
        //         },
        //     ),
        //     current_block,
        // );
        // self.counter += 1;
        // (
        //     None,
        //     Some(IRValue::REF(Ref {
        //         value: format!("{:?}", locals_id),
        //     })),
        // )
        //todo we are instead returning a ref!
        // todo do we need to do a load here?
        // self.write_instruction_to_block(
        //     Instruction::LOAD(
        //         format!("{:?}", locals_id),
        //         Ref {
        //             value: identifier.to_string(),
        //         },
        //     ),
        //     current_block,
        // );
        self.counter += 1;
        (
            None,
            Some(IRValue::REF(Ref {
                value: identifier.to_string(),
            })),
        )
    }
}
