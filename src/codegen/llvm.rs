extern crate llvm_sys;
use llvm_sys::core::{
    LLVMAppendBasicBlock, LLVMAppendBasicBlockInContext, LLVMArrayType, LLVMArrayType2,
    LLVMBasicBlockAsValue, LLVMBuildAlloca, LLVMBuildBitCast, LLVMBuildBr, LLVMBuildCall2,
    LLVMBuildCast, LLVMBuildCondBr, LLVMBuildGlobalString, LLVMBuildGlobalStringPtr, LLVMBuildICmp,
    LLVMBuildIntCast, LLVMBuildIntCast2, LLVMBuildLoad2, LLVMBuildRetVoid, LLVMBuildStore,
    LLVMConstInt, LLVMConstPointerNull, LLVMCreateBasicBlockInContext, LLVMGetNamedGlobal,
    LLVMInt1Type, LLVMInt32Type, LLVMInt8Type, LLVMPointerType, LLVMPositionBuilder,
    LLVMPositionBuilderAtEnd, LLVMVoidType,
};
use llvm_sys::execution_engine::LLVMGetGlobalValueAddress;
use llvm_sys::prelude::{LLVMModuleRef, LLVMValueRef};
use llvm_sys::{LLVMBasicBlock, LLVMBuilder, LLVMContext, LLVMModule, LLVMValue};
use log::{debug, error, info, warn};
use std::ffi::{CStr, CString};
use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::{fs, process::Command};

use crate::ir::{IRValue, Instruction, Ref};
use crate::symtable::SymTable;

pub struct LLVMValueBundle {
    pub llvm_value: LLVMValueRef,
    pub is_ref: bool,
}

pub struct LLVMCodeGenerator {
    pub anon_local_counter: usize,
    pub anon_string_counter: usize,
    pub anon_local_block_counter: usize,
    pub str_buffer: String,
    pub sym_table: SymTable<String, LLVMValueBundle>,
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

