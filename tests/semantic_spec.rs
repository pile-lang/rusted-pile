use rusted_pile::{grammar, lexer, parser::SLR::SLR, semantic::SemanticAnalyzer};

fn preemble(input: &str) -> Result<(), Box<dyn std::error::Error>> {
  let input = format!("assets/lang/semantic-tests/{}", input);

  let lang_contents = std::fs::read_to_string(input)?;
  let tokens = lexer::generate::compute_tokens(&lang_contents)?;

  let glc_contents = std::fs::read_to_string("assets/glc/lang.glc")?;
  let mut glc = grammar::parser::parse(&glc_contents)?;

  glc.compute_follow_set().expand();

  SLR::new(glc).parse(tokens.clone(), &lang_contents)?;
  SemanticAnalyzer::new(&lang_contents).analyze(tokens)?;

  Ok(())
}

#[test]
fn integer_sum_should_work() -> Result<(), Box<dyn std::error::Error>> {
  insta::assert_debug_snapshot!(preemble("integer_sum_should_work.pile"));

  Ok(())
}

#[test]
fn bool_sum_should_not_work() -> Result<(), Box<dyn std::error::Error>> {
  insta::assert_debug_snapshot!(preemble("bool_sum_should_not_work.pile"));

  Ok(())
}

#[test]
fn float_sum_should_work() -> Result<(), Box<dyn std::error::Error>> {
  insta::assert_debug_snapshot!(preemble("float_sum_should_work.pile"));

  Ok(())
}
