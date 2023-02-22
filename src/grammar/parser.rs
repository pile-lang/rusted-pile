use std::collections::HashMap;

use logos::{Lexer, Logos};

use super::{tokens::GLCTokens, Grammar, Symbol};

pub fn remove_angle_brackets(s: &str) -> String {
  s.replace(['<', '>'], "")
}

impl TryFrom<&mut Lexer<'_, GLCTokens>> for Grammar {
  type Error = &'static str;

  fn try_from(value: &mut Lexer<GLCTokens>) -> Result<Self, Self::Error> {
    let mut productions = Vec::new();

    let mut current_lhs: Option<Symbol> = None;
    let mut current_rhs: Vec<Symbol> = Vec::new();

    while let (Some(token), slice, _span) = (value.next(), value.slice(), value.span()) {
      match token {
        GLCTokens::NonTerminal => {
          if current_lhs.is_some() {
            current_rhs.push(Symbol::NonTerminal(remove_angle_brackets(slice)));
            continue;
          }

          current_lhs = Some(Symbol::NonTerminal(remove_angle_brackets(slice)));
        }
        GLCTokens::Terminal => {
          if current_lhs.is_none() {
            return Err("Terminal symbols can only be defined after a non terminal symbol");
          }

          current_rhs.push(Symbol::Terminal(slice.to_string()));
        }
        GLCTokens::Epsilon => {
          if current_lhs.is_none() {
            return Err("Epsilon can only be defined as the production of a non terminal symbol");
          }

          current_rhs.push(Symbol::Empty);
        }
        GLCTokens::Arrow => {
          if current_lhs.is_none() {
            return Err("Arrow can only be defined after a non terminal symbol");
          }
        }
        GLCTokens::Pipe => {
          if current_lhs.is_none() {
            return Err("Pipe can only be defined after a non terminal symbol");
          }

          productions.push((current_lhs.clone().unwrap(), current_rhs));
          current_rhs = Vec::new();
        }
        GLCTokens::EndOfProduction => {
          if current_lhs.is_none() {
            return Err("End of production can only be defined after a non terminal symbol");
          }

          productions.push((current_lhs.clone().unwrap(), current_rhs));
          current_lhs = None;
          current_rhs = Vec::new();
        }
        _ => {
          return Err("Unexpected token");
        }
      }
    }

    if let Some(current_non_terminal) = current_lhs {
      productions.push((current_non_terminal, current_rhs));
    }

    Ok(Self {
      productions,
      first_set: HashMap::new(),
      follow_set: HashMap::new(),
    })
  }
}

pub fn parse(input: &str) -> Result<Grammar, &'static str> {
  let mut lexer = GLCTokens::lexer(input);

  Grammar::try_from(&mut lexer)
}
