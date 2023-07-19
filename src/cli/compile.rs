use clap::{Args, ValueEnum};
use miette::Result as MietteResult;

use crate::{
  codegen::{self, CodeGeneratorTarget},
  grammar, lexer,
  parser::SLR::SLR,
};

use super::PileCompiler;

#[allow(clippy::upper_case_acronyms)]
#[derive(ValueEnum, Clone)]
pub enum Codegen {
  VM,
  LLVM,
}

#[derive(Args)]
pub struct Compile {
  #[arg(required = true, short, long)]
  pub filename: String,

  #[arg(short, long, default_value = "VM")]
  pub codegen: Codegen,

  #[arg(short, long, default_value = "output")]
  pub output: String,
}

impl PileCompiler {
  pub fn compile(
    Compile {
      filename,
      codegen,
      output,
    }: &Compile,
  ) -> MietteResult<(), Box<dyn std::error::Error>> {
    // Lexer
    let lang_contents = std::fs::read_to_string(filename)?;
    let tokens = lexer::generate::compute_tokens(&lang_contents)?;

    // Parser
    let glc_contents = std::fs::read_to_string("assets/glc/lang.glc")?;
    let mut glc = grammar::parser::parse(&glc_contents)?;

    glc.compute_follow_set().expand();

    let abstract_syntax_tree = SLR::new(glc)
      .parse(tokens, &lang_contents)?
      .ok_or("Failed to parse")?;

    match codegen {
      Codegen::VM => codegen::code_generator(CodeGeneratorTarget::VirtualMachine),
      Codegen::LLVM => codegen::code_generator(CodeGeneratorTarget::LLVM),
    }
    .generate(abstract_syntax_tree, output.clone())?;

    Ok(())
  }
}
