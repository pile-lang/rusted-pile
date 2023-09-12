use crate::{
  codegen::llvm::{
    builtins::{pop::PopBuiltin, push::PushBuiltin},
    compiler::Compiler, generate_code::GenerateLLVMIR,
  },
  parser::parse::AstNode,
};

pub fn generate(compiler: &Compiler<'_>, ast: &AstNode) -> anyhow::Result<()> {
  GenerateLLVMIR::generate(compiler, &ast.children[0])?;
  GenerateLLVMIR::generate(compiler, &ast.children[1])?;

  let right = PopBuiltin::call(compiler);
  let left = PopBuiltin::call(compiler);

  let result = compiler.builder().build_int_mul(left, right, "multmp");

  PushBuiltin::call(compiler, &[result.into()]);

  Ok(())
}
