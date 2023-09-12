use crate::codegen::llvm::{compiler::Compiler, globals::stack::Stack};

use super::abort::AbortBuiltin;

pub struct PushBuiltin;

// TODO: Copy the arch-llvm push, it works fine
impl PushBuiltin {
  pub fn declare<'ctx>(compiler: &Compiler<'ctx>, stack: &'ctx Stack<'ctx>) {
    let builder = compiler.builder();
    let module = compiler.module();

    let i32_type = compiler.i32_type();
    let insert_type = compiler.void_type().fn_type(&[i32_type.into()], false);
    let insert_func = module.add_function("push", insert_type, None);
    let entry = compiler.append_basic_block(insert_func, "entry");

    {
      builder.position_at_end(entry);

      // 1. Check if the stack is full
      let stack_full_block = compiler.append_basic_block(insert_func, "stack_full");
      let stack_not_full_block = compiler.append_basic_block(insert_func, "stack_not_full");
      builder.build_conditional_branch(
        stack.is_full(compiler),
        stack_full_block,
        stack_not_full_block,
      );

      // 2. If the stack is full
      {
        builder.position_at_end(stack_full_block);

        AbortBuiltin::call_from_values(
          compiler,
          "[ABORT @ push]: stack is already full\n",
          1,
          Some("error_message_stack_full".to_string()),
          Some("error_stack_full".to_string()),
        );

        builder.build_unreachable();
      }

      // 3. If not then store the value
      {
        // Get the value from the first parameter
        let value = insert_func.get_first_param().unwrap().into_int_value();

        builder.position_at_end(stack_not_full_block);

        stack.store(compiler, value);
      }

      builder.build_return(None);
    }
  }

  pub fn call_from_int(compiler: &Compiler<'_>, value: i32) {
    let builder = compiler.builder();

    builder.build_call(
      Self::get(compiler),
      &[compiler.i32_type().const_int(value as u64, false).into()],
      "push_int_call",
    );
  }

  pub fn call(compiler: &Compiler<'_>, args: &[inkwell::values::BasicMetadataValueEnum]) {
    compiler
      .builder()
      .build_call(Self::get(compiler), args, "printf_call");
  }

  pub fn get<'ctx>(compiler: &Compiler<'ctx>) -> inkwell::values::FunctionValue<'ctx> {
    compiler.module().get_function("push").unwrap()
  }
}
