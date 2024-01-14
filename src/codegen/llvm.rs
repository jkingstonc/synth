extern crate llvm_sys;
use llvm_sys::core::{
    LLVMAppendBasicBlock, LLVMAppendBasicBlockInContext, LLVMArrayType, LLVMArrayType2,
    LLVMBasicBlockAsValue, LLVMBuildAlloca, LLVMBuildBr, LLVMBuildCall2, LLVMBuildCondBr,
    LLVMBuildGlobalString, LLVMBuildGlobalStringPtr, LLVMBuildLoad2, LLVMBuildStore, LLVMConstInt,
    LLVMConstPointerNull, LLVMCreateBasicBlockInContext, LLVMGetNamedGlobal, LLVMInt1Type,
    LLVMInt32Type, LLVMInt8Type, LLVMPointerType, LLVMPositionBuilder, LLVMPositionBuilderAtEnd,
    LLVMVoidType,
};
use llvm_sys::execution_engine::LLVMGetGlobalValueAddress;
use llvm_sys::prelude::{LLVMModuleRef, LLVMValueRef};
use llvm_sys::{LLVMBasicBlock, LLVMBuilder, LLVMContext, LLVMValue};
use log::{debug, error, info, warn};
use std::ffi::{CStr, CString};
use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::{fs, process::Command};

use crate::ir::{IRValue, Instruction, Ref};
use crate::symtable::SymTable;
pub struct LLVMCodeGenerator {
    pub anon_string_counter: usize,
    pub anon_local_block_counter: usize,
    pub str_buffer: String,
    pub sym_table: SymTable<String, LLVMValueRef>,
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

            llvm_sys::core::LLVMPositionBuilderAtEnd(builder, bb);

            self.generate_instruction(instruction, context, builder, bb, function);

            // llvm_sys::core::LLVMPositionBuilderAtEnd(builder, bb);

            // Emit a `ret void` into the function
            llvm_sys::core::LLVMBuildRetVoid(builder);

