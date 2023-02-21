use std::collections::HashMap;

use crate::{
  grammar::{Grammar, Symbol},
  lexer::PileToken,
};

#[derive(Debug, Clone)]
enum Action {
  Shift(usize),  // shift to the state with the given index
  Reduce(usize), // reduce using the production with the given index
  Accept,        // accept the input
}

pub struct SLR {
  grammar: Grammar,
  action_table: HashMap<(usize, Symbol), Action>,
  goto_table: HashMap<(usize, Symbol), usize>,
}
impl SLR {
  pub fn new(grammar: Grammar) -> SLR {
    let mut slr = SLR {
      grammar,
      action_table: HashMap::new(),
      goto_table: HashMap::new(),
    };

    slr.build_tables();

    slr
  }

  fn build_tables(&mut self) {
    todo!()
  }

  fn actions(&self, state: usize, symbol: Symbol) -> Option<&Action> {
    todo!()
  }

  fn goto(&self, state: usize, symbol: Symbol) -> Option<&usize> {
    todo!()
  }
}
