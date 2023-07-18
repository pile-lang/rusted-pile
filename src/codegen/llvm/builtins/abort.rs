use inkwell::{values::FunctionValue, AddressSpace};

use crate::codegen::llvm::{
  compiler::Compiler,
  externs::{exit::ExitExtern, printf::PrintfExtern},
};

pub const ABORT_FUNCTION_NAME: &str = "abort";

pub struct AbortBuiltin;

impl AbortBuiltin {
  pub fn declare(compiler: &Compiler<'_>) {
    let builder = compiler.builder();
    let module = compiler.module();

    let i32_type = compiler.i32_type();
    let abort_type = compiler.void_type().fn_type(
      &[
        i32_type.ptr_type(AddressSpace::default()).into(), // Message
        i32_type.into(),                                   // Exit code
      ],
      false,
    );
    let abort_func = module.add_function(ABORT_FUNCTION_NAME, abort_type, None);

    let basic_block = compiler.append_basic_block(abort_func, "entry");
    {
      builder.position_at_end(basic_block);

      let params = abort_func.get_params();
      let message_ptr = params.get(0).unwrap().into_pointer_value();
      let exit_code = params.get(1).unwrap().into_int_value();

      PrintfExtern::call(compiler, &[message_ptr.into()]);
      ExitExtern::call(compiler, &[exit_code.into()]);

      builder.build_unreachable();
    }
  }

  pub fn call_from_values(
    compiler: &Compiler<'_>,
    message: &str,
    code: i32,
    string_name: Option<String>,
    call_name: Option<String>,
  ) {
    let string_name = string_name.unwrap_or_else(|| "message".to_string());
    let call_name = call_name.unwrap_or_else(|| "abort_call".to_string());

    let builder = compiler.builder();

    let message_ptr = builder.build_global_string_ptr(message, &string_name);

    builder.build_call(
      Self::get(compiler),
      &[
        message_ptr.as_pointer_value().into(),
        compiler.const_i32(code).into(),
      ],
      &call_name,
    );
  }

  pub fn get<'ctx>(compiler: &Compiler<'ctx>) -> FunctionValue<'ctx> {
    let module = compiler.module();

    module.get_function(ABORT_FUNCTION_NAME).unwrap_or_else(|| {
      Self::declare(compiler);
      module
        .get_function(ABORT_FUNCTION_NAME)
        .expect("abort function not found")
    })
  }
}
