use inkwell::values::{FunctionValue, IntValue, PointerValue};

use crate::codegen::llvm::{compiler::Compiler, globals::stack::Stack};

use super::abort::AbortBuiltin;

pub const POP_BUILTIN_FUNCTION_NAME: &str = "pop";

pub struct PopBuiltin;

impl PopBuiltin {
  pub fn declare<'ctx>(compiler: &Compiler<'ctx>, stack: &'ctx Stack<'ctx>) {
    let builder = compiler.builder();
    let module = compiler.module();

    let i32_type = compiler.i32_type();
    let remove_type = i32_type.fn_type(&[], false);
    let remove_func = module.add_function("pop", remove_type, None);
    let entry = compiler.append_basic_block(remove_func, "entry");

    builder.position_at_end(entry);

    // 1. Check if the stack is empty
    let stack_empty_block = compiler.append_basic_block(remove_func, "stack_empty");
    let stack_not_empty_block = compiler.append_basic_block(remove_func, "stack_not_empty");
    builder.build_conditional_branch(
      stack.is_empty(compiler),
      stack_empty_block,
      stack_not_empty_block,
    );

    // 2. If the stack is empty
    builder.position_at_end(stack_empty_block);
    AbortBuiltin::call_from_values(
      compiler,
      "[ABORT @ pop]: stack is already empty\n",
      1,
      Some("error_message_stack_empty".to_string()),
      Some("error_stack_empty".to_string()),
    );
    builder.build_unreachable();

    // 3. If not then remove the value
    builder.position_at_end(stack_not_empty_block);
    let top_ptr = stack.load_top_ptr(compiler);
    let prev_ptr = unsafe {
      builder.build_in_bounds_gep(
        i32_type,
        top_ptr,
        &[i32_type.const_int(u64::MAX, true)],
        "prevPtr",
      )
    };
    builder.build_store::<PointerValue>(stack.stack_top_ptr(), prev_ptr);

    // Load and return the top element
    let top_element = builder.build_load(i32_type, prev_ptr, "topElement");
    builder.build_return(Some(&top_element));
  }

  pub fn call<'ctx>(compiler: &Compiler<'ctx>) -> IntValue<'ctx> {
    let builder = compiler.builder();

    let top_element = builder.build_call(Self::get(compiler), &[], "topElement");

    top_element
      .try_as_basic_value()
      .left()
      .unwrap()
      .into_int_value()
  }

  pub fn get<'ctx>(compiler: &Compiler<'ctx>) -> FunctionValue<'ctx> {
    let module = compiler.module();
    module
      .get_function(POP_BUILTIN_FUNCTION_NAME)
      .expect("pop function not found")
  }
}
