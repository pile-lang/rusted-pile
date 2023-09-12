use super::symbol_table::Value;

// The stack frame will actually keep track of the values in the stack
// as the pile is a stack-based language
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StackFrame {
  pub values: Vec<Value>,
}
