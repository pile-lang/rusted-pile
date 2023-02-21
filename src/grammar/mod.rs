use std::fmt::Display;

pub mod expand;
pub mod parser;
pub mod tokens;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Symbol {
  Terminal(String),
  NonTerminal(String),
  Empty,
  Dot,
}

impl Symbol {
  pub fn is_terminal(&self) -> bool {
    matches!(self, Symbol::Terminal(_))
  }

  pub fn is_non_terminal(&self) -> bool {
    matches!(self, Symbol::NonTerminal(_))
  }

  pub fn is_empty(&self) -> bool {
    matches!(self, Symbol::Empty)
  }

  pub fn get_name(&self) -> String {
    match self {
      Symbol::Terminal(s) => s.clone(),
      Symbol::NonTerminal(s) => s.clone(),
      Symbol::Empty => "ε".to_string(),
      Symbol::Dot => todo!(),
    }
  }
}

impl Display for Symbol {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Symbol::Terminal(s) => write!(f, "{}", s),
      Symbol::NonTerminal(s) => write!(f, "<{}>", s),
      Symbol::Empty => write!(f, "ε"),
      Symbol::Dot => todo!(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Grammar {
  pub productions: Vec<(Symbol, Vec<Symbol>)>,
}

impl Display for Grammar {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (lhs, rhs) in &self.productions {
      write!(f, "{} -> ", lhs)?;

      for (i, symbol) in rhs.iter().enumerate() {
        if i != 0 {
          write!(f, " | ")?;
        }

        write!(f, "{}", symbol)?;
      }

      writeln!(f)?;
    }

    Ok(())
  }
}
