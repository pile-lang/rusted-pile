use crate::codegen::llvm::manager::LLVMManager;

pub struct ExitExtern;

pub const EXTERN_EXIT: &str = "exit";

impl ExitExtern {
  fn declare() {
    let manager = LLVMManager::get();
    let module = manager.module();

    let exit_type = manager
      .void_type()
      .fn_type(&[manager.i32_type().into()], false);
    module.add_function(EXTERN_EXIT, exit_type, None);
  }

  pub fn get() -> inkwell::values::FunctionValue<'static> {
    let manager = LLVMManager::get();
    let module = manager.module();

    module.get_function(EXTERN_EXIT).unwrap_or_else(|| {
      ExitExtern::declare();
      module
        .get_function(EXTERN_EXIT)
        .expect("printf function not found")
    })
  }

  pub fn call_from_int(code: i32) {
    let manager = LLVMManager::get();

    manager.builder().build_call(
      ExitExtern::get(),
      &[manager.const_int(code as u64).into()],
      "exit_call",
    );
  }

  pub fn call(args: &[inkwell::values::BasicMetadataValueEnum]) {
    let manager = LLVMManager::get();

    manager
      .builder()
      .build_call(ExitExtern::get(), args, "exit_call");
  }
}
