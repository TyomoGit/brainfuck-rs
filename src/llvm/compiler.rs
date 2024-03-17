use std::path::Path;

use anyhow::{anyhow, Ok, Result};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{CodeModel, RelocMode, Target, TargetMachine};
use inkwell::types::{FunctionType, IntType, PointerType};
use inkwell::values::{AnyValue, FunctionValue, GlobalValue, IntValue, PointerValue};
use inkwell::{targets, AddressSpace, IntPredicate, OptimizationLevel};

use crate::ast::Instruction;

#[derive(Debug)]
pub struct Compiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    machine: targets::TargetMachine,

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
#[allow(dead_code)]
struct Values<'ctx> {
    getchar_fn: FunctionValue<'ctx>,
    putchar_fn: FunctionValue<'ctx>,
    printf_fn: FunctionValue<'ctx>,
    main_fn: FunctionValue<'ctx>,

    array: GlobalValue<'ctx>,
    msg_ptr: GlobalValue<'ctx>,
    pointer_ptr: PointerValue<'ctx>,
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
            putchar_fn_type: context
                .i32_type()
                .fn_type(&[context.i8_type().into()], false),
            printf_fn_type: context.i32_type().fn_type(
                &[context.i8_type().ptr_type(AddressSpace::default()).into()],
                true,
            ),
            main_fn_type: context.i32_type().fn_type(&[], false),
        };

        let main_fn = module.add_function("main", types.main_fn_type, None);
        let entry_block = context.append_basic_block(main_fn, "entry_block");
        builder.position_at_end(entry_block);

        let array_value = types
            .i8_type
            .const_array(&[types.i8_type.const_zero(); 30000]);
        let array = module.add_global(array_value.get_type(), None, "array");
        array.set_initializer(&array_value);

        let msg_ptr = builder.build_global_string_ptr("[%p]", "message").unwrap();

        let pointer_ptr = builder
            .build_alloca(types.i8_ptr_type, "pointer_ptr")
            .unwrap();

        builder
            .build_store(pointer_ptr, array.as_pointer_value())
            .unwrap();

        let values = Values {
            getchar_fn: module.add_function("getchar", types.getchar_fn_type, None),
            putchar_fn: module.add_function("putchar", types.putchar_fn_type, None),
            printf_fn: module.add_function("printf", types.printf_fn_type, None),
            main_fn,
            array,
            msg_ptr,
            pointer_ptr,
        };

        Self {
            context,
            module,
            builder,
            machine,
            types,
            values,
        }
    }

    pub fn compile(&mut self, code: Vec<Instruction>) {
        for instruction in code {
            self.compile_instruction(instruction);
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
                let pointer = self.load_ptr_ptr(self.values.pointer_ptr);

                let new_pointer = unsafe {
                    self.builder
                        .build_in_bounds_gep(
                            self.types.i8_ptr_type,
                            pointer,
                            &[self.types.i32_type.const_int(1, false)],
                            "incremented_pointer",
                        )
                        .unwrap()
                };

                self.builder
                    .build_store(self.values.pointer_ptr, new_pointer)
                    .unwrap();
            }
            Instruction::DecrementPointer => {
                let one = self.types.i32_type.const_int(1, false);
                let diff = self.builder.build_int_neg(one, "minus_one").unwrap();

                let pointer = self.load_ptr_ptr(self.values.pointer_ptr);

                let new_pointer = unsafe {
                    self.builder
                        .build_in_bounds_gep(
                            self.types.i8_ptr_type,
                            pointer,
                            &[diff],
                            "decremented_pointer",
                        )
                        .unwrap()
                };

                self.builder
                    .build_store(self.values.pointer_ptr, new_pointer)
                    .unwrap();
            }
            Instruction::InclementValue => {
                let pointer = self.load_ptr_ptr(self.values.pointer_ptr);

                let value = self.load_value(pointer);

                let new_value = self
                    .builder
                    .build_int_add(
                        value,
                        self.types.i8_type.const_int(1, false),
                        "incremented_value",
                    )
                    .unwrap();
                self.builder.build_store(pointer, new_value).unwrap();
            }
            Instruction::DecrementValue => {
                let pointer = self.load_ptr_ptr(self.values.pointer_ptr);

                let value = self.load_value(pointer);

                let new_value = self
                    .builder
                    .build_int_sub(
                        value,
                        self.types.i8_type.const_int(1, false),
                        "decremented_value",
                    )
                    .unwrap();
                self.builder.build_store(pointer, new_value).unwrap();
            }
            Instruction::Output => {
                let pointer = self.load_ptr_ptr(self.values.pointer_ptr);

                let value = self.load_value(pointer);
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
                let pointer = self.load_ptr_ptr(self.values.pointer_ptr);
                self.builder.build_store(pointer, value).unwrap();
            }
            Instruction::Loop(instructions) => {
                let loop_start = self
                    .context
                    .append_basic_block(self.values.main_fn, "loop_start");
                let loop_body = self
                    .context
                    .append_basic_block(self.values.main_fn, "loop_body");

                self.builder.build_unconditional_branch(loop_start).unwrap();

                self.builder.position_at_end(loop_body);

                for instruction in instructions {
                    self.compile_instruction(instruction);
                }

                let before_end = self.values.main_fn.get_last_basic_block().unwrap();
                let loop_end = self
                    .context
                    .append_basic_block(self.values.main_fn, "loop_end");

                self.builder.position_at_end(loop_start);
                let pointer = self
                    .builder
                    .build_load(self.types.i8_ptr_type, self.values.pointer_ptr, "pointer")
                    .unwrap()
                    .into_pointer_value();

                let value = self.load_value(pointer);

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

    /// i8 ptr ptr -> i8 ptr
    fn load_ptr_ptr(&self, ptr_ptr: PointerValue<'ctx>) -> PointerValue<'ctx> {
        self.builder
            .build_load(self.types.i8_ptr_type, ptr_ptr, "pointer")
            .unwrap()
            .into_pointer_value()
    }

    fn load_value(&self, ptr: PointerValue<'ctx>) -> IntValue<'ctx> {
        self.builder
            .build_load(self.types.i8_type, ptr, "value")
            .unwrap()
            .into_int_value()
    }

    pub fn write_to_file(&self, file: &Path) -> Result<()> {
        self.module
            .verify()
            .map_err(|e| anyhow!("module verification failed: {}", e))?;
        self.machine
            .write_to_file(&self.module, targets::FileType::Object, file)
            .map_err(|e| anyhow!("failed to write object file: {}", e))?;

        // let mut ir = std::fs::File::create("a.ll").unwrap();
        // ir.write_all(self.module.to_string().as_bytes())?;

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
