use inkwell::values::FunctionValue;

use crate::codegen::llvm::compiler::Compiler;

pub const EXIT_FUNCTION_NAME: &str = "exit";

pub struct ExitExtern;

impl ExitExtern {
  pub fn declare(compiler: &Compiler<'_>) {
    compiler.module().add_function(
      EXIT_FUNCTION_NAME,
      compiler
        .void_type()
        .fn_type(&[compiler.i32_type().into()], false),
      None,
    );
  }

  pub fn get<'ctx>(compiler: &Compiler<'ctx>) -> FunctionValue<'ctx> {
    compiler
      .module()
      .get_function(EXIT_FUNCTION_NAME)
      .unwrap_or_else(|| {
        Self::declare(compiler);
        compiler
          .module()
          .get_function(EXIT_FUNCTION_NAME)
          .expect("exit function not found")
      })
  }

  pub fn call_from_int(compiler: &Compiler<'_>, code: i32) {
    compiler.builder().build_call(
      Self::get(compiler),
      &[compiler.const_i32(code).into()],
      "exit_call_from_int",
    );
  }

  pub fn call<'ctx>(
    compiler: &Compiler<'ctx>,
    args: &[inkwell::values::BasicMetadataValueEnum<'ctx>],
  ) {
    compiler
      .builder()
      .build_call(Self::get(compiler), args, "exit_call");
  }
}
