use logos::Logos;
use miette::Result as MietteResult;

use super::{
  errors::LexerError,
  tokens::{span_to_tuple, Token},
  PileToken,
};

pub fn compute_tokens(input: &str) -> MietteResult<Vec<PileToken>> {
  let mut lex = Token::lexer(input);
  let mut tokens: Vec<PileToken> = Vec::new();

  while let (Some(token), slice, span) = (lex.next(), lex.slice(), lex.span()) {
    match token {
      Token::Error => {
        return Err(LexerError::UnsupportedFormat {
          input: input.to_string(),
          extension_src: span_to_tuple(lex.span()),
          advice: "test".to_string(),
        })?;
      }
      _ => {
        tokens.push(PileToken {
          token,
          span,
          slice: slice.into(),
        });
      }
    }
  }

  Ok(tokens)
}
