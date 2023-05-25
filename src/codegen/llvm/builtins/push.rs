use inkwell::values::IntValue;

use crate::codegen::llvm::{
  globals::stack::Stack, manager::LLVMManager, utils::get_params::FunctionParams,
};

use super::abort::AbortBuiltinFunction;

pub fn generate_push_function() {
  let manager = LLVMManager::get();
  let (context, module, builder) = manager.fetch_all();

  let i32_type = context.i32_type();
  let insert_type = context.void_type().fn_type(&[i32_type.into()], false);
  let insert_func = module.add_function("push", insert_type, None);
  let entry = context.append_basic_block(insert_func, "entry");

  {
    builder.position_at_end(entry);

    // Prelude. Get the value from the first parameter
    let value = insert_func
      .get_param::<IntValue>(0)
      .expect("Expected a value in the first parameter of the push function");
    let stack_manager = Stack::get();

    // 1. Check if the stack is full
    let stack_full_block = context.append_basic_block(insert_func, "stack_full");
    let stack_not_full_block = context.append_basic_block(insert_func, "stack_not_full");
    builder.build_conditional_branch(
      stack_manager.is_full(),
      stack_full_block,
      stack_not_full_block,
    );

    // 2. If the stack is full
    {
      builder.position_at_end(stack_full_block);

      AbortBuiltinFunction::call_from_values(
        "[ABORT @ push]: stack is already full",
        1,
        Some("error_message_stack_full".to_string()),
        None,
      );

      builder.build_unreachable();
    }

    // 3. If not then store the value
    {
      builder.position_at_end(stack_not_full_block);

      stack_manager.store(value);
    }

    builder.build_return(None);
  }
}
