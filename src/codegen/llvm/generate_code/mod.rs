// Iter over the AST and call the needed functions to generate LLVM IR

use crate::{
  lexer::tokens::{ArithmeticOperators, StackOperators, Token},
  parser::parse::AstNode,
};

use super::{builtins::pop::PopBuiltin, compiler::Compiler};

pub mod arithmetic;
pub mod stack;

pub struct GenerateLLVMIR;

impl GenerateLLVMIR {
  pub fn generate(compiler: &Compiler<'_>, ast: &AstNode) -> anyhow::Result<()> {
    match ast.token {
      Token::Program => {
        let module = compiler.module();
        let builder = compiler.builder();

        let main_type = compiler.i32_type().fn_type(&[], false);
        let main_func = module.add_function("main", main_type, None);
        let entry = compiler.append_basic_block(main_func, "entry");
        builder.position_at_end(entry);

        for child in ast.children.iter() {
          GenerateLLVMIR::generate(compiler, child)?;
        }

        let top_element = PopBuiltin::call(compiler);
        builder.build_return(Some(&top_element));
      }
      Token::ArithmeticOp(operator) => match operator {
        ArithmeticOperators::Plus => arithmetic::plus::generate(compiler, ast)?,
        ArithmeticOperators::Times => arithmetic::times::generate(compiler, ast)?,
        ArithmeticOperators::Minus => arithmetic::minus::generate(compiler, ast)?,
        ArithmeticOperators::Divide => arithmetic::divide::generate(compiler, ast)?,
        ArithmeticOperators::Modulo => arithmetic::modulo::generate(compiler, ast)?,
      },
      Token::Integer(..) => stack::push::generate(compiler, ast)?,
      Token::StackOps(operator) => match operator {
        StackOperators::Dump => stack::dump::generate(compiler, ast)?,
        _ => todo!(),
      },
      _ => todo!(),
    }

    Ok(())
  }
}
