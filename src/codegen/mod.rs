use crate::parser::parse::AstNode;

pub trait CodeGenerator {
  fn generate(&mut self, ast: AstNode, filename: String) -> anyhow::Result<()>;
}

pub enum CodeGeneratorTarget {
  LLVM,
  Wasm,
  VirtualMachine,
}

pub mod llvm;
pub mod vm;
pub mod wasm;

// Choose the code generator based on the target
pub fn code_generator(target: CodeGeneratorTarget) -> Box<dyn CodeGenerator> {
  match target {
    CodeGeneratorTarget::LLVM => Box::<llvm::LLVMCodeGenerator>::default(),
    CodeGeneratorTarget::Wasm => Box::<wasm::WasmCodeGenerator>::default(),
    CodeGeneratorTarget::VirtualMachine => Box::<vm::VMCodeGenerator>::default(),
  }
}
