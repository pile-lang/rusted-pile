use crate::grammar::Symbol;

use super::{
  closure::{ClosureItem, Lhs, Rhs},
  Action,
  SLR::SLR,
};

impl SLR {
  pub fn build_tables(&mut self) -> &mut Self {
    let grammar_productions = self.grammar.productions.clone();

    self.closure_set.iter().for_each(
      |(
        _,
        ClosureItem {
          id,
          transitions,
          productions,
          ..
        },
      )| {
        for production in productions {
          let (_, rhs): (Lhs, Rhs) = production.clone().into();
          if rhs.last().unwrap() == &Symbol::Dot {
            let production_without_dot = production.copy_without_dot();

            for (idx, _prod) in grammar_productions.iter().enumerate() {
              let (lhs, rhs): (Lhs, Rhs) = _prod.clone();
              if lhs == production_without_dot.lhs && rhs == production_without_dot.rhs {
                if idx == 0 {
                  // apply R3
                  self.action_table.insert((*id, Symbol::End), Action::Accept);
                  continue;
                }

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
      },
    );

    // define all the reduce actions
    for key in self.action_table.clone().keys() {
      let action = self.action_table.get(key).unwrap();
      let (idx, _) = key;

      if let Action::Reduce(_action) = action {
        self.add_action(*idx, &Action::Reduce(*_action));
      }
    }

    // to define all errors
    for key in self.action_table.clone().keys() {
      let (idx, _) = key;
      self.add_action(*idx, &Action::Error);
    }

    self
  }

  fn add_action(&mut self, row: usize, action: &Action) {
    let terminals = self.grammar.terminals();

    for terminal in terminals {
      if self.action_table.get(&(row, terminal.clone())).is_none() {
        self
          .action_table
          .insert((row, terminal.clone()), action.clone());
      };
    }
  }

  #[allow(dead_code)]
  pub fn display_actions_table(&self) -> &Self {
    // Sort the action table by key considering the usize
    let mut keys: Vec<(usize, Symbol)> = self.action_table.keys().cloned().collect();

    keys.sort_by(|a, b| a.0.cmp(&b.0));

    keys.iter().for_each(|key| {
      let action = self.action_table.get(key).unwrap();
      println!("({}, {:?}) -> {:?}", key.0, key.1, action);
    });

    self
  }

  #[allow(dead_code)]
  pub fn display_goto_table(&self) -> &Self {
    // Sort the action table by key considering the usize
    let mut keys: Vec<(usize, Symbol)> = self.goto_table.keys().cloned().collect();

    keys.sort_by(|a, b| a.0.cmp(&b.0));

    keys.iter().for_each(|key| {
      let action = self.goto_table.get(key).unwrap();
      println!("({}, {:?}) -> {:?}", key.0, key.1, action);
    });

    self
  }
}
