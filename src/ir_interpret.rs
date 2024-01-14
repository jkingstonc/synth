use std::{collections::HashMap, string, time::Instant};

use log::{debug, info};

use crate::{
    compiler::CompilerOptions,
    ir::{IRValue, Instruction, Ref},
};

pub struct IRInterpreter<'a> {
    pub compiler_options: &'a CompilerOptions,
    pub counter: usize,
    // pub instruction: Instruction,
    // todo for now this is an i32 but should be a generic 'value'
    // this should be a symtable
    pub variables_map: HashMap<String, IRValue>,
}

/*
This will walk through the instructions and evaluate them (no JIT yet :().
Still a massive WIP as we need to decide on the instruction model (SSA [Single Static Assignment] etc).
*/
impl IRInterpreter<'_> {
    pub fn execute(&mut self, instruction: &Instruction) -> Option<IRValue> {
        let now = Instant::now();
        self.variables_map.insert(
            "printf".to_string(),
            IRValue::INTRINSIC("printf".to_string()),
        );
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

    fn execute_instruction(&mut self, instruction: &Instruction) -> Option<IRValue> {
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

    fn execute_program(&mut self, instructions: Box<Vec<Instruction>>) -> Option<IRValue> {
        let mut result: Option<IRValue> = None;
        for instruction in instructions.to_vec() {
            result = self.execute_instruction(&instruction);
        }
        result
    }

    fn excecute_block(&mut self, instructions: Box<Vec<Instruction>>) -> Option<IRValue> {
        let mut result: Option<IRValue> = None;
        for instruction in instructions.to_vec() {
            result = self.execute_instruction(&instruction);
        }
        result
    }

    fn execute_load(&mut self, label: &String, ref_value: &Ref) -> Option<IRValue> {
        if let Some(val) = self.variables_map.get(&ref_value.value) {
            self.variables_map.insert(label.to_string(), val.clone());
        } else if ref_value.value == "printf" {
            self.variables_map
                .insert(label.to_string(), IRValue::INTRINSIC("printf".to_string()));
        } else if ref_value.value == "SYNTH_FILENAME" {
            self.variables_map.insert(
                label.to_string(),
                IRValue::STRING(self.compiler_options.current_file.to_string()),
            );
        } else {
            panic!("couldn't find var");
        }
        None
    }

    fn execute_stack_var(&mut self, label: &String, value: &Option<IRValue>) -> Option<IRValue> {
        if let Some(data) = value {
            match data {
                IRValue::INT(i) => self.variables_map.insert(label.to_string(), data.clone()),
                IRValue::FLOAT(f) => self.variables_map.insert(label.to_string(), data.clone()),
                IRValue::STRING(s) => self.variables_map.insert(label.to_string(), data.clone()),
                IRValue::REF(r) => {
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

    fn execute_call(&mut self, label: &String, callee: &String, arg: &IRValue) -> Option<IRValue> {
        debug!("ummm {} {:?}", callee.to_string(), self.variables_map);
        let mut callee_data = self
            .variables_map
            .get(callee)
            .expect("could not find var")
            .clone();

        match callee_data {
            IRValue::INTRINSIC(i) => {
                if i == "printf" {
                    // assume that args are passed in as locals
                    let mut arg_data: IRValue;

                    // todo global formatter in ir.rs
                    match arg {
                        IRValue::REF(r) => {
                            arg_data = self
                                .variables_map
                                .get(&r.value)
                                .expect("expected arg as ref")
                                .clone()
                        }
                        IRValue::INT(_) => arg_data = arg.clone(),
                        IRValue::STRING(_) => arg_data = arg.clone(),
                        _ => panic!("couldn't print instruction data :("),
                    };

                    match arg_data {
                        IRValue::INT(i) => println!("{}", i),
                        IRValue::FLOAT(f) => println!("{}", f),
                        IRValue::STRING(s) => println!("{}", s),
                        _ => panic!("couldn't print IRValue"),
                    };
                }
            }
            _ => panic!("callee must be function or intrinsic"),
        };
        None
    }

    fn execute_add(&mut self, label: &String, left: &IRValue, right: &IRValue) -> Option<IRValue> {
        let mut lhs = 0;
        let mut rhs = 0;
        match left {
            IRValue::INT(i) => lhs = *i,
            IRValue::REF(r) => {
                if let Some(v) = self.variables_map.get(&r.value) {
                    match v {
                        IRValue::INT(i) => lhs = *i,
                        _ => panic!("unsupported type"),
                    }
                }
            }
            _ => panic!("unsupported type for execution"),
        }
        match right {
            IRValue::INT(i) => rhs = *i,
            IRValue::REF(r) => {
                if let Some(v) = self.variables_map.get(&r.value) {
                    match v {
                        IRValue::INT(i) => rhs = *i,
                        _ => panic!("unsupported type"),
                    }
                }
            }
            _ => panic!("unsupported type for execution"),
        }
        self.variables_map
            .insert(label.to_string(), IRValue::INT(lhs + rhs));
        Some(IRValue::INT(lhs + rhs))
    }

    fn evaluate_instruction_data_for_booleanness(&self, value: &IRValue) -> bool {
        match value {
            IRValue::REF(r) => {
                let val = self.variables_map.get(&r.value).expect("couldn't find var");
                self.evaluate_instruction_data_for_booleanness(val)
            }
            IRValue::INT(i) => *i > 0,
            IRValue::FLOAT(f) => *f > 0.0,
            _ => panic!("unknown condition type"),
        }
    }

    fn execute_cond_br(
        &mut self,
        condition: &IRValue,
        body: &Box<Instruction>,
        else_body: &Option<Box<Instruction>>,
    ) -> Option<IRValue> {
        debug!("{:?}", condition);
        let condition_booleanness = self.evaluate_instruction_data_for_booleanness(condition);
        if condition_booleanness {
            self.execute_instruction(&body);
        } else if let Some(else_body_unwrapped) = else_body {
            self.execute_instruction(else_body_unwrapped);
        }
        None
    }
}
