use log::{debug, error, info, warn};
use std::io::Write;
use std::time::Instant;
use std::{fs, process::Command};

use crate::ir::{Instruction, InstructionData};
pub struct LLVMCodeGenerator {
    pub str_buffer: String,
}

impl LLVMCodeGenerator {
    pub fn generate(&mut self, instruction: &Instruction) {
        let now = Instant::now();

        match fs::create_dir_all("./build") {
            Ok(_) => {}
            Err(err) => {
                error!("failed to create build directory /build {}.", err);
                panic!("failed to create build directory /build {}.", err);
            }
        }

        // self.generate_instruction(instruction);

        unsafe {
            println!("creating context.");
            let context = llvm_sys::core::LLVMContextCreate();
            let module =
                llvm_sys::core::LLVMModuleCreateWithName(b"my_module\0".as_ptr() as *const _);
            let builder = llvm_sys::core::LLVMCreateBuilderInContext(context);

            // Get the type signature for void nop(void);
            // Then create it in our module.
            let void = llvm_sys::core::LLVMVoidTypeInContext(context);
            let function_type = llvm_sys::core::LLVMFunctionType(void, std::ptr::null_mut(), 0, 0);
            let function = llvm_sys::core::LLVMAddFunction(
                module,
                b"nop\0".as_ptr() as *const _,
                function_type,
            );

            // Create a basic block in the function and set our builder to generate
            // code in it.
            let bb = llvm_sys::core::LLVMAppendBasicBlockInContext(
                context,
                function,
                b"entry\0".as_ptr() as *const _,
            );
            llvm_sys::core::LLVMPositionBuilderAtEnd(builder, bb);

            // Emit a `ret void` into the function
            llvm_sys::core::LLVMBuildRetVoid(builder);

            // Dump the module as IR to stdout.
            llvm_sys::core::LLVMDumpModule(module);

            // Clean up. Values created in the context mostly get cleaned up there.
            llvm_sys::core::LLVMDisposeBuilder(builder);
            llvm_sys::core::LLVMDisposeModule(module);
            llvm_sys::core::LLVMContextDispose(context);
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
            Instruction::PROGRAM(instructions) => self.generate_program(instructions),
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

    fn generate_program(&mut self, instructions: &Box<Vec<Instruction>>) {
        for instruction in instructions.iter() {
            self.generate_instruction(instruction);
        }
    }

    fn generate_block(&mut self, label: &String, block: &Box<Vec<Instruction>>) {
        self.str_buffer += label;
        self.str_buffer += ":\n";
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
