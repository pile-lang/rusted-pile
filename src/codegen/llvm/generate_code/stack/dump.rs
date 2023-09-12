use crate::codegen::llvm::{builtins::pop::PopBuiltin, externs::printf::PrintfExtern};

pub fn generate(
  compiler: &crate::codegen::llvm::compiler::Compiler<'_>,
  _ast: &crate::parser::parse::AstNode,
) -> anyhow::Result<()> {
  let value = PopBuiltin::call(compiler);
  let message_ptr = compiler
    .builder()
    .build_global_string_ptr("%d\n", "dump_message")
    .as_pointer_value();

  PrintfExtern::call(compiler, &[message_ptr.into(), value.into()]);

  Ok(())
}
