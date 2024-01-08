use std::{collections::HashMap, string, time::Instant};

use log::{debug, info};

use crate::{
    compiler::CompilerOptions,
    ir::{Instruction, InstructionData, Ref},
};

pub struct IRInterpreter<'a> {
    pub compiler_options: &'a CompilerOptions,
    pub counter: usize,
    // pub instruction: Instruction,
    // todo for now this is an i32 but should be a generic 'value'
    pub variables_map: HashMap<std::string::String, InstructionData>,
}

/*
This will walk through the instructions and evaluate them (no JIT yet :().
Still a massive WIP as we need to decide on the instruction model (SSA [Single Static Assignment] etc).
*/
impl IRInterpreter<'_> {
    pub fn execute(&mut self, instruction: &Instruction) -> Option<InstructionData> {
        let now = Instant::now();
        let result = self.execute_instruction(instruction);
        debug!("vars {:?}", self.variables_map);
        let elapsed = now.elapsed();
        debug!(
            "execution time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
        result
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Option<InstructionData> {
        match instruction {
            Instruction::PROGRAM(instructions) => self.execute_program(instructions.clone()),
            Instruction::BLOCK(_, instructions) => self.excecute_block(instructions.clone()),
            Instruction::STACK_VAR(label, value) => self.execute_stack_var(label, value),
            Instruction::LOAD(label, value) => self.execute_load(label, value),
            Instruction::ADD(label, left, right) => self.execute_add(label, left, right),
            Instruction::CALL(label, callee, arg) => self.execute_call(label, callee, arg),
            Instruction::COND_BR(condition, body, else_body) => {
                self.execute_cond_br(condition, body, else_body)
            }
            // InstructionType::INT => self.execute_int(instruction),
            // InstructionType::ADD => self.execute_add(instruction),
            // InstructionType::STACK_VAR => self.execute_var(instruction),
            _ => panic!(),
        }
    }

    fn execute_program(&mut self, instructions: Box<Vec<Instruction>>) -> Option<InstructionData> {
        let mut result: Option<InstructionData> = None;
        for instruction in instructions.to_vec() {
            result = self.execute_instruction(&instruction);
        }
        result
    }

    fn excecute_block(&mut self, instructions: Box<Vec<Instruction>>) -> Option<InstructionData> {
        let mut result: Option<InstructionData> = None;
        for instruction in instructions.to_vec() {
            result = self.execute_instruction(&instruction);
        }
        result
    }

    fn execute_load(
        &mut self,
        label: &std::string::String,
        ref_value: &Ref,
    ) -> Option<InstructionData> {
        if let Some(val) = self.variables_map.get(&ref_value.value) {
            self.variables_map.insert(label.to_string(), val.clone());
        } else if ref_value.value == "printf" {
            self.variables_map.insert(
                label.to_string(),
                InstructionData::INTRINSIC("printf".to_string()),
            );
        } else {
            panic!("couldn't find var");
        }
        None
    }

    fn execute_stack_var(
        &mut self,
        label: &std::string::String,
        value: &Option<InstructionData>,
    ) -> Option<InstructionData> {
        if let Some(data) = value {
            match data {
                InstructionData::INT(i) => {
                    self.variables_map.insert(label.to_string(), data.clone())
                }
                InstructionData::FLOAT(f) => {
                    self.variables_map.insert(label.to_string(), data.clone())
                }
                InstructionData::STRING(s) => {
                    self.variables_map.insert(label.to_string(), data.clone())
                }
                InstructionData::REF(r) => {
                    if let Some(v) = self.variables_map.get(&r.value) {
                        self.variables_map.insert(label.to_string(), v.clone());
                    } else {
                        panic!("couldn't find var");
                    };
                    None
                }
                _ => panic!("unsupported type for execution"),
            };
        };
        None
    }

    fn execute_call(
        &mut self,
        label: &std::string::String,
        callee: &InstructionData,
        arg: &InstructionData,
    ) -> Option<InstructionData> {
        debug!("umm {:?}", callee);

        let mut callee_data: InstructionData;
        match callee {
            InstructionData::REF(r) => {
                callee_data = self
                    .variables_map
                    .get(&r.value)
                    .expect("could not find var")
                    .clone();
            }
            _ => panic!("callee lookup should be ref"),
        }

        match callee_data {
            InstructionData::INTRINSIC(i) => {
                if i == "printf" {
                    // assume that args are passed in as locals
                    let mut arg_data: InstructionData;

                    // todo global formatter in ir.rs
                    match arg {
                        InstructionData::REF(r) => {
                            arg_data = self
                                .variables_map
                                .get(&r.value)
                                .expect("expected arg as ref")
                                .clone()
                        }
                        _ => panic!("couldn't print instruction data :("),
                    };

                    match arg_data {
                        InstructionData::INT(i) => println!("{}", i),
                        InstructionData::FLOAT(f) => println!("{}", f),
                        InstructionData::STRING(s) => println!("{}", s),
                        _ => panic!("couldn't print InstructionData"),
                    };
                }
            }
            _ => panic!("callee must be function or intrinsic"),
        };
        None
    }

    fn execute_add(
        &mut self,
        label: &std::string::String,
        left: &InstructionData,
        right: &InstructionData,
    ) -> Option<InstructionData> {
        let mut lhs = 0;
        let mut rhs = 0;
        match left {
            InstructionData::INT(i) => lhs = *i,
            InstructionData::REF(r) => {
                if let Some(v) = self.variables_map.get(&r.value) {
                    match v {
                        InstructionData::INT(i) => lhs = *i,
                        _ => panic!("unsupported type"),
                    }
                }
            }
            _ => panic!("unsupported type for execution"),
        }
        match right {
            InstructionData::INT(i) => rhs = *i,
            InstructionData::REF(r) => {
                if let Some(v) = self.variables_map.get(&r.value) {
                    match v {
                        InstructionData::INT(i) => rhs = *i,
                        _ => panic!("unsupported type"),
                    }
                }
            }
            _ => panic!("unsupported type for execution"),
        }
        self.variables_map
            .insert(label.to_string(), InstructionData::INT(lhs + rhs));
        Some(InstructionData::INT(lhs + rhs))
    }

    fn execute_cond_br(
        &mut self,
        condition: &InstructionData,
        body: &Box<Instruction>,
        else_body: &Option<Box<Instruction>>,
    ) -> Option<InstructionData> {
        debug!("{:?}", condition);
        let mut condition_booleanness = false;
        match condition {
            InstructionData::INT(i) => condition_booleanness = *i > 0,
            InstructionData::FLOAT(f) => condition_booleanness = *f > 0.0,
            _ => panic!("unknown condition type"),
        }

        if condition_booleanness {
            self.execute_instruction(&body);
        } else if let Some(else_body_unwrapped) = else_body {
            self.execute_instruction(else_body_unwrapped);
        }
        None
    }
}
