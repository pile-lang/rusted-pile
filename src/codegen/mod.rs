use crate::parser::parse::AstNode;

pub trait CodeGenerator {
  fn generate(&self, ast: AstNode) -> anyhow::Result<()>;
}

pub enum CodeGeneratorTarget {
  LLVM,
  Wasm,
}

pub mod llvm;
pub mod wasm;

// Choose the code generator based on the target
pub fn code_generator(target: CodeGeneratorTarget) -> Box<dyn CodeGenerator> {
  match target {
    CodeGeneratorTarget::LLVM => Box::<llvm::LLVMCodeGenerator>::default(),
    CodeGeneratorTarget::Wasm => Box::<wasm::WasmCodeGenerator>::default(),
  }
}
