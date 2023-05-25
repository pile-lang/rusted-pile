use inkwell::{
  types::FunctionType,
  values::{FunctionValue, IntValue, PointerValue},
  AddressSpace,
};

use crate::codegen::llvm::{
  externs::{exit::ExitExtern, printf::PrintfExtern},
  manager::LLVMManager,
  utils::get_params::FunctionParams,
};

pub struct AbortBuiltinFunction;

impl AbortBuiltinFunction {
  pub fn declare() {
    let manager = LLVMManager::get();
    let (context, module, builder) = manager.fetch_all();

    let abort_type = context.void_type().fn_type(
      &[
        context.i32_type().ptr_type(AddressSpace::default()).into(), // Message
        context.i32_type().into(),                                   // Exit code
      ],
      false,
    );
    let abort_func = module.add_function("abort", abort_type, None);

    let basic_block = context.append_basic_block(abort_func, "entry");
    {
      builder.position_at_end(basic_block);

      let message_ptr = abort_func.get_param::<PointerValue>(0).unwrap();
      let exit_code = abort_func.get_param::<IntValue>(1).unwrap();

      PrintfExtern::call(&[message_ptr.into()]);
      ExitExtern::call_from_int(201);

      builder.build_unreachable();
    }

    builder.build_return(None);
  }

  pub fn get() -> FunctionValue<'static> {
    let manager = LLVMManager::get();
    let module = manager.module();

    module.get_function("abort").unwrap_or_else(|| {
      Self::declare();
      module
        .get_function("abort")
        .expect("abort function not found")
    })
  }

  pub fn call_from_values(
    message: &str,
    code: i32,
    string_name: Option<String>,
    call_name: Option<String>,
  ) {
    let string_name = string_name.unwrap_or_else(|| "message".to_string());
    let call_name = call_name.unwrap_or_else(|| "abort_call".to_string());

    let manager = LLVMManager::get();
    let builder = manager.builder();

    let message_ptr = builder.build_global_string_ptr(message, &string_name);
    builder.build_call(
      Self::get(),
      &[
        message_ptr.as_pointer_value().into(),
        manager.const_int(code as u64).into(),
      ],
      &call_name,
    );
  }

  pub fn call(args: &[inkwell::values::BasicMetadataValueEnum]) {
    let manager = LLVMManager::get();

    manager
      .builder()
      .build_call(Self::get(), args, "abort_call");
  }
}
