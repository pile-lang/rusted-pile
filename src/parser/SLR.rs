use std::collections::HashMap;

use crate::{
  grammar::{production::Production, Grammar, Symbol},
  parser::closure::{Lhs, Rhs},
};

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

    slr.build_tables();

    slr
  }

  fn build_tables(&mut self) {
    let grammar_productions = self.grammar.productions.clone();

    for (
      _,
      ClosureItem {
        id,
        transitions,
        productions,
        ..
      },
    ) in &self.closure_set
    {
      for production in productions {
        let (_, rhs): (Lhs, Rhs) = production.clone().into();
        match rhs.last().unwrap() {
          Symbol::Dot => {
            let production_without_dot = production.copy_without_dot();

            for (idx, _prod) in grammar_productions.iter().enumerate() {
              let (lhs, rhs): (Lhs, Rhs) = _prod.clone().into();
              if lhs == production_without_dot.lhs && rhs == production_without_dot.rhs {
                if idx == 0 {
                  // apply R3
                  self.action_table.insert((*id, Symbol::End), Action::Accept);
                } else {
                  // apply R2
                  let follow_set = self.grammar.follow_set.get(&lhs).unwrap();

                  for follow in follow_set {
                    self
                      .action_table
                      .insert((*id, follow.clone()), Action::Reduce(idx));
                  }
                }
              }
            }
          }
          _ => {}
        }
      }

      for (symbol, goto) in transitions {
        // apply R1 and R4
        match symbol {
          Symbol::Terminal(_) => {
            self
              .action_table
              .insert((*id, symbol.clone()), Action::Shift(*goto));
          }
          Symbol::NonTerminal(_) => {
            self.goto_table.insert((*id, symbol.clone()), *goto);
          }
          _ => {}
        }
      }
    }

    // define all the reduce actions
    for key in self.action_table.clone().keys() {
      let action = self.action_table.get(key).unwrap();
      let (idx, _) = key;

      match action {
        Action::Reduce(_action) => {
          self.add_action(idx.clone(), &Action::Reduce(*_action));
        }
        _ => {}
      }
    }

    // to define all errors
    for key in self.action_table.clone().keys() {
      let action = self.action_table.get(key).unwrap();

      match action {
        Action::Shift(_) => {
          self.add_action(key.0, &Action::Error);
        }
        _ => {}
      }
    }
  }

  fn add_action(&mut self, row: usize, action: &Action) {
    let terminals = self.grammar.terminals();

    for terminal in terminals {
      match self.action_table.get(&(row, terminal.clone())) {
        None => {
          self
            .action_table
            .insert((row, terminal.clone()), action.clone());
        }
        _ => {}
      };
    }
  }
}
