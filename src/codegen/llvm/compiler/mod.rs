use std::path::Path;

use inkwell::{builder::Builder, context::Context, module::Module};

// use super::wrapper::context::Context;

pub struct Compiler<'ctx> {
  context: &'ctx Context,
  module: Module<'ctx>,
  builder: Builder<'ctx>,
}

impl<'ctx> Compiler<'ctx> {
  pub fn new(context: &'ctx Context, name: &str) -> Self {
    let module = context.create_module(name);
    let builder = context.create_builder();

    Self {
      context,
      module,
      builder,
    }
  }

  pub fn module(&self) -> &Module<'ctx> {
    &self.module
  }

  pub fn builder(&self) -> &Builder<'ctx> {
    &self.builder
  }

  pub fn append_basic_block(
    &self,
    function: inkwell::values::FunctionValue<'ctx>,
    name: &str,
  ) -> inkwell::basic_block::BasicBlock<'ctx> {
    self.context.append_basic_block(function, name)
  }

  // ====================== Types ======================

  pub fn array_type(&self, size: u32) -> inkwell::types::ArrayType<'ctx> {
    self.context.i32_type().array_type(size)
  }

  pub fn i32_type(&self) -> inkwell::types::IntType<'ctx> {
    self.context.i32_type()
  }

  pub fn ptr_i32_type(&self) -> inkwell::types::PointerType<'ctx> {
    self
      .context
      .i32_type()
      .ptr_type(inkwell::AddressSpace::default())
  }

  pub fn void_type(&self) -> inkwell::types::VoidType<'ctx> {
    self.context.void_type()
  }

  pub fn const_i32(&self, value: i32) -> inkwell::values::IntValue<'ctx> {
    self.i32_type().const_int(value as u64, false)
  }

  pub fn const_u32(&self, value: u32) -> inkwell::values::IntValue<'ctx> {
    self.i32_type().const_int(value as u64, false)
  }

  pub fn fn_type(
    &self,
    param_types: &[inkwell::types::BasicMetadataTypeEnum<'ctx>],
    is_var_args: bool,
  ) -> inkwell::types::FunctionType<'ctx> {
    self.context.void_type().fn_type(param_types, is_var_args)
  }
}

impl<'ctx> Drop for Compiler<'ctx> {
  fn drop(&mut self) {
    let module = &self.module;

    // let binding = module.print_to_string();
    // let output = binding.to_str().unwrap();
    // println!("{}", output);

    module.print_to_file(Path::new("output.ll")).unwrap();

    // Invoke clang -o output output.ll
    std::process::Command::new("clang")
      .arg("-o")
      .arg("output")
      .arg("output.ll")
      .arg("-lc")
      .output()
      .expect("failed to execute process");
  }
}
