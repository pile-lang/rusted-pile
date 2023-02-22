use std::{
  cell::RefCell,
  collections::{HashMap, VecDeque},
  fmt::Display,
  hash::Hash,
  rc::Rc,
};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Production {
  lhs: Symbol,
  rhs: Vec<Symbol>,
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

    let mut new_rhs = rhs.clone();
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

type Rhs = Vec<Symbol>;
type Lhs = Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureItem {
  id: usize,
  kernel: Vec<Production>,
  productions: Vec<Production>,
  transitions: HashMap<Symbol, usize>,
}

impl PartialOrd for ClosureItem {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for ClosureItem {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.id.cmp(&other.id)
  }
}

impl Hash for ClosureItem {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.kernel.hash(state);
  }
}

impl Display for ClosureItem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "ClosureItem [{}] {{", self.id)?;

    writeln!(f, "\tProductions:")?;
    for production in &self.productions {
      let (lhs, rhs): (Lhs, Rhs) = production.clone().into();
      write!(f, "\t\t{} -> ", lhs)?;

      for symbol in rhs {
        write!(f, "{} ", symbol)?;
      }
      writeln!(f)?;
    }

    writeln!(f, "\tTransitions:")?;

    for (symbol, state) in &self.transitions {
      writeln!(f, "\t\t{} -> {}", symbol, state)?;
    }

    write!(f, "\n}}")?;

    Ok(())
  }
}

impl ClosureItem {
  pub fn new(kernel: Vec<Production>, grammar: &Grammar, id: usize) -> ClosureItem {
    let productions = kernel.clone();

    let mut closure = ClosureItem {
      id,
      kernel,
      productions,
      transitions: HashMap::new(),
    };

    closure.expand(grammar).clone()
  }

  pub fn new_long_lived(
    kernel: Vec<Production>,
    grammar: &Grammar,
    id: usize,
  ) -> Rc<RefCell<ClosureItem>> {
    let productions = kernel.clone();

    let closure = Rc::new(RefCell::new(ClosureItem {
      id,
      kernel,
      productions,
      transitions: HashMap::new(),
    }));

    closure.borrow_mut().expand(grammar);

    closure
  }

  pub fn expand(&mut self, grammar: &Grammar) -> &Self {
    let mut new_productions = Vec::new();
    let mut added_new_productions = true;

    while added_new_productions {
      added_new_productions = false;

      for production in &self.productions {
        let next_symbol = production.next_symbol_after_dot();

        if let Some(next_symbol) = next_symbol {
          if let Some(productions) = grammar.get_production(&next_symbol) {
            for production in productions {
              let new_production: Production = production.into();
              let new_production_with_dot = new_production.add_dot();

              if !self.productions.contains(&new_production_with_dot)
                && !new_productions.contains(&new_production_with_dot)
              {
                new_productions.push(new_production_with_dot);
                added_new_productions = true;
              }
            }
          }
        }
      }

      self.productions.append(&mut new_productions);
    }

    self
  }

  pub fn populate(
    &mut self,
    grammar: &Grammar,
  ) -> HashMap<Vec<Production>, Rc<RefCell<ClosureItem>>> {
    let mut closure_set: HashMap<Vec<Production>, Rc<RefCell<ClosureItem>>> = HashMap::new();

    // First add the first closure set
    let self_ref = Rc::new(RefCell::new(self.clone()));

    closure_set.insert(self.kernel.clone(), self_ref.clone());

    // Iterate over the closure set and expand each closure item
    let mut queue: VecDeque<Rc<RefCell<ClosureItem>>> = VecDeque::new();

    queue.push_back(self_ref);

    let mut counter = 1;
    while !queue.is_empty() {
      let closure_item = queue.pop_front().unwrap();
      let mut closure_item = closure_item.borrow_mut();

      for production in &closure_item.clone().productions {
        let next_symbol = production.next_symbol_after_dot();

        if next_symbol.is_none() {
          continue;
        }

        let next_symbol = next_symbol.unwrap();

        match next_symbol.clone() {
          Symbol::Empty => continue,
          Symbol::Dot => {
            panic!("There should not be a dot followed by a dot");
          }
          _ => {}
        }

        let kernel = &closure_item
          .find_goto_productions(next_symbol.clone())
          .into_iter()
          .map(|p| p.forward_dot())
          .collect::<Vec<Production>>();

        let current_id = closure_item.id;

        let closure_id = match closure_set.get(kernel) {
          Some(closure_item) => match closure_item.try_borrow() {
            Ok(closure_item) => closure_item.id,
            Err(_) => current_id,
          },
          None => {
            let closure_item = ClosureItem::new_long_lived(kernel.clone(), grammar, counter);
            counter += 1;

            queue.push_back(closure_item.clone());
            closure_set.insert(kernel.clone(), closure_item);

            counter - 1
          }
        };

        closure_item.transitions.insert(next_symbol, closure_id);
      }
    }

    closure_set
  }

  /// Find the goto productions for a given symbol
  /// It's all of the productions where the symbol is preceded by a dot
  pub fn find_goto_productions(&self, symbol: Symbol) -> Vec<Production> {
    let mut goto_productions = Vec::new();

    for production in &self.productions {
      let next_symbol = production.next_symbol_after_dot();

      if let Some(next_symbol) = next_symbol {
        if next_symbol == symbol {
          goto_productions.push(production.clone());
        }
      }
    }

    goto_productions
  }
}

pub struct SLR {
  grammar: Grammar,
  action_table: HashMap<(usize, Symbol), Action>,
  goto_table: HashMap<(usize, Symbol), usize>,
  closure_set: HashMap<Vec<Production>, Rc<RefCell<ClosureItem>>>,
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
    // let mut closure_items = Vec::new();
    // slr.closure_set.iter().for_each(|(_, closure_item)| {
    //   closure_items.push(closure_item.clone()); });
    //
    // closure_items.sort();

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
