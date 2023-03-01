use logos::{Lexer, Logos};
use serde::Serialize;
use std::fmt::Display;

fn def_type(lex: &mut Lexer<Token>) -> Option<&'static str> {
  let slice = lex.slice();
  let slice = &slice[4..slice.len() - 1];
  match slice {
    "i32" => Some("i32"),
    "i64" => Some("i64"),
    "f32" => Some("f32"),
    "f64" => Some("f64"),
    "bool" => Some("bool"),
    _ => None,
  }
}

fn _def_type_with_init(lex: &mut Lexer<Token>) -> Option<&'static str> {
  let slice = lex.slice();
  let slice = &slice[4..slice.len() - 1];
  let slice = slice.split(',').collect::<Vec<&str>>();
  match slice[0] {
    "i32" => Some("i32"),
    "i64" => Some("i64"),
    "f32" => Some("f32"),
    "f64" => Some("f64"),
    "bool" => Some("bool"),
    _ => None,
  }
}

fn types(lex: &mut Lexer<Token>) -> Option<&'static str> {
  let slice = lex.slice();
  match slice {
    "i32" => Some("i32"),
    "i64" => Some("i64"),
    "f32" => Some("f32"),
    "f64" => Some("f64"),
    "bool" => Some("bool"),
    _ => None,
  }
}

fn to_integer(lex: &mut Lexer<Token>) -> Option<i32> {
  let slice = lex.slice();
  if slice.starts_with("0x") {
    i32::from_str_radix(&slice[2..], 16).ok()
  } else if slice.starts_with("0b") {
    i32::from_str_radix(&slice[2..], 2).ok()
  } else if slice.starts_with("0o") {
    i32::from_str_radix(&slice[2..], 8).ok()
  } else {
    slice.parse().ok()
  }
}

fn to_float(lex: &mut Lexer<Token>) -> Option<f32> {
  let slice = lex.slice();
  slice.parse().ok()
}

fn to_boolean(lex: &mut Lexer<Token>) -> Option<bool> {
  let slice = lex.slice();
  match slice {
    "true" => Some(true),
    "false" => Some(false),
    _ => None,
  }
}

#[derive(Logos, Debug, Clone, Copy, PartialEq, Serialize)]
#[logos(subpattern decimal = r"[0-9]+")]
#[logos(subpattern hex = r"0[xX][0-9a-fA-F]+")]
#[logos(subpattern binary = r"0b[0-1]+")]
#[logos(subpattern octal = r"0o[0-7]+")]
#[logos(subpattern full_float = r"[0-9]+\.[0-9]+")]
#[logos(subpattern right_float = r"\.[0-9]+")]
#[logos(subpattern left_float = r"[0-9]+\.")]
#[logos(subpattern exponent_float = r"[0-9]+e[0-9]+")]
pub enum Token {
  #[regex(r"[ \t\n\f]+", logos::skip)]
  #[error]
  Error,

  /// Integer literals
  #[regex(r"(?&decimal)", to_integer)]
  #[regex(r"(?&hex)", to_integer)]
  #[regex(r"(?&binary)", to_integer)]
  #[regex(r"(?&octal)", to_integer)]
  Integer(i32),

  /// Float literals
  #[regex(r"(?&full_float)", to_float)]
  #[regex(r"(?&right_float)", to_float)]
  #[regex(r"(?&left_float)", to_float)]
  #[regex(r"(?&exponent_float)", to_float)]
  Float(f32),

  /// Boolean literals
  #[token("true", to_boolean)]
  #[token("false", to_boolean)]
  Boolean(bool),

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
  #[regex("i32|i64|f32|f64|bool", types)]
  Types(&'static str),

  /// Def Type (def(i32))
  #[regex("def\\((i32|i64|f32|f64|bool)\\)", def_type)]
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
