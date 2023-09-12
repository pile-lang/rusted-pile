use logos::{Lexer, Logos};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  I32,
  I64,
  F32,
  F64,
}

fn def_type(lex: &mut Lexer<Token>) -> Option<Type> {
  let slice = lex.slice();
  let slice = &slice[4..slice.len() - 1];
  match slice {
    "i32" => Some(Type::I32),
    "i64" => Some(Type::I64),
    "f32" => Some(Type::F32),
    "f64" => Some(Type::F64),
    _ => None,
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithmeticOperators {
  Plus,
  Minus,
  Times,
  Divide,
  Modulo,
}

fn parse_arithmetic_op(lex: &mut Lexer<Token>) -> Option<ArithmeticOperators> {
  let slice = lex.slice();
  match slice {
    "+" => Some(ArithmeticOperators::Plus),
    "-" => Some(ArithmeticOperators::Minus),
    "*" => Some(ArithmeticOperators::Times),
    "/" => Some(ArithmeticOperators::Divide),
    "%" => Some(ArithmeticOperators::Modulo),
    _ => None,
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComparisonOperators {
  EqualTo,
  NotEqualTo,
  LessThan,
  LessThanOrEqualTo,
  GreaterThan,
  GreaterThanOrEqualTo,
}

fn parse_comparison_op(lex: &mut Lexer<Token>) -> Option<ComparisonOperators> {
  let slice = lex.slice();
  match slice {
    "=" => Some(ComparisonOperators::EqualTo),
    "<>" => Some(ComparisonOperators::NotEqualTo),
    "<" => Some(ComparisonOperators::LessThan),
    "<=" => Some(ComparisonOperators::LessThanOrEqualTo),
    ">" => Some(ComparisonOperators::GreaterThan),
    ">=" => Some(ComparisonOperators::GreaterThanOrEqualTo),
    _ => None,
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StackOperators {
  Drop,
  Dup,
  Dump,
}

fn parse_stack_op(lex: &mut Lexer<Token>) -> Option<StackOperators> {
  let slice = lex.slice();
  match slice {
    "drop" => Some(StackOperators::Drop),
    "dup" => Some(StackOperators::Dup),
    "dump" => Some(StackOperators::Dump),
    _ => None,
  }
}

fn parse_to_string(lex: &mut Lexer<Token>) -> Option<String> {
  let slice = lex.slice();
  // remove the quotes
  let slice = &slice[1..slice.len() - 1];
  Some(slice.to_string())
}

fn to_boolean(lex: &mut Lexer<Token>) -> Option<bool> {
  let slice = lex.slice();
  match slice {
    "true" => Some(true),
    "false" => Some(false),
    _ => None,
  }
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
  #[regex(r"[ \t\n\f]+", logos::skip)]
  #[error]
  Error,

  /// Integer literals
  #[regex(r"[0-9]+", |lex| lex.slice().parse(), priority = 2)]
  #[regex(r"0[xX][0-9a-fA-F]+", |lex| i32::from_str_radix(&lex.slice()[2..], 16))]
  #[regex(r"0b[0-1]+", |lex| i32::from_str_radix(&lex.slice()[2..], 2))]
  #[regex(r"0o[0-7]+", |lex| i32::from_str_radix(&lex.slice()[2..], 8))]
  Integer(i32),

  /// Float literals
  #[regex(r"[+-]?([0-9]*[.])?[0-9]+([eE][+-]?[0-9]+)?", |lex| lex.slice().parse(), priority = 1)]
  Float(f32),

  /// Boolean literals
  #[token("true", to_boolean)]
  #[token("false", to_boolean)]
  Boolean(bool),

  /// String literals
  #[regex(r#""([^"\\]|\\.)*""#, parse_to_string)]
  String(String),

  /// Operators
  /// Plus, minus, times, divide, modulo
  #[regex(r"\+|-|\*|/|%", parse_arithmetic_op)]
  ArithmeticOp(ArithmeticOperators),

  /// Comparison operators
  #[regex(r"=|<>|<=|>=|<|>", parse_comparison_op)]
  ComparisonOp(ComparisonOperators),

  /// Cast (::)
  #[regex(r"::")]
  CastOp,

  /// Keywords

  /// Stack Ops
  #[regex(r"drop|dup|dump", parse_stack_op)]
  StackOps(StackOperators),

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
  DefType(Type),

  /// Identifiers
  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
  Identifier,

  /// End of input
  #[regex(r"\$")]
  EndOfInput,

  /// Comments (\)
  #[regex(r"\\.*", logos::skip)]
  Comment,

  /// Decoy Program token
  Program,
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
