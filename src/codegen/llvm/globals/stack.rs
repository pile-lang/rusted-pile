use inkwell::values::{BasicValueEnum, GlobalValue, IntValue, PointerValue};

use crate::codegen::llvm::compiler::Compiler;

#[derive(Clone, Copy)]
pub struct Stack<'ctx> {
  pub stack: GlobalValue<'ctx>,
  pub top: GlobalValue<'ctx>,
  pub size: u32,
}

pub const STACK_NAME: &str = "gStack";
pub const STACK_TOP_PTR_NAME: &str = "gTopPtr";

impl<'ctx> Stack<'ctx> {
  pub fn new(size: u32, compiler: &Compiler<'ctx>) -> Self {
    let module = compiler.module();

    let array_type = compiler.array_type(size);

    let g_array = module.add_global(array_type, None, STACK_NAME);
    g_array.set_linkage(inkwell::module::Linkage::Internal);
    g_array.set_initializer(&array_type.const_zero());

    let g_top = module.add_global(compiler.ptr_i32_type(), None, STACK_TOP_PTR_NAME);
    g_top.set_linkage(inkwell::module::Linkage::Internal);
    g_top.set_initializer(&g_array.as_pointer_value());

    Self {
      stack: g_array,
      top: g_top,
      size,
    }
  }

  pub fn stack_top_ptr(&self) -> PointerValue {
    self.top.as_pointer_value()
  }

  pub fn stack_ptr(&self) -> PointerValue {
    self.stack.as_pointer_value()
  }

  pub fn is_full(&'ctx self, compiler: &Compiler<'ctx>) -> IntValue<'ctx> {
    let builder = compiler.builder();

    let top = self.load_top_ptr(compiler);

    let end_of_stack_ptr = unsafe {
      builder.build_in_bounds_gep(
        compiler.i32_type(),
        self.stack_ptr(),
        &[compiler.const_u32(self.size)],
        "nextTopPtr",
      )
    };

    builder.build_int_compare(
      inkwell::IntPredicate::EQ,
      top,
      end_of_stack_ptr,
      "isStackFull",
    )
  }

  pub fn is_empty(&'ctx self, compiler: &Compiler<'ctx>) -> IntValue<'ctx> {
    let builder = compiler.builder();

    let top = self.load_top_ptr(compiler);
    let start_of_stack_ptr = self.stack_ptr();

    builder.build_int_compare(
      inkwell::IntPredicate::EQ,
      top,
      start_of_stack_ptr,
      "isStackEmpty",
    )
  }

  // Store in the stack
  pub fn store(&'ctx self, compiler: &Compiler<'ctx>, value: IntValue) {
    let builder = compiler.builder();

    // Load gTop into the gTop variable
    let ptr = self.load_top_ptr(compiler);

    // Store the element at the position where the top is pointing to
    builder.build_store(ptr, value);

    // Increment the top
    let next_ptr = unsafe {
      builder.build_in_bounds_gep(
        compiler.i32_type(),
        ptr,
        &[compiler.i32_type().const_int(1, false)],
        "nextTopPtr",
      )
    };

    builder.build_store(self.stack_top_ptr(), next_ptr);
  }

  pub fn load_top(&'ctx self, compiler: &Compiler<'ctx>) -> BasicValueEnum<'ctx> {
    compiler.builder().build_load(
      compiler.ptr_i32_type(),
      self.stack_top_ptr(),
      STACK_TOP_PTR_NAME,
    )
  }

  pub fn load_top_ptr(&'ctx self, compiler: &Compiler<'ctx>) -> PointerValue<'ctx> {
    self.load_top(compiler).into_pointer_value()
  }

  pub fn load_size(&'ctx self, compiler: &Compiler<'ctx>) -> BasicValueEnum<'ctx> {
    compiler.builder().build_load(
      compiler.ptr_i32_type(),
      self.stack_top_ptr(),
      STACK_TOP_PTR_NAME,
    )
  }

  pub fn load_size_ptr(&'ctx self, compiler: &Compiler<'ctx>) -> PointerValue<'ctx> {
    self.load_size(compiler).into_pointer_value()
  }
}
