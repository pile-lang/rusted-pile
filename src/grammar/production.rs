use super::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Production {
  pub lhs: Symbol,
  pub rhs: Vec<Symbol>,
}

impl Production {
  pub fn new(lhs: Symbol, rhs: Vec<Symbol>) -> Production {
    Production { lhs, rhs }
  }

  pub fn lhs(&self) -> &Symbol {
    &self.lhs
  }

  pub fn rhs(&self) -> &Vec<Symbol> {
    &self.rhs
  }

  pub fn rhs_mut(&mut self) -> &mut Vec<Symbol> {
    &mut self.rhs
  }

  pub fn lhs_mut(&mut self) -> &mut Symbol {
    &mut self.lhs
  }

  pub fn add_dot(&self) -> Self {
    let (lhs, rhs) = self.clone().into();
    let mut new_rhs = rhs;
    new_rhs.insert(0, Symbol::Dot);

    Production::new(lhs, new_rhs)
  }

  pub fn forward_dot(&self) -> Self {
    let (lhs, rhs) = self.clone().into();
    let dot_index = rhs
      .iter()
      .position(|symbol| symbol == &Symbol::Dot)
      .expect("There should be a dot in every production");

    if dot_index + 1 >= rhs.len() {
      return Production::new(lhs, rhs);
    }

    let mut new_rhs = rhs;
    new_rhs.swap(dot_index, dot_index + 1);

    Production::new(lhs, new_rhs)
  }

  pub fn next_symbol_after_dot(&self) -> Option<Symbol> {
    let (_, rhs) = self.clone().into();
    let dot_index = rhs.iter().position(|symbol| symbol == &Symbol::Dot);

    if let Some(dot_index) = dot_index {
      if dot_index + 1 < rhs.len() {
        return Some(rhs[dot_index + 1].clone());
      }
    }

    None
  }

  pub fn copy_without_dot(&self) -> Self {
    let (lhs, rhs) = self.clone().into();
    let new_rhs = rhs
      .iter()
      .filter(|symbol| symbol != &&Symbol::Dot)
      .cloned()
      .collect();

    Production::new(lhs, new_rhs)
  }
}

impl From<Production> for (Symbol, Vec<Symbol>) {
  fn from(val: Production) -> Self {
    (val.lhs, val.rhs)
  }
}

impl From<(Symbol, Vec<Symbol>)> for Production {
  fn from(val: (Symbol, Vec<Symbol>)) -> Self {
    Production::new(val.0, val.1)
  }
}
