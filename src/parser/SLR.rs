use std::collections::HashMap;

use crate::grammar::{production::Production, Grammar, Symbol};

use super::{closure::ClosureItem, Action};

#[allow(dead_code)]
pub struct SLR {
  pub grammar: Grammar,
  pub action_table: HashMap<(usize, Symbol), Action>,
  pub goto_table: HashMap<(usize, Symbol), usize>,
  pub closure_set: HashMap<Vec<Production>, ClosureItem>,
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

    slr.build_tables();

    // slr.display_actions_table();

    slr
  }
}
