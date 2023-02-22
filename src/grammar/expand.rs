use super::{parser::remove_angle_brackets, Grammar, Symbol};

impl Grammar {
  /// Expand the grammar by adding a new first production that leads to the
  /// start symbol.
  /// Example:
  /// ```
  /// S -> A | B
  /// ```
  /// becomes
  /// ```
  /// S' -> S
  /// S -> A | B
  /// ```
  pub fn expand(&mut self) -> &mut Self {
    let start_symbol = self.productions[0].0.clone();
    let new_start_symbol = Symbol::NonTerminal(format!(
      "{}'",
      remove_angle_brackets(&start_symbol.get_name())
    ));
    self
      .productions
      .insert(0, (new_start_symbol, vec![start_symbol]));

    self
  }
}
