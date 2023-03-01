use self::{stack_frame::StackFrame, symbol_table::SymbolTable};

pub mod analyze;
pub mod errors;
pub mod stack_frame;
pub mod symbol_table;

pub struct SemanticAnalyzer {
  symbol_table: SymbolTable,
  stack: StackFrame,
  source_code: String,
}
