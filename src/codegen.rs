use log::{debug, error, info, warn};
use std::fs;
use std::io::Write;
use std::time::Instant;

use crate::ir::{Instruction, InstructionData};
pub struct X86CodeGenerator {
    pub str_buffer: String,
}

impl X86CodeGenerator {
    pub fn generate(&mut self, instruction: &Instruction) {
        let now = Instant::now();

        self.generate_instruction(instruction);

        match fs::create_dir_all("./build") {
            Ok(_) => {}
            Err(err) => {
                error!("failed to create build directory /build {}.", err);
                panic!("failed to create build directory /build {}.", err);
            }
        }

        let mut file = match fs::File::create("./build/build.asm") {
            Err(err) => {
                error!("failed to create asm {}.", err);
                panic!("failed to create asm {}.", err);
            }
            Ok(file) => file,
        };

        match file.write_all(self.str_buffer.as_bytes()) {
            Err(err) => {
                error!("failed to write to asm {}.", err);
                panic!("failed to write to asm {}.", err);
            }
            Ok(file) => {}
        }

        let elapsed = now.elapsed();
        debug!(
            "codegen time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
    }

    fn generate_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::BLOCK(label, block) => self.generate_block(label, block),
            Instruction::STACK_VAR(label, instruction_data) => {
                self.generate_stack_var(label, instruction_data)
            }
            _ => panic!("unsupported instruction"),
        };
    }

    fn instruction_data_to_asm_string(&self, instruction_data: &InstructionData) -> String {
        match instruction_data {
            InstructionData::INT(i) => i.to_string(),
            InstructionData::FLOAT(f) => f.to_string(),
            _ => todo!("need to implement InstructionData to string"),
        }
    }

    fn generate_block(&mut self, label: &String, block: &Box<Vec<Instruction>>) {
        for instruction in block.iter() {
            self.generate_instruction(instruction);
        }
    }

    fn generate_stack_var(&mut self, label: &String, instruction_data: &Option<InstructionData>) {
        if let Some(val) = instruction_data {
            self.str_buffer += &format!("mov eax, {}", self.instruction_data_to_asm_string(val));
        }
    }
}
