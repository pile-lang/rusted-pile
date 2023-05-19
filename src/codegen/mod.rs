pub trait CodeGenerator {
  fn generate(&self) -> anyhow::Result<()>;
}

pub mod wasm;

// Choose the code generator based on the target
pub fn choose_code_generator(target: &str) -> Box<dyn CodeGenerator> {
  match target {
    "wasm" => Box::new(wasm::WasmCodeGenerator {}),
    _ => panic!("Unknown target: {}", target),
  }
}
