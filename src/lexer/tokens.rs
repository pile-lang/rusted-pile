use logos::{Lexer, Logos};
use std::fmt::Display;

fn def_type(lex: &mut Lexer<Token>) -> Option<&'static str> {
  let slice = lex.slice();
  let slice = &slice[4..slice.len() - 1];
  match slice {
    "i32" => Some("i32"),
    "i64" => Some("i64"),
    "f32" => Some("f32"),
    "f64" => Some("f64"),
    _ => None,
  }
}

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
  /// Plus, minus, times, divide, modulo
  #[regex(r"\+|-|\*|/|%")]
  ArithmeticOp,

  /// Comparison operators
  #[regex(r"=|<>|<=|>=|<|>")]
  ComparisonOp,

  /// Cast (::)
  #[regex(r"::")]
  CastOp,

  /// Keywords

  /// Stack Ops
  #[regex(r"drop|dup")]
  StackOps,

  // @ sign
  #[token("@")]
  AtSign,

  #[token("while")]
  While,

  #[token("do")]
  Do,

  #[token("end")]
  End,

  #[token("if")]
  If,

  #[token("else")]
  Else,

  #[token("range")]
  Range,

  /// Types
  #[regex("i32|i64|f32|f64")]
  Types,

  /// Def Type (def(i32))
  #[regex("def\\((i32|i64|f32|f64)\\)", def_type)]
  DefType(&'static str),

  /// Identifiers
  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
  Identifier,

  /// End of input
  #[regex(r"\$")]
  EndOfInput,
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub fn span_to_tuple(span: logos::Span) -> (usize, usize) {
  (span.start, span.end - span.start)
}

impl Token {
  /// Given any enum that carry a Data, get only the enum variant
  /// Example: `DefType("i32")` -> `DefType`
  pub fn get_token_type_only(&self) -> String {
    self.to_string().split('(').next().unwrap().to_string()
  }
}