            let s = llvm_sys::core::LLVMPrintModuleToString(module);
            let contents_str = CStr::from_ptr(s).to_str().unwrap();
            let mut ir_file = File::create("./build/build.ir").expect("unable to create file");
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
            let function_type = llvm_sys::core::LLVMFunctionType(
                LLVMInt32Type(),
                // std::ptr::null_mut(),
                &mut LLVMPointerType(LLVMInt8Type(), 0),
                1,
                0,
            );
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
        context: *mut LLVMContext,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        match instruction {
            Instruction::PROGRAM(instructions) => self.generate_program(
                instructions,
                context,
                builder,
                current_block,
                current_function,
            ),
            Instruction::BLOCK(name, instructions) => self.generate_block(
                instructions,
                context,
                builder,
                current_block,
                current_function,
            ),
            Instruction::ADD(location, first, second) => self.generate_add(
                location,
                first,
                second,
                context,
                builder,
                current_block,
                current_function,
            ),
            Instruction::STACK_VAR(location, value) => self.generate_stack_var(
                location,
                value,
                context,
                builder,
                current_block,
                current_function,
            ),
            Instruction::LOAD(location, value) => self.generate_load(
                location,
                value,
                context,
                builder,
                current_block,
                current_function,
            ),
            Instruction::CALL(location, callee, arg) => self.generate_call(
                location,
                callee,
                arg,
                context,
                builder,
                current_block,
                current_function,
            ),
            // Instruction::BLOCK(label, block) => self.generate_block(label, block),
            // Instruction::STACK_VAR(label, instruction_data) => {
            //     self.generate_stack_var(label, instruction_data)
            // }
            Instruction::COND_BR(condition, body, else_body) => self.generate_cond_br(
                condition,
                body,
                else_body,
                context,
                builder,
                current_block,
                current_function,
            ),
            _ => panic!("unsupported instruction {:?}", instruction),
        }
    }

    fn instruction_data_to_llvm_value_ref(&mut self) -> LLVMValueRef {
        unsafe { llvm_sys::core::LLVMConstInt(llvm_sys::core::LLVMInt32Type(), 4, 1) }
    }

    fn generate_program(
        &mut self,
        instructions: &Box<Vec<Instruction>>,
        context: *mut LLVMContext,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        for instruction in instructions.iter() {
            self.generate_instruction(
                instruction,
                context,
                builder,
                current_block,
                current_function,
            );
        }
        None
    }

    fn generate_block(
        &mut self,
        instructions: &Box<Vec<Instruction>>,
        context: *mut LLVMContext,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        for instruction in instructions.iter() {
            self.generate_instruction(
                instruction,
                context,
                builder,
                current_block,
                current_function,
            );
        }
        None
    }

    fn generate_load(
        &mut self,
        label: &String,
        value: &Ref,
        context: *mut LLVMContext,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            // LLVMBuildLoad2(builder, Ty, PointerVal, Name)>
            let label_var = CString::new(label.as_bytes()).expect("i am a c string");
            let label_var_ptr = label_var.as_ptr();
            let llvm_ptr_val = self
                .sym_table
                .get(value.value.to_string())
                .expect("expected value");
            let load_value = LLVMBuildLoad2(
                builder,
                LLVMPointerType(LLVMInt8Type(), 0),
                llvm_ptr_val.clone(),
                label_var_ptr,
            );
            self.sym_table.add(label.to_string(), load_value);
        }
        None
    }

    fn generate_cond_br(
        &mut self,
        condition: &IRValue,
        body: &Box<Instruction>,
        else_body: &Option<Box<Instruction>>,
        context: *mut LLVMContext,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        // todo get the value
        unsafe {
            let val = LLVMConstInt(LLVMInt1Type(), 1 as u64, 1);
            // Create a basic block in the function and set our builder to generate
            // code in it.
            let then_body_str =
                CString::new(format!("{}_block", self.anon_local_block_counter)).unwrap();
            self.anon_local_block_counter += 1;
            let then_body_str_ptr = then_body_str.as_ptr();
            let then_block =
                LLVMAppendBasicBlockInContext(context, current_function, then_body_str_ptr);
            // todo dont generate the else at all if we don't have it
            let else_body_str =
                CString::new(format!("{}_block", self.anon_local_block_counter)).unwrap();
            self.anon_local_block_counter += 1;
            let else_body_str_ptr = else_body_str.as_ptr();
            let else_block =
                LLVMAppendBasicBlockInContext(context, current_function, else_body_str_ptr);
            let done_str = CString::new(format!("{}_done", self.anon_local_block_counter)).unwrap();
            self.anon_local_block_counter += 1;
            let done_str_ptr = done_str.as_ptr();
            let done_block = LLVMAppendBasicBlockInContext(context, current_function, done_str_ptr);

            LLVMBuildCondBr(builder, val, then_block, else_block);

            LLVMPositionBuilderAtEnd(builder, then_block);
            self.generate_instruction(&body, context, builder, current_block, current_function);
            LLVMBuildBr(builder, done_block);

            // print positioning here!
            LLVMPositionBuilderAtEnd(builder, else_block);
            if let Some(else_body_instruction) = else_body {
                self.generate_instruction(
                    else_body_instruction,
                    context,
                    builder,
                    current_block,
                    current_function,
                );
            }
            LLVMBuildBr(builder, done_block);

            LLVMPositionBuilderAtEnd(builder, done_block);
        }

        None
    }

    fn generate_call(
        &mut self,
        label: &String,
        callee: &String,
        arg: &IRValue,
        context: *mut LLVMContext,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            let func_value = self
                .sym_table
                .get(callee.to_owned())
                .expect("expected printf");

            let function_type = llvm_sys::core::LLVMFunctionType(
                LLVMInt32Type(),
                &mut LLVMPointerType(LLVMInt8Type(), 0),
                1,
                0,
            );

            let printf_var = CString::new("call_result").expect("i am a c string");
            let printf_var_ptr = printf_var.as_ptr();

            match arg {
                IRValue::REF(r) => {
                    let mut arg0 = self
                        .sym_table
                        .get(r.value.to_string())
                        .expect("expected value")
                        .clone();

                    LLVMBuildCall2(
                        builder,
                        function_type,
                        *func_value,
                        &mut arg0,
                        // &mut LLVMConstPointerNull(LLVMVoidType()),
                        1,
                        printf_var_ptr,
                    );
                }
                IRValue::STRING(s) => {
                    let global_label =
                        CString::new(format!("{}_anon_string", self.anon_string_counter))
                            .expect("i am a c string");
                    let global_label_ptr = global_label.as_ptr();
                    let s_value = CString::new(s.to_string()).expect("i am a c string");
                    let s_value_ptr = s_value.as_ptr();
                    let mut string_value =
                        LLVMBuildGlobalString(builder, s_value_ptr, global_label_ptr);

                    // let mut arg0 = self.sym_table.get("x".to_string()).unwrap().clone();

                    LLVMBuildCall2(
                        builder,
                        function_type,
                        *func_value,
                        &mut string_value,
                        // &mut LLVMConstPointerNull(LLVMVoidType()),
                        1,
                        printf_var_ptr,
                    );

                    self.anon_string_counter += 1;
                }
                _ => todo!(),
            }
        }
        None
    }

    fn generate_stack_var(
        &mut self,
        label: &String,
        value: &Option<IRValue>,
        context: *mut LLVMContext,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            let c_str = CString::new(label.as_str()).unwrap();
            let ptr = c_str.as_ptr();
            if let Some(val) = value {
                match val {
                    IRValue::REF(r) => {
                        let alloca_instruction = llvm_sys::core::LLVMBuildAlloca(
                            builder,
                            llvm_sys::core::LLVMInt32Type(),
                            ptr,
                        );
                        let initializer_value =
                            self.sym_table.get(r.value.to_string()).unwrap().clone();
                        let store_instruction = llvm_sys::core::LLVMBuildStore(
                            builder,
                            initializer_value,
                            alloca_instruction,
                        );
                        self.sym_table.add(label.to_string(), store_instruction);
                    }
                    IRValue::INT(i) => {
                        let alloca_instruction = llvm_sys::core::LLVMBuildAlloca(
                            builder,
                            llvm_sys::core::LLVMInt32Type(),
                            ptr,
                        );
                        let store_instruction = llvm_sys::core::LLVMBuildStore(
                            builder,
                            LLVMConstInt(
                                llvm_sys::core::LLVMInt32Type(),
                                (*i).try_into().unwrap(),
                                1,
                            ),
                            alloca_instruction,
                        );
                        self.sym_table.add(label.to_string(), store_instruction);
                    }
                    IRValue::STRING(s) => {
                        // first allocate space for the global string
                        let c_str = CString::new(s.to_string()).expect("i am a c string");
                        let ptr = c_str.as_ptr();
                        let c_str_label =
                            CString::new(format!("{}_anon_string", self.anon_string_counter))
                                .expect("i am a c string");
                        self.anon_string_counter += 1;
                        let ptr_label = c_str_label.as_ptr();
                        let mut llvm_string_value = LLVMBuildGlobalString(builder, ptr, ptr_label);

                        // then we need to load the string
                        let label_str = CString::new(label.to_string()).unwrap();
                        let label_str_ptr = label_str.as_ptr();
                        // allocate space for a pointer
                        let tmp_name = CString::new(format!("{}.0", label)).unwrap();
                        let tmp_ptr = tmp_name.as_ptr();
                        let alloca_instruction =
                            LLVMBuildAlloca(builder, LLVMPointerType(LLVMInt8Type(), 0), tmp_ptr);
                        // then store the pointer to the str in that pointer
                        LLVMBuildStore(builder, llvm_string_value, alloca_instruction);
                        // then actually load the pointer value onto the stack
                        let mut load = LLVMBuildLoad2(
                            builder,
                            LLVMPointerType(LLVMInt8Type(), 0),
                            alloca_instruction,
                            label_str_ptr,
                        );

                        self.sym_table.add(label.to_string(), load);
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
        first: &IRValue,
        second: &IRValue,
        context: *mut LLVMContext,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            let mut left: LLVMValueRef;
            let mut right: LLVMValueRef;
            match first {
                IRValue::INT(i) => {
                    left = LLVMConstInt(llvm_sys::core::LLVMInt32Type(), *i as u64, 1)
                }
                _ => todo!("not implemented"),
            }
            match second {
                IRValue::INT(i) => {
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
