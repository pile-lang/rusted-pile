use inkwell::values::{BasicValueEnum, GlobalValue, IntValue, PointerValue};
use singleton_manager::sm;

use crate::codegen::llvm::manager::LLVMManager;

pub struct Stack<'ctx> {
  pub stack: GlobalValue<'ctx>,
  pub top: GlobalValue<'ctx>,
  pub size: u32,
}

pub const STACK_NAME: &str = "gStack";
pub const STACK_TOP_PTR_NAME: &str = "gTopPtr";

impl<'ctx> Stack<'static> {
  pub fn new(size: u32) -> Self {
    let manager = LLVMManager::get();
    let module = manager.module();

    let array_type = manager.array_type(size);

    let g_array = module.add_global(array_type, None, STACK_NAME);
    g_array.set_linkage(inkwell::module::Linkage::Internal);
    g_array.set_initializer(&array_type.const_zero());

    let g_top = module.add_global(manager.ptr_i32_type(), None, STACK_TOP_PTR_NAME);
    g_top.set_linkage(inkwell::module::Linkage::Internal);
    g_top.set_initializer(&g_array.as_pointer_value());

    Self {
      stack: g_array,
      top: g_top,
      size,
    }
  }

  fn load_top(&self) -> BasicValueEnum {
    let manager = LLVMManager::get();

    manager.builder().build_load(
      manager.ptr_i32_type(),
      self.top.as_pointer_value(),
      STACK_TOP_PTR_NAME,
    )
  }

  fn stack_top_ptr(&self) -> PointerValue {
    self.top.as_pointer_value()
  }

  fn stack_ptr(&self) -> PointerValue {
    self.stack.as_pointer_value()
  }

  pub fn is_full(&self) -> IntValue {
    let manager = LLVMManager::get();
    let builder = manager.builder();

    let end_of_stack_ptr = unsafe {
      builder.build_in_bounds_gep(
        manager.i32_type(),
        self.stack_ptr(),
        &[manager.const_int(self.size as u64)],
        "nextTopPtr",
      )
    };

    builder.build_int_compare(
      inkwell::IntPredicate::EQ,
      self.stack_top_ptr(),
      end_of_stack_ptr,
      "isFull",
    )
  }

  // Store in the stack
  pub fn store(&self, value: IntValue) {
    let manager = LLVMManager::get();
    let builder = manager.builder();

    // Load gTop into the gTop variable
    let ptr = self.stack_top_ptr();

    // Store the element at the position where the top is pointing to
    builder.build_store(ptr, value);

    // Increment the top
    let next_ptr = unsafe {
      builder.build_in_bounds_gep(
        manager.i32_type(),
        ptr,
        &[manager.i32_type().const_int(1, false)],
        "nextTopPtr",
      )
    };

    builder.build_store(ptr, next_ptr);
  }

  pub fn get() -> &'static mut Self {
    sm()
      .get::<Self>("Stack")
      .expect("Failed to get Stack. Probably not created yet.")
  }

  pub fn create(size: u32) {
    sm()
      .set("Stack", Self::new(size))
      .expect("Failed to create Stack");
  }
}