            self.generate_instruction(instruction, context, module, builder, bb, function);

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
            self.sym_table.add(
                "printf".to_string(),
                LLVMValueBundle {
                    llvm_value: function,
                    is_ref: false,
                },
            );
        }
    }

    fn generate_instruction(
        &mut self,
        instruction: &Instruction,
        context: *mut LLVMContext,
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        match instruction {
            Instruction::PROGRAM(instructions) => self.generate_program(
                instructions,
                context,
                module,
                builder,
                current_block,
                current_function,
            ),
            Instruction::BLOCK(name, instructions) => self.generate_block(
                instructions,
                context,
                module,
                builder,
                current_block,
                current_function,
            ),
            Instruction::ADD(location, first, second) => self.generate_add(
                location,
                first,
                second,
                context,
                module,
                builder,
                current_block,
                current_function,
            ),
            Instruction::STACK_VAR(location, value) => self.generate_stack_var(
                location,
                value,
                context,
                module,
                builder,
                current_block,
                current_function,
            ),
            Instruction::LOAD(location, value) => self.generate_load(
                location,
                value,
                context,
                module,
                builder,
                current_block,
                current_function,
            ),
            Instruction::CALL(location, callee, arg) => self.generate_call(
                location,
                callee,
                arg,
                context,
                module,
                builder,
                current_block,
                current_function,
            ),
            Instruction::STORE(storee, value) => self.generate_store(
                storee,
                value,
                context,
                module,
                builder,
                current_block,
                current_function,
            ),
            Instruction::FUNC(name, instruction) => self.generate_func(
                name,
                instruction,
                context,
                module,
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
                module,
                builder,
                current_block,
                current_function,
            ),
            _ => panic!("unsupported instruction {:?}", instruction),
        }
    }

    // todo this fundementally doesn't work as rust strings are awful
    fn string_to_c_str(&self, string: &String) -> *const i8 {
        let c = CString::new(string.to_string()).unwrap();
        c.as_ptr()
    }

    fn ir_value_to_llvm_value(
        &mut self,
        ir_value: &IRValue,
        builder: *mut LLVMBuilder,
    ) -> LLVMValueRef {
        // unsafe { llvm_sys::core::LLVMConstInt(llvm_sys::core::LLVMInt32Type(), 4, 1) }

        unsafe {
            match ir_value {
                IRValue::INT(i) => LLVMConstInt(LLVMInt32Type(), *i as u64, 1),
                IRValue::FLOAT(_) => todo!(),
                IRValue::REF(r) => {
                    let value_bundle = self
                        .sym_table
                        .get(r.value.to_owned())
                        .expect("expected value");
                    // let c_str = CString::new(format!("{}_local", self.anon_local_counter)).unwrap();
                    let c_string =
                        self.string_to_c_str(&format!("{}_local", self.anon_local_counter));
                    self.anon_local_counter += 1;
                    if value_bundle.is_ref {
                        // todo this is not always right. we don't want to load if its not an alloca instruction.
                        // say its just an add instruction in the symbol table, we want to return that.
                        // we need to store some information along with it.
                        return LLVMBuildLoad2(
                            builder,
                            LLVMInt32Type(),
                            value_bundle.llvm_value.clone(),
                            c_string,
                        );
                    }

                    value_bundle.llvm_value
                }
                IRValue::STRING(_) => todo!(),
                IRValue::INTRINSIC(_) => todo!(),
            }
        }
    }

    fn generate_program(
        &mut self,
        instructions: &Box<Vec<Instruction>>,
        context: *mut LLVMContext,
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        for instruction in instructions.iter() {
            self.generate_instruction(
                instruction,
                context,
                module,
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
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        for instruction in instructions.iter() {
            self.generate_instruction(
                instruction,
                context,
                module,
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
        module: *mut LLVMModule,
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
            if llvm_ptr_val.is_ref {
                let load_value = LLVMBuildLoad2(
                    builder,
                    LLVMPointerType(LLVMInt8Type(), 0),
                    llvm_ptr_val.llvm_value.clone(),
                    label_var_ptr,
                );
                self.sym_table.add(
                    label.to_string(),
                    LLVMValueBundle {
                        llvm_value: load_value,
                        is_ref: false, // todo this may be wrong! check the llvm_ptr_val
                    },
                );
            }
            panic!("must be ref to do a load!");
        }
        None
    }

    fn generate_cond_br(
        &mut self,
        condition: &IRValue,
        body: &Box<Instruction>,
        else_body: &Option<Box<Instruction>>,
        context: *mut LLVMContext,
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            let int_cast_c_str = CString::new(format!("{}", self.anon_local_counter)).unwrap();
            self.anon_local_counter += 1;
            let int_cast_c_str_ptr = int_cast_c_str.as_ptr();
            let mut cond_value: *mut LLVMValue;

            match condition {
                IRValue::INT(i) => {
                    cond_value = LLVMBuildIntCast(
                        builder,
                        LLVMConstInt(LLVMInt32Type(), *i as u64, 1),
                        LLVMInt1Type(),
                        int_cast_c_str_ptr,
                    );
                }
                IRValue::REF(r) => {
                    let int_val = self.sym_table.get(r.value.to_string()).unwrap();
                    let load_c_str: CString =
                        CString::new(format!("{}", self.anon_local_counter)).unwrap();
                    self.anon_local_counter += 1;
                    let load_c_str_ptr = load_c_str.as_ptr();
                    if int_val.is_ref {
                        let load_instr = LLVMBuildLoad2(
                            builder,
                            LLVMInt32Type(),
                            int_val.llvm_value.clone(),
                            load_c_str_ptr,
                        );
                        cond_value = LLVMBuildICmp(
                            builder,
                            llvm_sys::LLVMIntPredicate::LLVMIntNE,
                            load_instr,
                            LLVMConstInt(LLVMInt32Type(), 0 as u64, 1),
                            int_cast_c_str_ptr,
                        );
                    } else {
                        cond_value = LLVMBuildICmp(
                            builder,
                            llvm_sys::LLVMIntPredicate::LLVMIntNE,
                            int_val.llvm_value,
                            LLVMConstInt(LLVMInt32Type(), 0 as u64, 1),
                            int_cast_c_str_ptr,
                        );
                    }
                }
                _ => todo!(),
            }

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

            LLVMBuildCondBr(builder, cond_value, then_block, else_block);

            LLVMPositionBuilderAtEnd(builder, then_block);
            self.generate_instruction(
                &body,
                context,
                module,
                builder,
                current_block,
                current_function,
            );
            LLVMBuildBr(builder, done_block);

            // print positioning here!
            LLVMPositionBuilderAtEnd(builder, else_block);
            if let Some(else_body_instruction) = else_body {
                self.generate_instruction(
                    else_body_instruction,
                    context,
                    module,
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

    fn generate_store(
        &mut self,
        storee: &Ref,
        value: &IRValue,
        context: *mut LLVMContext,
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            let llvm_value = self.ir_value_to_llvm_value(value, builder);
            let storee_ptr = self.sym_table.get(storee.value.to_string()).unwrap();
            // todo make a generic way to get an llvm value from an IRValue
            LLVMBuildStore(builder, llvm_value, storee_ptr.llvm_value.clone());
        }
        None
    }

    fn generate_func(
        &mut self,
        name: &String,
        instruction: &Instruction,
        context: *mut LLVMContext,
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            let void = LLVMVoidType();
            let function_type = llvm_sys::core::LLVMFunctionType(void, std::ptr::null_mut(), 0, 0);

            let fn_name = CString::new(name.to_string()).unwrap();
            let ptr_fn_name = fn_name.as_ptr();

            let function = llvm_sys::core::LLVMAddFunction(module, ptr_fn_name, function_type);
            let bb = llvm_sys::core::LLVMAppendBasicBlockInContext(
                context,
                function,
                b"entry\0".as_ptr() as *const _,
            );

            self.sym_table.add(
                name.to_string(),
                LLVMValueBundle {
                    llvm_value: function,
                    is_ref: false,
                },
            );

            LLVMPositionBuilderAtEnd(builder, bb);

            self.generate_instruction(instruction, context, module, builder, bb, function);

            LLVMBuildRetVoid(builder);

            LLVMPositionBuilderAtEnd(builder, current_block);
        }
        None
    }

    fn generate_call(
        &mut self,
        label: &String,
        callee: &String,
        arg: &IRValue,
        context: *mut LLVMContext,
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            let func_value = self
                .sym_table
                .get(callee.to_owned())
                .expect("expected printf")
                .llvm_value;

            let function_type = llvm_sys::core::LLVMFunctionType(
                LLVMInt32Type(),
                &mut LLVMPointerType(LLVMInt8Type(), 0),
                1,
                0,
            );

            let printf_var = CString::new("call_result").expect("i am a c string");
            let printf_var_ptr = printf_var.as_ptr();

            match arg {
                IRValue::INT(i) => {
                    // todo
                }
                IRValue::REF(r) => {
                    let mut arg0 = self
                        .sym_table
                        .get(r.value.to_string())
                        .expect("expected value")
                        .llvm_value
                        .clone();

                    LLVMBuildCall2(
                        builder,
                        function_type,
                        func_value,
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
                        func_value,
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
        module: *mut LLVMModule,
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
                        let initializer_value = self
                            .sym_table
                            .get(r.value.to_string())
                            .unwrap()
                            .llvm_value
                            .clone();
                        let store_instruction = llvm_sys::core::LLVMBuildStore(
                            builder,
                            initializer_value,
                            alloca_instruction,
                        );
                        self.sym_table.add(
                            label.to_string(),
                            LLVMValueBundle {
                                llvm_value: alloca_instruction,
                                is_ref: true,
                            },
                        );
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
                        self.sym_table.add(
                            label.to_string(),
                            LLVMValueBundle {
                                llvm_value: alloca_instruction,
                                is_ref: true,
                            },
                        );
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

                        self.sym_table.add(
                            label.to_string(),
                            LLVMValueBundle {
                                llvm_value: load,
                                is_ref: false,
                            },
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
        first: &IRValue,
        second: &IRValue,
        context: *mut LLVMContext,
        module: *mut LLVMModule,
        builder: *mut LLVMBuilder,
        current_block: *mut LLVMBasicBlock,
        current_function: *mut LLVMValue,
    ) -> Option<*mut LLVMValue> {
        unsafe {
            let left = self.ir_value_to_llvm_value(first, builder);
            let right = self.ir_value_to_llvm_value(second, builder);

            let add_instr = llvm_sys::core::LLVMBuildAdd(
                builder,
                left,
                right,
                location.to_owned().as_bytes().as_ptr() as *const i8,
            );

            self.sym_table.add(
                location.to_string(),
                LLVMValueBundle {
                    llvm_value: add_instr,
                    is_ref: false,
                },
            );

            None
        }
    }
}
