use inkwell::context::Context;

use crate::parser::parse::AstNode;

use self::{
  builtins::{abort::AbortBuiltin, pop::PopBuiltin, push::PushBuiltin},
  compiler::Compiler,
  externs::{exit::ExitExtern, printf::PrintfExtern},
  globals::stack::Stack,
};

use super::CodeGenerator;

pub mod builtins;
pub mod compiler;
pub mod externs;
pub mod generate_code;
pub mod globals;

#[derive(Default)]
pub struct LLVMCodeGenerator;

impl CodeGenerator for LLVMCodeGenerator {
  fn generate(&mut self, ast: AstNode) -> anyhow::Result<()> {
    // This trick is to ensure that stack is dropped before context
    let stack;
    {
      let context = Context::create();
      let compiler = Compiler::new(&context, "main");
      stack = Stack::new(64 * 1024, &compiler);

      PrintfExtern::declare(&compiler);
      ExitExtern::declare(&compiler);
      AbortBuiltin::declare(&compiler);
      PushBuiltin::declare(&compiler, &stack);
      PopBuiltin::declare(&compiler, &stack);

      generate_code::GenerateLLVMIR::generate(&compiler, &ast)?;
    }

    Ok(())
  }
}
