use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Ok, Result};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, RelocMode, Target, TargetMachine};
use inkwell::types::{FunctionType, IntType, PointerType};
use inkwell::values::{AnyValue, FunctionValue, GlobalValue, PointerValue};
use inkwell::{targets, AddressSpace, IntPredicate, OptimizationLevel};

use crate::ast::Instruction;

#[derive(Debug)]
pub struct Compiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    machine: targets::TargetMachine,

    loop_stack: Vec<(BasicBlock<'ctx>, BasicBlock<'ctx>)>,

    types: Types<'ctx>,
    values: Values<'ctx>,
}

#[derive(Debug)]
struct Types<'ctx> {
    i8_ptr_type: PointerType<'ctx>,
    i8_type: IntType<'ctx>,
    i32_type: IntType<'ctx>,
    getchar_fn_type: FunctionType<'ctx>,
    putchar_fn_type: FunctionType<'ctx>,
    printf_fn_type: FunctionType<'ctx>,
    main_fn_type: FunctionType<'ctx>,
}

#[derive(Debug)]
struct Values<'ctx> {
    getchar_fn: FunctionValue<'ctx>,
    putchar_fn: FunctionValue<'ctx>,
    printf_fn: FunctionValue<'ctx>,
    main_fn: FunctionValue<'ctx>,

    array: GlobalValue<'ctx>,
    string_ptr_msg: GlobalValue<'ctx>,
    pointer: PointerValue<'ctx>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, machine: targets::TargetMachine) -> Self {
        let module = context.create_module("main");
        let builder = context.create_builder();

        let types = Types {
            i8_ptr_type: context.i8_type().ptr_type(AddressSpace::default()),
            i8_type: context.i8_type(),
            i32_type: context.i32_type(),
            getchar_fn_type: context.i8_type().fn_type(&[], false),
            putchar_fn_type: context.i32_type().fn_type(&[context.i8_type().into()], false),
            printf_fn_type: context.i32_type().fn_type(&[context.i8_type().ptr_type(AddressSpace::default()).into()], true),
            main_fn_type: context.i32_type().fn_type(&[], false),
        };

        let main_fn = module.add_function("main", types.main_fn_type, None);
        let entry_block = context.append_basic_block(main_fn, "entry_block");
        builder.position_at_end(entry_block);

        let array_ = types.i8_type.const_array(&[types.i8_type.const_zero(); 30000]);
        let array = module.add_global(array_.get_type(), None, "array");
        array.set_initializer(&array_);

        let string_ptr_msg = builder
            .build_global_string_ptr("[%p]", "message")
            .unwrap();

            let pointer = builder.build_alloca(types.i8_ptr_type, "pointer").unwrap();

            builder
                .build_store(pointer, array.as_pointer_value())
                .unwrap();

        let values = Values {
            getchar_fn: module.add_function("getchar", types.getchar_fn_type, None),
            putchar_fn: module.add_function("putchar", types.putchar_fn_type, None),
            printf_fn: module.add_function("printf", types.printf_fn_type, None),
            main_fn,
            array,
            string_ptr_msg,
            pointer,
        };

        Self {
            context,
            module,
            builder,
            machine,
            loop_stack: Vec::new(),
            types,
            values
        }
    }

    pub fn compile(&mut self, code: Vec<Instruction>) {
        for op in code {
            self.compile_instruction(op);
        }

        self.builder
            .position_at_end(self.values.main_fn.get_last_basic_block().unwrap());
        self.builder
            .build_return(Some(&self.types.i32_type.const_int(0, false)))
            .unwrap();
    }

    fn compile_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::InclementPointer => {
                let pointer_ = self
                    .builder
                    .build_load(self.types.i8_ptr_type, self.values.pointer, "pointer")
                    .unwrap()
                    .into_pointer_value();

                let new_pointer_ = unsafe {
                    self.builder
                        .build_in_bounds_gep(
                            self.types.i8_ptr_type,
                            pointer_,
                            &[self.types.i32_type.const_int(1, false)],
                            "new_pointer",
                        )
                        .unwrap()
                };

                self.builder.build_store(self.values.pointer, new_pointer_).unwrap();
            }
            Instruction::DecrementPointer => {
                let one = self.types.i32_type.const_int(1, false);
                let diff = self.builder.build_int_neg(one, "diff").unwrap();

                let pointer_ = self
                    .builder
                    .build_load(self.types.i8_ptr_type, self.values.pointer, "pointer")
                    .unwrap()
                    .into_pointer_value();

                let new_pointer_ = unsafe {
                    self.builder
                        .build_in_bounds_gep(self.types.i8_ptr_type, pointer_, &[diff], "new_pointer")
                        .unwrap()
                };

                self.builder.build_store(self.values.pointer, new_pointer_).unwrap();
            }
            Instruction::InclementValue => {
                let pointer_ = self
                    .builder
                    .build_load(self.types.i8_ptr_type, self.values.pointer, "pointer")
                    .unwrap()
                    .into_pointer_value();

                let value = self
                    .builder
                    .build_load(self.types.i8_type, pointer_, "value")
                    .unwrap()
                    .into_int_value();

                let new_value = self
                    .builder
                    .build_int_add(value, self.types.i8_type.const_int(1, false), "new_value")
                    .unwrap();
                self.builder.build_store(pointer_, new_value).unwrap();
            }
            Instruction::DecrementValue => {
                let pointer_ = self
                    .builder
                    .build_load(self.types.i8_ptr_type, self.values.pointer, "pointer")
                    .unwrap()
                    .into_pointer_value();
                let value = self
                    .builder
                    .build_load(self.types.i8_type, pointer_, "value")
                    .unwrap()
                    .into_int_value();

                let one = self.types.i8_type.const_int(1, false);
                let diff = self.builder.build_int_neg(one, "diff").unwrap();

                let new_value = self.builder.build_int_sub(value, one, "new_value").unwrap();
                self.builder.build_store(pointer_, new_value).unwrap();
            }
            Instruction::Output => {
                let pointer_ = self
                    .builder
                    .build_load(self.types.i8_ptr_type, self.values.pointer, "value")
                    .unwrap()
                    .into_pointer_value();
                let value = self
                    .builder
                    .build_load(self.types.i8_type, pointer_, "value")
                    .unwrap()
                    .into_int_value();
                self.builder
                    .build_call(self.values.putchar_fn, &[value.into()], "call_putchar")
                    .unwrap();
            }
            Instruction::Input => {
                let value = self
                    .builder
                    .build_call(self.values.getchar_fn, &[], "call_getchar")
                    .unwrap()
                    .as_any_value_enum()
                    .into_int_value();
                let pointer_ = self
                    .builder
                    .build_load(self.types.i8_ptr_type, self.values.pointer, "pointer")
                    .unwrap()
                    .into_pointer_value();
                self.builder.build_store(pointer_, value).unwrap();
            }
            Instruction::Loop(instructions) => {
                let loop_start = self.context.append_basic_block(self.values.main_fn, "loop_start");
                let loop_body = self.context.append_basic_block(self.values.main_fn, "loop_body");
                self.loop_stack.push((loop_start, loop_body));

                // self.builder.position_before(&loop_start.get_first_instruction().unwrap());
                self.builder.build_unconditional_branch(loop_start).unwrap();

                self.builder.position_at_end(loop_body);


                for instruction in instructions {
                    self.compile_instruction(instruction);
                }


                let (loop_start, loop_body) = self.loop_stack.pop().unwrap();
                let before_end = self.values.main_fn.get_last_basic_block().unwrap();
                let loop_end = self.context.append_basic_block(self.values.main_fn, "loop_end");

                self.builder.position_at_end(loop_start);
                let pointer_ = self
                    .builder
                    .build_load(self.types.i8_ptr_type, self.values.pointer, "pointer")
                    .unwrap()
                    .into_pointer_value();

                let value = self
                    .builder
                    .build_load(self.types.i8_type, pointer_, "value")
                    .unwrap()
                    .into_int_value();

                let condition = self
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        value,
                        self.types.i8_type.const_zero(),
                        "condition",
                    )
                    .unwrap();

                self.builder
                    .build_conditional_branch(condition, loop_body, loop_end)
                    .unwrap();

                self.builder.position_at_end(before_end);

                self.builder.build_unconditional_branch(loop_start).unwrap();

                self.builder.position_at_end(loop_end);
            }
        }
    }

    pub fn write_to_file(&self, file: &Path) -> Result<()> {
        self.module
            .verify()
            .map_err(|e| anyhow!("module verification failed: {}", e))?;
        self.machine
            .write_to_file(&self.module, targets::FileType::Object, file)
            .map_err(|e| anyhow!("failed to write object file: {}", e))?;

        let mut ir = std::fs::File::create("a.ll").unwrap();

        ir.write_all(self.module.to_string().as_bytes())?;

        Ok(())
    }

    pub fn run_jit(&self) -> Result<()> {
        Target::initialize_native(&targets::InitializationConfig::default())
            .map_err(|e| anyhow!("failed to initialize native target: {}", e))?;

        let engine = self
            .module
            .create_jit_execution_engine(OptimizationLevel::Aggressive)
            .map_err(|e| anyhow!("failed to create JIT engine: {}", e))?;

        unsafe {
            engine
                .get_function::<unsafe extern "C" fn() -> i32>("main")
                .unwrap()
                .call();
        }

        Ok(())
    }
}

pub fn host_machine() -> Result<targets::TargetMachine> {
    Target::initialize_native(&targets::InitializationConfig::default())
        .map_err(|e| anyhow!("failed to initialize native target: {}", e))?;

    let triple = TargetMachine::get_default_triple();
    let target =
        Target::from_triple(&triple).map_err(|e| anyhow!("failed to create target: {}", e))?;

    let cpu = TargetMachine::get_host_cpu_name();
    let features = TargetMachine::get_host_cpu_features();

    let opt_level = OptimizationLevel::Aggressive;
    let reloc_mode = RelocMode::Default;
    let code_model = CodeModel::Default;

    target
        .create_target_machine(
            &triple,
            cpu.to_str()?,
            features.to_str()?,
            opt_level,
            reloc_mode,
            code_model,
        )
        .ok_or(anyhow!("failed to create target machine"))
}
