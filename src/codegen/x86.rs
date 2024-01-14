use log::{debug, error, info, warn};
use std::io::Write;
use std::time::Instant;
use std::{fs, process::Command};

use crate::ir::{IRValue, Instruction};
pub struct X86CodeGenerator {
    pub str_buffer: String,
}

impl X86CodeGenerator {
    pub fn generate(&mut self, instruction: &Instruction) {
        let now = Instant::now();

        match fs::create_dir_all("./build") {
            Ok(_) => {}
            Err(err) => {
                error!("failed to create build directory /build {}.", err);
                panic!("failed to create build directory /build {}.", err);
            }
        }

        // win32 hello world (linking with libc)
        self.str_buffer +=
            "global _main\nextern _printf\nsection .text\n_main:\npush message\ncall _printf\nadd esp, 4\nret\nmessage:\ndb 'Hello, World', 10, 0";

        // *nix hello world
        // self.str_buffer +=
        // "section    .text\nglobal _start\n_start:\nmov edx, len\nmov ecx, msg\n\nmov ebx, 1\n\nmov eax, 4\nint 0x80\n\nmov eax, 1\nint 0x80";
        // self.str_buffer += "\nsection    .data\nmsg    db \"hello, world!\"\nlen    equ $ -msg";

        // self.generate_instruction(instruction);

        // win32 (-lmsvcrt links with libc)
        // nasm -f wind32 -o build.o build.asm
        // ld -m i386pe -o hello hello.o -lmsvcrt

        // *nix
        // nasm -f elf32 -o build.o build.asm
        // ld -m elf_i386 -o hello hello.o

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

        Command::new("nasm")
            .args(["-f", "win32", "-o", "./build/build.o", "./build/build.asm"])
            .spawn()
            .expect("failed to assemble ./build/build.asm");
        Command::new("ld")
            .args([
                "-m",
                "i386pe",
                "-o",
                "./build/build.exe",
                "./build/build.o",
                "-lmsvcrt",
            ])
            .spawn()
            .expect("failed to link ./build/build.o");

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

    fn instruction_data_to_asm_string(&self, instruction_data: &IRValue) -> String {
        match instruction_data {
            IRValue::INT(i) => i.to_string(),
            IRValue::FLOAT(f) => f.to_string(),
            _ => todo!("need to implement IRValue to string"),
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

    fn generate_stack_var(&mut self, label: &String, instruction_data: &Option<IRValue>) {
        if let Some(val) = instruction_data {
            self.str_buffer += &format!("mov eax, {}", self.instruction_data_to_asm_string(val));
        }
    }
}
