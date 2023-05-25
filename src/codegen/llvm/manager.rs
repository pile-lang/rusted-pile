use inkwell::{builder::Builder, context::Context, module::Module};
use singleton_manager::sm;
use std::sync::{Arc, Mutex};

pub struct LLVMManager<'ctx> {
  pub guard: Mutex<()>,
  pub context: Arc<Mutex<Context>>,
  pub module: Module<'ctx>,
  pub builder: Builder<'ctx>,
}

impl<'ctx> LLVMManager<'ctx> {
  pub fn new(context: Arc<Mutex<Context>>) -> Self {
    let context_clone = Arc::clone(&context);
    let module = context_clone.lock().unwrap().create_module("main");

    let context_clone = Arc::clone(&context);
    let builder = context_clone.lock().unwrap().create_builder();

    Self {
      guard: Mutex::new(()),
      context,
      module,
      builder,
    }
  }

  pub fn context(&self) -> &Context {
    &self.context.lock().unwrap()
  }

  pub fn module(&self) -> &Module<'ctx> {
    &self.module
  }

  pub fn builder(&self) -> &Builder<'ctx> {
    &self.builder
  }

  pub fn fetch_all(&self) -> (&'ctx Context, &Module<'ctx>, &Builder<'ctx>) {
    (&self.context(), &self.module, &self.builder)
  }

  // Helper functions for types and stuff
  pub fn i32_type(&self) -> inkwell::types::IntType<'ctx> {
    self.context().i32_type()
  }

  pub fn ptr_i32_type(&self) -> inkwell::types::PointerType<'ctx> {
    self.i32_type().ptr_type(inkwell::AddressSpace::default())
  }

  pub fn void_type(&self) -> inkwell::types::VoidType<'ctx> {
    self.context().void_type()
  }

  pub fn array_type(&self, size: u32) -> inkwell::types::ArrayType<'ctx> {
    self.i32_type().array_type(size)
  }

  pub fn const_int(&self, value: u64) -> inkwell::values::IntValue<'ctx> {
    self.i32_type().const_int(value, false)
  }

  pub fn get() -> &'static mut Self {
    sm()
      .get::<Self>("LLVMManager")
      .expect("Failed to get LLVMManager. Probably not created yet.")
  }

  pub fn create(context: Arc<Mutex<Context>>) {
    sm()
      .set("LLVMManager", LLVMManager::new(context))
      .expect("Failed to create LLVMManager");
  }
}
