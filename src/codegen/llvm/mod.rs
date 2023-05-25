use std::{rc::Rc, sync::{Arc, Mutex}};

use inkwell::context::Context;

use crate::parser::parse::AstNode;

use self::{globals::stack::Stack, manager::LLVMManager};

use super::CodeGenerator;

pub mod builtins;
pub mod externs;
pub mod globals;
pub mod manager;
pub mod operations;
pub mod utils;

pub struct LLVMCodeGenerator {}

impl LLVMCodeGenerator {
  pub fn new() -> Self {
    Self {}
  }
}

impl Default for LLVMCodeGenerator {
  fn default() -> Self {
    Self::new()
  }
}

impl CodeGenerator for LLVMCodeGenerator {
  fn generate(&self, ast: AstNode) -> anyhow::Result<()> {
    let context = Arc::new(Mutex::new(Context::create()));

    LLVMManager::create(context);
    Stack::create(1024);

    Ok(())
  }
}
