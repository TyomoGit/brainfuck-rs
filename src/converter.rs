use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Result};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::types::{BasicType, PointerType};
use inkwell::values::AnyValue;
use inkwell::{targets, AddressSpace, IntPredicate, OptimizationLevel};

use crate::op::Op;

#[derive(Debug)]
pub struct Converter<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    machine: targets::TargetMachine,

    code: Vec<Op>,
    loop_start_end: Vec<(BasicBlock<'ctx>, BasicBlock<'ctx>)>,
}

impl<'ctx> Converter<'ctx> {
    pub fn new(context: &'ctx Context, machine: targets::TargetMachine, code: Vec<Op>) -> Self {
        let module = context.create_module("main");
        let builder = context.create_builder();

        Self {
            context,
            module,
            builder,
            machine,
            code,
            loop_start_end: Vec::new(),
        }
    }

    pub fn convert(&mut self) {
        let i8_type = self.context.i8_type();
        let i8_ptr_type = i8_type.ptr_type(AddressSpace::default());
        let i32_type = self.context.i32_type();
        let i64_type = self.context.i64_type();

        let getchar_fn_type = i8_type.fn_type(&[], false);
        let getchar_fn = self.module.add_function("getchar", getchar_fn_type, None);

        let putchar_fn_type = i32_type.fn_type(&[i8_type.into()], false);
        let putchar_fn = self.module.add_function("putchar", putchar_fn_type, None);

        let array_ = i8_type.const_array(&[i8_type.const_zero(); 30000]);
        // let array = self.builder.build_array_alloca(i8_type, i32_type.const_int(30000, false), "array").unwrap();
        // let zero = i8_type.const_zero();
        // self.builder.build_store(array, zero).unwrap();
        let array = self.module.add_global(array_.get_type(), None, "array");
        array.set_initializer(&array_);

        let main_fn_type = i32_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_fn_type, None);

        let entry_block = self.context.append_basic_block(main_fn, "entry_block");
        self.builder.position_at_end(entry_block);

        let pointer = self.builder.build_alloca(i8_ptr_type, "pointer").unwrap();
        self.builder.build_store(pointer, array.as_pointer_value()).unwrap();

        for op in self.code.iter() {
            match op {
                Op::InclementPointer => {
                    let pointer = self
                        .builder
                        .build_load(i8_ptr_type, pointer, "pointer")
                        .unwrap()
                        .into_pointer_value();
                    let new_pointer = unsafe {
                        self.builder
                            .build_in_bounds_gep(
                                i8_ptr_type,
                                pointer,
                                &[i32_type.const_int(1, false)],
                                "new_pointer",
                            )
                            .unwrap()
                    };

                    self.builder.build_store(pointer, new_pointer).unwrap();
                }
                Op::DecrementPointer => {
                    let pointer = self
                        .builder
                        .build_load(i8_ptr_type, pointer, "pointer")
                        .unwrap()
                        .into_pointer_value();

                    let one = i64_type.const_int(1, false);
                    let diff = self.builder.build_int_neg(one, "diff").unwrap();

                    let new_pointer = unsafe {
                        self.builder
                            .build_in_bounds_gep(i8_type, pointer, &[diff], "new_pointer")
                            .unwrap()
                    };

                    self.builder.build_store(pointer, new_pointer).unwrap();
                }
                Op::InclementValue => {
                    let value = self
                        .builder
                        .build_load(i8_type, pointer, "pointer")
                        .unwrap()
                        .into_int_value();

                    let new_value = self
                        .builder
                        .build_int_add(value, i8_type.const_int(1, false), "new_value")
                        .unwrap();
                    self.builder.build_store(pointer, new_value).unwrap();
                }
                Op::DecrementValue => {
                    let pointer = self
                        .builder
                        .build_load(i8_ptr_type, pointer, "pointer")
                        .unwrap()
                        .into_pointer_value();
                    let value = self
                        .builder
                        .build_load(i8_type, pointer, "value")
                        .unwrap()
                        .into_int_value();

                    let one = i8_type.const_int(1, false);
                    let diff = self.builder.build_int_neg(one, "diff").unwrap();

                    let new_value = self
                        .builder
                        .build_int_sub(value, diff, "new_value")
                        .unwrap();
                    self.builder.build_store(pointer, new_value).unwrap();
                }
                Op::Output => {
                    let value = self
                        .builder
                        .build_load(i8_type, pointer, "value")
                        .unwrap()
                        .into_int_value();
                    self.builder
                        .build_call(putchar_fn, &[value.into()], "call_putchar")
                        .unwrap();
                }
                Op::Input => {
                    let value = self
                        .builder
                        .build_call(getchar_fn, &[], "call_getchar")
                        .unwrap()
                        .as_any_value_enum()
                        .into_int_value();
                    self.builder.build_store(pointer, value).unwrap();
                }
                Op::LoopStart { if_zero: _ } => {
                    let block_before_loop = main_fn.get_last_basic_block().unwrap();
                    let loop_start = self.context.append_basic_block(main_fn, "loop_start");
                    let loop_body = self.context.append_basic_block(main_fn, "loop_body");
                    let loop_end = self.context.append_basic_block(main_fn, "loop_end");
                    self.loop_start_end.push((loop_start, loop_end));

                    self.builder.position_at_end(block_before_loop);
                    self.builder.build_unconditional_branch(loop_start).unwrap();

                    self.builder.position_at_end(loop_start);

                    let value = self
                        .builder
                        .build_load(i8_type, pointer, "value")
                        .unwrap()
                        .into_int_value();

                    let condition = self
                        .builder
                        .build_int_compare(
                            IntPredicate::NE,
                            value,
                            i8_type.const_zero(),
                            "condition",
                        )
                        .unwrap();

                    self.builder
                        .build_conditional_branch(condition, loop_body, loop_end)
                        .unwrap();

                    self.builder.position_at_end(loop_body);
                }
                Op::LoopEnd { if_non_zero: _ } => {
                    let start_and_end = self.loop_start_end.pop().unwrap();

                    self.builder
                        .build_unconditional_branch(start_and_end.0)
                        .unwrap();

                    self.builder.position_at_end(start_and_end.1);
                }
            }
        }

        self.builder
            .build_return(Some(&i32_type.const_int(0, false)))
            .unwrap();
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
}

pub fn host_machine() -> Result<targets::TargetMachine> {
    Target::initialize_native(&targets::InitializationConfig::default())
        .map_err(|e| anyhow!("failed to initialize native target: {}", e))?;

    let triple = TargetMachine::get_default_triple();
    let target =
        Target::from_triple(&triple).map_err(|e| anyhow!("failed to create target: {}", e))?;

    let cpu = TargetMachine::get_host_cpu_name();
    let features = TargetMachine::get_host_cpu_features();

    let opt_level = OptimizationLevel::None;
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
