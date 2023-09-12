use crate::{
  codegen::llvm::{builtins::push::PushBuiltin, compiler::Compiler},
  lexer::tokens::Token,
  parser::parse::AstNode,
};

pub fn generate(compiler: &Compiler<'_>, ast: &AstNode) -> anyhow::Result<()> {
  match ast.token {
    Token::Integer(value) => {
      PushBuiltin::call_from_int(compiler, value);
      Ok(())
    }
    _ => Err(anyhow::anyhow!("Invalid token")),
  }
}

pub fn generate_from_int(compiler: &Compiler<'_>, value: i32) {
  PushBuiltin::call_from_int(compiler, value);
}
