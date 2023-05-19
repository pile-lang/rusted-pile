// CodeGenerator

use super::CodeGenerator;

pub struct WasmCodeGenerator {}

impl CodeGenerator for WasmCodeGenerator {
  fn generate(&self) -> anyhow::Result<()> {
    Ok(())
  }
}
