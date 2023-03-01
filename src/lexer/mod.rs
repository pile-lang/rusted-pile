use std::ops::Range;

use serde::Serialize;

use self::tokens::Token;

pub mod errors;
pub mod generate;
pub mod tokens;

#[derive(Debug, Clone, Serialize)]
pub struct PileToken {
  pub token: Token,
  pub slice: String,
  pub span: Range<usize>,
}
