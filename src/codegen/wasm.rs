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
  fn generate(&self, ast: AstNode) -> anyhow::Result<()> {
    Ok(())
  }
}
