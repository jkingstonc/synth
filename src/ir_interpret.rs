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
    pub variables_map: HashMap<std::string::String, i32>,
}

/*
This will walk through the instructions and evaluate them (no JIT yet :().
Still a massive WIP as we need to decide on the instruction model (SSA [Single Static Assignment] etc).
*/
impl IRInterpreter<'_> {
    pub fn execute(&mut self, instruction: &Instruction) {
        let now = Instant::now();
        self.execute_instruction(instruction);
        debug!("vars {:?}", self.variables_map);
        let elapsed = now.elapsed();
        debug!(
            "execution time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::PROGRAM(instructions) => self.execute_program(instructions.clone()),
            Instruction::BLOCK(_, instructions) => self.excecute_block(instructions.clone()),
            Instruction::STACK_VAR(label, value) => self.execute_stack_var(label, value),
            Instruction::LOAD(label, value) => self.execute_load(label, value),
            Instruction::ADD(label, left, right) => self.execute_add(label, left, right),
            // InstructionType::INT => self.execute_int(instruction),
            // InstructionType::ADD => self.execute_add(instruction),
            // InstructionType::STACK_VAR => self.execute_var(instruction),
            _ => panic!(),
        }
    }

    fn execute_program(&mut self, instructions: Box<Vec<Instruction>>) {
        for instruction in instructions.to_vec() {
            self.execute_instruction(&instruction);
        }
    }

    fn excecute_block(&mut self, instructions: Box<Vec<Instruction>>) {
        for instruction in instructions.to_vec() {
            self.execute_instruction(&instruction);
        }
    }

    fn execute_load(&mut self, label: &std::string::String, ref_value: &Ref) {
        if let Some(val) = self.variables_map.get(&ref_value.value) {
            self.variables_map.insert(label.to_string(), *val);
        } else {
            panic!("couldn't find var");
        }
    }

    fn execute_stack_var(&mut self, label: &std::string::String, value: &Option<InstructionData>) {
        if let Some(data) = value {
            match data {
                InstructionData::INT(i) => self.variables_map.insert(label.to_string(), *i),
                InstructionData::REF(r) => {
                    if let Some(v) = self.variables_map.get(&r.value) {
                        self.variables_map.insert(label.to_string(), *v);
                    } else {
                        panic!("couldn't find var");
                    };
                    None
                }
                _ => panic!("unsupported type for execution"),
            };
        };
    }
    fn execute_add(
        &mut self,
        label: &std::string::String,
        left: &InstructionData,
        right: &InstructionData,
    ) {
        let result = 0;

        let mut lhs = 0;
        let mut rhs = 0;
        match left {
            InstructionData::INT(i) => lhs = *i,
            InstructionData::REF(r) => {
                if let Some(v) = self.variables_map.get(&r.value) {
                    lhs = *v;
                }
            }
            _ => panic!("unsupported type for execution"),
        }
        match right {
            InstructionData::INT(i) => rhs = *i,
            InstructionData::REF(r) => {
                if let Some(v) = self.variables_map.get(&r.value) {
                    rhs = *v;
                }
            }
            _ => panic!("unsupported type for execution"),
        }
        self.variables_map.insert(label.to_string(), lhs + rhs);
    }
}
