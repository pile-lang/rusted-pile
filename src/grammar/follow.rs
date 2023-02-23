use std::collections::{HashMap, HashSet};

use super::{Grammar, Symbol};

impl Grammar {
  pub fn compute_follow_set(&mut self) -> &mut Self {
    // 1. Add $ to the follow set of the start symbol
    self
      .follow_set
      .insert(self.start_symbol(), vec![Symbol::End].into_iter().collect());

    let mut last_follow_set = HashMap::new();
    while last_follow_set != self.follow_set {
      last_follow_set = self.follow_set.clone();

      // Compute follow set for each production
      for (lhs, _) in self.productions.clone() {
        self.follow_set_for_production(lhs.clone());
      }
    }

    self
  }

  pub fn follow_set_for_production(&mut self, lhs: Symbol) {
    // Filter out the lhs
    let binding = self.productions.clone();
    let all_productions_but_the_current = binding.iter().filter(|(_lhs, _)| lhs != *_lhs);

    for (lhs, rhs) in all_productions_but_the_current {
      for symbol in rhs.iter() {
        // If the symbol is not a non-terminal, skip it ✅
        if !symbol.is_non_terminal() {
          continue;
        }

        // Find the symbol next to the current symbol ✅
        let next_symbol = rhs.iter().skip_while(|s| *s != symbol).nth(1);

        // 1. If the symbol is the last symbol in the production, add the follow set of the lhs to the follow set of the symbol ✅
        if next_symbol.is_none() {
          // Add the follow set of the production to the follow set of the next_symbol
          let binding = self.follow_set.clone();
          let production_follow_set = binding.get(lhs);
          if production_follow_set.is_none() {
            continue;
          }

          self
            .follow_set
            .entry(symbol.clone())
            .or_insert(HashSet::new())
            .extend(production_follow_set.unwrap().clone());

          continue;
        }

        let next_symbol = next_symbol.unwrap();

        match next_symbol {
          Symbol::Terminal(_) => {
            // 2. If the next_symbol is a terminal, add it to the follow set of the symbol ✅
            self
              .follow_set
              .entry(symbol.clone())
              .or_insert(HashSet::new())
              .insert(next_symbol.clone());
          }
          Symbol::NonTerminal(_) => {
            // If the first set of the next symbol is not computed yet, then compute it
            if !self.first_set.contains_key(next_symbol) {
              self.first_set_for_symbol(next_symbol.clone());
            }

            // If the first set of the next symbol contains the empty symbol and the next symbol is the
            // last, then add the follow set of the production to the follow set of the symbol
            if self
              .first_set
              .get(next_symbol)
              .unwrap()
              .contains(&Symbol::Empty)
              && rhs.iter().last().unwrap() == symbol
            {
              // Add the follow set of the production to the follow set of the next_symbol
              self
                .follow_set
                .entry(symbol.clone())
                .or_insert(HashSet::new())
                .insert(next_symbol.clone());
            }

            // Add the first set of the next symbol to the follow set of the symbol, except the empty
            // symbol
            self
              .first_set
              .get(next_symbol)
              .unwrap()
              .iter()
              .filter(|s| **s != Symbol::Empty)
              .for_each(|s| {
                self
                  .follow_set
                  .entry(symbol.clone())
                  .or_insert(HashSet::new())
                  .insert(s.clone());
              });
          }
          _ => {
            panic!("Unexpected symbol: {:?}", next_symbol);
          }
        }
      }
    }
  }

  fn first_set_for_symbol(&mut self, lhs: Symbol) {
    // Filter out the lhs
    let binding = self.productions.clone();
    let all_productions_but_the_current = binding.iter().filter(|(_lhs, _)| lhs != *_lhs);

    for (lhs, rhs) in all_productions_but_the_current {
      for symbol in rhs.iter() {
        match symbol {
          Symbol::Terminal(_) | Symbol::Empty => {
            // 2. If the next symbol is a terminal, add it to the follow set of the symbol ✅
            self
              .first_set
              .entry(lhs.clone())
              .or_insert(HashSet::new())
              .insert(symbol.clone());
          }
          Symbol::NonTerminal(_) => {
            // If the first set of the production that produces the symbol is not computed yet, then
            // compute it
            if !self.first_set.contains_key(symbol) {
              self.first_set_for_symbol(lhs.clone());
            }

            let binding = self.first_set.clone();
            let first_set = binding.get(symbol).unwrap();

            // Add the first set of the symbol to the first set of the production
            first_set.iter().for_each(|s| {
              self
                .first_set
                .entry(lhs.clone())
                .or_insert(HashSet::new())
                .insert(s.clone());
            });

            // If the first set of the symbol contains the empty symbol, then continue to the next
            // symbol
            if first_set.contains(&Symbol::Empty) {
              continue;
            }

            // If the first set of the production that produces the symbol does not contain the empty
            // symbol, then stop
            break;
          }
          _ => {
            panic!("Unexpected symbol: {:?}", symbol);
          }
        }
      }
    }
  }
}
