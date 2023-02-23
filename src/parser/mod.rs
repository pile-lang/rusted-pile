#[allow(non_snake_case)]
pub mod SLR;
pub mod closure;
pub mod parsing_table;
pub mod parse;
pub mod errors;

#[derive(Debug, Clone)]
pub enum Action {
  Shift(usize),  // shift to the state with the given index
  Reduce(usize), // reduce using the production with the given index
  Accept,        // accept the input
  Error,         // error
}
