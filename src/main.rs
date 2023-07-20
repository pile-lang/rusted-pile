use miette::Result as MietteResult;
use rusted_pile::{
  codegen::{self, CodeGeneratorTarget},
  grammar,
  interpreter::vm,
  lexer,
  parser::SLR::SLR,
  semantic,
};
use std::fs;

#[allow(dead_code)]
fn generate() -> MietteResult<(), Box<dyn std::error::Error>> {
  // Get the file name from the command line arguments
  let args: Vec<String> = std::env::args().collect();
  let filename = &args[1];

  // Lexer
  let lang_contents = fs::read_to_string(filename)?;
  let tokens = lexer::generate::compute_tokens(&lang_contents)?;

  // Parser
  let glc_contents = fs::read_to_string("assets/glc/lang.glc")?;
  let mut glc = grammar::parser::parse(&glc_contents)?;

  glc.compute_follow_set().expand();

  let abstract_syntax_tree = SLR::new(glc)
    .parse(tokens, &lang_contents)?
    .ok_or("Failed to parse")?;

  semantic::SemanticAnalyzer::new(lang_contents).analyze(&abstract_syntax_tree)?;

  // Codegen
  // println!("{}", abstract_syntax_tree);
  codegen::code_generator(CodeGeneratorTarget::VirtualMachine).generate(abstract_syntax_tree)?;
  // codegen::code_generator(CodeGeneratorTarget::LLVM).generate(abstract_syntax_tree)?;

  Ok(())
}

fn consume() -> MietteResult<(), Box<dyn std::error::Error>> {
  vm::VMInterpreter::run("bytecode.bin")?;

  Ok(())
}

fn main() -> MietteResult<(), Box<dyn std::error::Error>> {
  generate()?;
  consume()?;
  Ok(())
}
