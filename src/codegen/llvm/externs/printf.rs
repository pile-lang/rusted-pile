use inkwell::values::FunctionValue;

use crate::codegen::llvm::compiler::Compiler;

pub const PRINTF_FUNCTION_NAME: &str = "printf";

pub struct PrintfExtern;

impl PrintfExtern {
  pub fn declare(compiler: &Compiler<'_>) {
    compiler.module().add_function(
      PRINTF_FUNCTION_NAME,
      compiler
        .i32_type()
        .fn_type(&[compiler.ptr_i32_type().into()], true),
      None,
    );
  }

  pub fn get<'ctx>(compiler: &Compiler<'ctx>) -> FunctionValue<'ctx> {
    let module = compiler.module();

    module
      .get_function(PRINTF_FUNCTION_NAME)
      .unwrap_or_else(|| {
        Self::declare(compiler);
        module
          .get_function(PRINTF_FUNCTION_NAME)
          .expect("abort function not found")
      })
  }

  pub fn call_from_str(compiler: &Compiler<'_>, message: &str) {
    let builder = compiler.builder();

    let message_ptr = builder.build_global_string_ptr(message, "message");
    builder.build_call(
      PrintfExtern::get(compiler),
      &[message_ptr.as_pointer_value().into()],
      "printf_call_from_str",
    );
  }

  pub fn call(compiler: &Compiler<'_>, args: &[inkwell::values::BasicMetadataValueEnum]) {
    compiler
      .builder()
      .build_call(PrintfExtern::get(compiler), args, "printf_call");
  }
}
