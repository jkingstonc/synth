use log::debug;

use crate::ir::{Instruction, InstructionData, InstructionType};

pub struct IRInterpreter {
    pub counter: usize,
    pub instructions: Box<Vec<Instruction>>,
}

impl IRInterpreter {
    pub fn execute(&mut self) {
        while (self.counter < self.instructions.len()) {
            match self.instructions.get(self.counter) {
                Some(instruction) => self.execute_instr(instruction.clone()),
                None => panic!(),
            }
        }
    }

    pub fn execute_instr(&mut self, instruction: Instruction) {
        match instruction.instruction_type {
            InstructionType::INT => self.execute_int(instruction),
            InstructionType::ADD => self.execute_add(instruction),
            _ => panic!(),
        }
    }

    pub fn execute_int(&mut self, int_instruction: Instruction) {
        self.counter += 1;
    }

    pub fn execute_add(&mut self, add_instruction: Instruction) {
        match &add_instruction.data {
            InstructionData::DOUBLE_REF(first, second) => {
                // fetch the data at the first & second
                let left = self.instructions.get(first.value);
                let right = self.instructions.get(second.value);

                let left_value: i32;
                match left {
                    Some(left_instr_value) => match left_instr_value.instruction_type {
                        InstructionType::INT => match left_instr_value.data {
                            InstructionData::INT(i) => left_value = i,
                            _ => panic!("invalid instruction value"),
                        },
                        _ => panic!("invalid instruction value"),
                    },
                    None => panic!(),
                }
                let right_value: i32;
                match right {
                    Some(right_instr_value) => match right_instr_value.instruction_type {
                        InstructionType::INT => match right_instr_value.data {
                            InstructionData::INT(i) => right_value = i,
                            _ => panic!("invalid instruction value"),
                        },
                        _ => panic!("invalid instruction value"),
                    },
                    None => panic!(),
                }
                debug!(
                    "{:?} + {:?} = {:?}",
                    left_value,
                    right_value,
                    left_value + right_value
                );
            }
            _ => panic!(),
        }
        self.counter += 1;
    }
}
