extern crate llvm_sys;
use llvm_sys::core::{
    LLVMBuildCall2, LLVMBuildGlobalStringPtr, LLVMConstInt, LLVMConstPointerNull, LLVMInt32Type,
    LLVMPositionBuilder, LLVMVoidType,
};
use llvm_sys::prelude::{LLVMModuleRef, LLVMValueRef};
use llvm_sys::{LLVMBasicBlock, LLVMBuilder, LLVMValue};
use log::{debug, error, info, warn};
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::{fs, process::Command};

use crate::ir::{Instruction, InstructionData, Ref};
use crate::symtable::SymTable;
pub struct LLVMCodeGenerator {
    pub str_buffer: String,
    pub sym_table: SymTable<std::string::String, LLVMValueRef>,
}

/*
References:
- https://github.com/lyledean1/calculon/blob/main/src/main.rs
- https://medium.com/@jayphelps/using-llvm-from-rust-to-generate-webassembly-93e8c193fdb4
*/
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

            self.generate_builtins(module, builder);

            // Get the type signature for void nop(void);
            // Then create it in our module.
            let void = llvm_sys::core::LLVMVoidTypeInContext(context);
            let i32_type = llvm_sys::core::LLVMInt32Type();
            let function_type = llvm_sys::core::LLVMFunctionType(void, std::ptr::null_mut(), 0, 0);
            let function = llvm_sys::core::LLVMAddFunction(
                module,
                b"main\0".as_ptr() as *const _,
                function_type,
            );

            // Create a basic block in the function and set our builder to generate
            // code in it.
            let bb = llvm_sys::core::LLVMAppendBasicBlockInContext(
                context,
                function,
                b"entry\0".as_ptr() as *const _,
            );

            // let c_str = CString::new("hi").expect("i am a c string");
            // let ptr = c_str.as_ptr();
            // let glob = llvm_sys::core::LLVMAddGlobal(module, llvm_sys::core::LLVMInt32Type(), ptr);

            // llvm_sys::core::LLVMPositionBuilder(builder, bb, function);
            llvm_sys::core::LLVMPositionBuilderAtEnd(builder, bb);

            self.generate_instruction(instruction, builder, bb);

            // let c_str = CString::new("hi").unwrap();
            // let ddd = c_str.as_ptr();
            // llvm_sys::core::LLVMBuildAlloca(
            //     builder,
            //     llvm_sys::core::LLVMInt64TypeInContext(context),
            //     b"asdgasdg\0".as_ptr() as *const _,
            // );

            // let add = llvm_sys::core::LLVMBuildAdd(
            //     builder,
            //     LLVMConstInt(llvm_sys::core::LLVMInt32Type(), 44 as u64, 1),
            //     LLVMConstInt(llvm_sys::core::LLVMInt32Type(), 55 as u64, 1),
            //     add1,
            // );
            // let add2 = llvm_sys::core::LLVMBuildAdd(
            //     builder,
            //     add,
            //     LLVMConstInt(llvm_sys::core::LLVMInt32Type(), 555 as u64, 1),
            //     c_str,
            // );

            llvm_sys::core::LLVMPositionBuilderAtEnd(builder, bb);

            // Emit a `ret void` into the function
            llvm_sys::core::LLVMBuildRetVoid(builder);

            let s = llvm_sys::core::LLVMPrintModuleToString(module);
            let contents_str = CStr::from_ptr(s).to_str().unwrap();
            let mut ir_file = File::create("./build/build.ir").expect("unable to create file");
            debug!("ir_file {:?}", ir_file);
            if let Err(_) = ir_file.write_all(contents_str.as_bytes()) {
                panic!("failed to write ir");
            }

            // Clean up. Values created in the context mostly get cleaned up there.
            llvm_sys::core::LLVMDisposeBuilder(builder);
            llvm_sys::core::LLVMDisposeModule(module);
            llvm_sys::core::LLVMContextDispose(context);

            Command::new("llc")
                .args(["./build/build.ir", "-o", "./build/build.s"])
                .output()
                .expect("failed to build ./build/build.ir");

            Command::new("clang")
                .args(["-c", "./build/build.s", "-o", "./build/build.o"])
                .output()
                .expect("failed to build ./build/build.s");

            Command::new("clang")
                .args(["./build/build.o", "-o", "./build/build.exe"])
                .output()
                .expect("failed to build ./build/build.o");
        }

        let elapsed = now.elapsed();
        debug!(
            "codegen time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
    }

    fn generate_builtins(
        &mut self,
        module: LLVMModuleRef,
        builder: *mut LLVMBuilder,
        // current_block: *mut LLVMBasicBlock,
    ) {
        unsafe {
            let function_type =
                llvm_sys::core::LLVMFunctionType(LLVMInt32Type(), std::ptr::null_mut(), 0, 0);
            let function = llvm_sys::core::LLVMAddFunction(
                module,
                b"printf\0".as_ptr() as *const _,
                function_type,
            );
            self.sym_table.add("printf".to_string(), function);
        }
    }

    fn generate_instruction(
        &mut self,
        instruction: &Instruction,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
    ) -> Option<*mut LLVMValue> {
        match instruction {
            Instruction::PROGRAM(instructions) => {
                self.generate_program(instructions, builder, current_block)
            }
            Instruction::ADD(location, first, second) => {
                self.generate_add(location, first, second, builder, current_block)
            }
            Instruction::STACK_VAR(location, value) => {
                self.generate_stack_var(location, value, builder, current_block)
            }
            Instruction::LOAD(location, value) => {
                self.generate_load(location, value, builder, current_block)
            }
            Instruction::CALL(location, callee, arg) => {
                self.generate_call(location, callee, arg, builder, current_block)
            }
            // Instruction::BLOCK(label, block) => self.generate_block(label, block),
            // Instruction::STACK_VAR(label, instruction_data) => {
            //     self.generate_stack_var(label, instruction_data)
            // }
            _ => panic!("unsupported instruction {:?}", instruction),
        }
    }

    fn instruction_data_to_llvm_value_ref(&mut self) -> LLVMValueRef {
        unsafe { llvm_sys::core::LLVMConstInt(llvm_sys::core::LLVMInt32Type(), 4, 1) }
    }

    fn generate_program(
        &mut self,
        instructions: &Box<Vec<Instruction>>,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
    ) -> Option<*mut LLVMValue> {
        for instruction in instructions.iter() {
            self.generate_instruction(instruction, builder, current_block);
        }
        None
    }

    fn generate_load(
        &mut self,
        label: &String,
        value: &Ref,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
    ) -> Option<*mut LLVMValue> {
        // unsafe { llvm_sys::core::LLVMBuildLoad2(builder, Ty, PointerVal, Name) }
        debug!("... doing load label {:?} value {:?}", label, value);
        None
    }

    fn generate_call(
        &mut self,
        label: &String,
        callee: &InstructionData,
        arg: &InstructionData,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            match callee {
                InstructionData::REF(r) => {
                    // let func_value = self
                    //     .sym_table
                    //     .get(r.value.to_string())
                    //     .expect("expected function value");

                    let func_value = self
                        .sym_table
                        .get("printf".to_owned())
                        .expect("expected printf");
                    let function_type = llvm_sys::core::LLVMFunctionType(
                        LLVMInt32Type(),
                        std::ptr::null_mut(),
                        0,
                        0,
                    );

                    debug!("printf {:?}", func_value);

                    let c_str = CString::new("hello, world!").expect("i am a c string");
                    let ptr = c_str.as_ptr();
                    let c_str_var = CString::new("hello_world").expect("i am a c string");
                    let ptr_var = c_str_var.as_ptr();
                    let mut string_value = LLVMBuildGlobalStringPtr(builder, ptr, ptr_var);
                    let printf_var = CString::new("printf").expect("i am a c string");
                    let printf_var_ptr = printf_var.as_ptr();
                    LLVMBuildCall2(
                        builder,
                        function_type,
                        *func_value,
                        // &mut string_value,
                        &mut LLVMConstPointerNull(LLVMVoidType()),
                        0,
                        printf_var_ptr,
                    );
                }
                _ => panic!(),
            }
        }
        None
    }

    fn generate_stack_var(
        &mut self,
        label: &String,
        value: &Option<InstructionData>,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            let c_str = CString::new(label.as_str()).unwrap();
            let ptr = c_str.as_ptr();
            let alloca_instruction =
                llvm_sys::core::LLVMBuildAlloca(builder, llvm_sys::core::LLVMInt32Type(), ptr);
            if let Some(val) = value {
                match val {
                    InstructionData::REF(r) => {
                        let initializer_value =
                            self.sym_table.get(r.value.to_string()).unwrap().clone();
                        llvm_sys::core::LLVMBuildStore(
                            builder,
                            initializer_value,
                            alloca_instruction,
                        );
                    }
                    InstructionData::INT(i) => {
                        llvm_sys::core::LLVMBuildStore(
                            builder,
                            LLVMConstInt(
                                llvm_sys::core::LLVMInt32Type(),
                                (*i).try_into().unwrap(),
                                1,
                            ),
                            alloca_instruction,
                        );
                    }
                    _ => todo!(),
                }
            }
            None
        }
    }

    fn generate_add(
        &mut self,
        location: &String,
        first: &InstructionData,
        second: &InstructionData,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            let mut left: LLVMValueRef;
            let mut right: LLVMValueRef;
            match first {
                InstructionData::INT(i) => {
                    left = LLVMConstInt(llvm_sys::core::LLVMInt32Type(), *i as u64, 1)
                }
                _ => todo!("not implemented"),
            }
            match second {
                InstructionData::INT(i) => {
                    right = LLVMConstInt(llvm_sys::core::LLVMInt32Type(), *i as u64, 1)
                }
                _ => todo!("not implemented"),
            }

            let add_instr = llvm_sys::core::LLVMBuildAdd(
                builder,
                left,
                right,
                location.to_owned().as_bytes().as_ptr() as *const i8,
            );

            self.sym_table.add(location.to_string(), add_instr);

            // llvm_sys::core::LLVMPositionBuilderAtEnd(builder, current_block);
            // llvm_sys::core::LLVMBuildRet(builder, add_instr);
            None
        }
    }
}
