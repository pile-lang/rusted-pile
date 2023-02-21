use logos::Logos;
use std::fmt::Display;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum Token {
  #[regex(r"[ \t\n\f]+", logos::skip)]
  #[error]
  Error,

  /// Integer literals
  #[regex(r"[0-9]+")]
  #[regex(r"0[xX][0-9a-fA-F]+")]
  #[regex(r"0b[0-1]+")]
  #[regex(r"0o[0-7]+")]
  Integer,

  /// Float literals
  #[regex("[0-9]+\\.[0-9]+")]
  #[regex("\\.[0-9]+")]
  #[regex("[0-9]+\\.")]
  #[regex(r"[0-9]+e[0-9]+")]
  Float,

  /// Operators
  #[token("+")]
  Plus,

  /// Keywords
  #[regex("drop|dup")]
  Keyword,
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub fn span_to_tuple(span: logos::Span) -> (usize, usize) {
  (span.start, span.end - span.start)
}
