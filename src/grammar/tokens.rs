use std::fmt::Display;

use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum GLCTokens {
  #[regex(r"[ \t\n\f]+", logos::skip)]
  #[error]
  Error,

  /// Non terminal symbols
  #[regex(r"<[a-zA-Z0-9\-]+>")]
  NonTerminal,

  /// Terminal symbols
  #[regex(r"[a-zA-Z0-9\-]+")]
  Terminal,

  /// The arrow symbol
  #[token("->")]
  Arrow,

  /// The empty string
  #[token("ε")]
  Epsilon,

  /// The pipe symbol
  #[token("|")]
  Pipe,

  /// The end of a production
  #[token(";")]
  EndOfProduction,
}

impl Display for GLCTokens {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}
