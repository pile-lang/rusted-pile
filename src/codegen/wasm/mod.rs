use crate::parser::parse::AstNode;

use super::CodeGenerator;

pub struct WasmCodeGenerator {}

impl WasmCodeGenerator {
  pub fn new() -> Self {
    Self {}
  }
}

impl Default for WasmCodeGenerator {
  fn default() -> Self {
    Self::new()
  }
}

impl CodeGenerator for WasmCodeGenerator {
  fn generate(&mut self, _ast: AstNode) -> anyhow::Result<()> {
    Ok(())
  }
}
