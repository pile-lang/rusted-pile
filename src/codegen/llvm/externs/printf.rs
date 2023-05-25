use crate::codegen::llvm::manager::LLVMManager;

pub struct PrintfExtern;

impl PrintfExtern {
  pub fn declare() {
    let manager = LLVMManager::get();
    let module = manager.module();

    let printf_type = manager
      .i32_type()
      .fn_type(&[manager.ptr_i32_type().into()], true);
    module.add_function("printf", printf_type, None);
  }

  pub fn get() -> inkwell::values::FunctionValue<'static> {
    let manager = LLVMManager::get();
    let module = manager.module();

    module.get_function("printf").unwrap_or_else(|| {
      PrintfExtern::declare();
      module
        .get_function("printf")
        .expect("printf function not found")
    })
  }

  pub fn call_from_str(message: &str) {}

  pub fn call(args: &[inkwell::values::BasicMetadataValueEnum]) {
    let manager = LLVMManager::get();

    manager
      .builder()
      .build_call(PrintfExtern::get(), args, "printf_call");
  }
}
