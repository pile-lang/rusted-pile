use std::collections::HashMap;

use crate::grammar::{production::Production, Grammar, Symbol};

use super::{closure::ClosureItem, Action};

#[allow(dead_code)]
pub struct SLR {
  grammar: Grammar,
  action_table: HashMap<(usize, Symbol), Action>,
  goto_table: HashMap<(usize, Symbol), usize>,
  closure_set: HashMap<Vec<Production>, ClosureItem>,
}

impl SLR {
  pub fn new(grammar: Grammar) -> SLR {
    // Build the first closure set I0
    let mut kernel = Vec::new();
    let first_production: Production = (grammar.productions[0].clone()).into();
    kernel.push(first_production.add_dot());

    let mut i0 = ClosureItem::new(kernel, &grammar, 0);
    let closure_set = i0.populate(&grammar);

    let mut slr = SLR {
      grammar,
      closure_set,
      action_table: HashMap::new(),
      goto_table: HashMap::new(),
    };

    // Store all the closure items in a vector
    let mut closure_items = Vec::new();
    slr.closure_set.iter().for_each(|(_, closure_item)| {
      closure_items.push(closure_item.clone());
    });

    closure_items.sort();

    // print closure items
    closure_items.iter().for_each(|closure_item| {
      println!("{}", closure_item);
    });

    slr.build_tables();

    slr
  }

  fn build_tables(&mut self) {
    todo!()
  }

  #[allow(dead_code)]
  fn actions(&self, _state: usize, _symbol: Symbol) -> Option<&Action> {
    todo!()
  }

  #[allow(dead_code)]
  fn goto(&self, _state: usize, _symbol: Symbol) -> Option<&usize> {
    todo!()
  }
}
