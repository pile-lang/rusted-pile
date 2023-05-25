use miette::Result as MietteResult;
use rusted_pile::{
  codegen::{self, CodeGeneratorTarget},
  grammar, lexer,
  parser::SLR::SLR,
};
use std::fs;

fn main() -> MietteResult<(), Box<dyn std::error::Error>> {
  // Lexer
  let lang_contents = fs::read_to_string("assets/lang/test.pile")?;
  let tokens = lexer::generate::compute_tokens(&lang_contents)?;

  // Parser
  let glc_contents = fs::read_to_string("assets/glc/lang.glc")?;
  let mut glc = grammar::parser::parse(&glc_contents)?;

  glc.compute_follow_set().expand();

  let abstract_syntax_tree = SLR::new(glc)
    .parse(tokens, &lang_contents)?
    .ok_or("Failed to parse")?;

  // Codegen
  println!("{}", abstract_syntax_tree);
  codegen::code_generator(CodeGeneratorTarget::LLVM).generate(abstract_syntax_tree)?;

  Ok(())
}
