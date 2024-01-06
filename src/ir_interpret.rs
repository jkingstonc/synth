use std::{collections::HashMap, string, time::Instant};

use log::{debug, info};

use crate::ir::{Instruction, InstructionData, Ref};

pub struct IRInterpreter {
    pub counter: usize,
    // pub instruction: Instruction,
    // todo for now this is an i32 but should be a generic 'value'
    pub variables_map: HashMap<std::string::String, i32>,
}

/*
This will walk through the instructions and evaluate them (no JIT yet :().
Still a massive WIP as we need to decide on the instruction model (SSA [Single Static Assignment] etc).
*/
impl IRInterpreter {
    pub fn execute(&mut self, instruction: Instruction) {
        let now = Instant::now();
        self.execute_instruction(&instruction);
        let elapsed = now.elapsed();
        debug!(
            "execution time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
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
        debug!("vars {:?}", self.variables_map);
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
        debug!("ummm {:?} {:?}", left, right);
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
        debug!("vars {:?}", self.variables_map);
    }

    // pub fn execute_int(&mut self, int_instruction: Instruction) {
    //     // todo probably check this is actually an INT
    //     if let Some(data) = int_instruction.data {
    //         match data {
    //             InstructionData::INT(i) => self.variables_map.insert(self.counter, i),
    //             _ => panic!(),
    //         };
    //     };
    //     self.counter += 1;
    // }

    // pub fn execute_add(&mut self, add_instruction: Instruction) {
    //     if let Some(data) = add_instruction.data {
    //         match &data {
    //             InstructionData::DOUBLE_REF(first, second) => {
    //                 //// fetch the data from the instructions at the first & second
    //                 // let left = self.instructions.get(first.value);
    //                 // let right = self.instructions.get(second.value);

    //                 // let left_value: i32;
    //                 // match left {
    //                 //     Some(left_instr_value) => match left_instr_value.instruction_type {
    //                 //         InstructionType::INT => match left_instr_value.data {
    //                 //             InstructionData::INT(i) => left_value = i,
    //                 //             _ => panic!("invalid instruction value"),
    //                 //         },
    //                 //         _ => panic!("invalid instruction value"),
    //                 //     },
    //                 //     None => panic!(),
    //                 // }
    //                 // let right_value: i32;
    //                 // match right {
    //                 //     Some(right_instr_value) => match right_instr_value.instruction_type {
    //                 //         InstructionType::INT => match right_instr_value.data {
    //                 //             InstructionData::INT(i) => right_value = i,
    //                 //             _ => panic!("invalid instruction value"),
    //                 //         },
    //                 //         _ => panic!("invalid instruction value"),
    //                 //     },
    //                 //     None => panic!(),
    //                 // }
    //                 // let value = left_value + right_value;
    //                 // self.variables_map.insert(self.counter, value);
    //                 // debug!("{:?} + {:?} = {:?}", left_value, right_value, value);
    //                 // let left = self.variables_map.get(&first.value);
    //                 // let right = self.variables_map.get(&second.value);

    //                 // let left_value: i32;
    //                 // match left {
    //                 //     Some(left_instr_value) => left_value = *left_instr_value,
    //                 //     None => panic!(),
    //                 // }
    //                 // let right_value: i32;
    //                 // match right {
    //                 //     Some(right_instr_value) => right_value = *right_instr_value,
    //                 //     None => panic!(),
    //                 // }
    //                 // let value = left_value + right_value;
    //                 // self.variables_map.insert(self.counter, value);
    //                 // debug!("{:?} + {:?} = {:?}", left_value, right_value, value);
    //             }
    //             _ => panic!(),
    //         }
    //     }
    //     self.counter += 1;
    // }

    // pub fn execute_var(&mut self, add_instruction: Instruction) {
    //     todo!();
    //     self.counter += 1;
    // }
}
