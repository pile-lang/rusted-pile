use std::{fs::File, io::Read};

use crate::codegen::vm::ByteCode;

use self::{
  arithmetic::{ArithmeticInstruction, ArithmeticMethod},
  comparison::{ComparisonInstruction, ComparisonMethod},
  stack::{dump::DumpInstruction, dup::DupInstruction, pop::PopInstruction, push::PushInstruction},
  value::Value,
};

pub mod arithmetic;
pub mod comparison;
pub mod stack;
pub mod value;

pub struct VMInterpreter;

impl VMInterpreter {
  pub fn open(bytecode_file: &str) -> anyhow::Result<Vec<ByteCode>> {
    let mut file =
      File::open(bytecode_file).map_err(|e| anyhow::anyhow!("Error opening file: {}", e))?;
    let mut encoded = Vec::new();
    file
      .read_to_end(&mut encoded)
      .map_err(|e| anyhow::anyhow!("Error reading file: {}", e))?;

    bincode::deserialize(&encoded).map_err(|e| anyhow::anyhow!("Error deserializing: {}", e))
  }

  pub fn run(bytecode_file: &str) -> anyhow::Result<()> {
    let bytecode = VMInterpreter::open(bytecode_file)?;
    // println!("{:?}", bytecode);

    VM::new().execute(&bytecode)?;

    Ok(())
  }
}

pub struct VM {
  stack: Vec<Value>,
}

impl VM {
  /// Creates a new [`VM`].
  pub fn new() -> Self {
    Self { stack: vec![] }
  }

  pub fn execute(&mut self, bytecode: &[ByteCode]) -> anyhow::Result<()> {
    for instruction in bytecode {
      match instruction {
        // Stack
        ByteCode::PushInt(value) => PushInstruction::eval(&mut self.stack, *value)?,
        ByteCode::PushFloat(value) => PushInstruction::eval(&mut self.stack, *value)?,
        ByteCode::PushStr(value) => PushInstruction::eval(&mut self.stack, value.clone())?,
        ByteCode::PushBool(value) => PushInstruction::eval(&mut self.stack, *value)?,
        ByteCode::Dump => DumpInstruction::eval(&mut self.stack)?,
        ByteCode::Dup => DupInstruction::eval(&mut self.stack)?,
        ByteCode::Pop => PopInstruction::eval(&mut self.stack)?,

        // Arithmetic
        ByteCode::Add => ArithmeticInstruction::eval(&mut self.stack, ArithmeticMethod::Add)?,
        ByteCode::Sub => ArithmeticInstruction::eval(&mut self.stack, ArithmeticMethod::Sub)?,
        ByteCode::Mul => ArithmeticInstruction::eval(&mut self.stack, ArithmeticMethod::Mul)?,
        ByteCode::Div => ArithmeticInstruction::eval(&mut self.stack, ArithmeticMethod::Div)?,
        ByteCode::Mod => ArithmeticInstruction::eval(&mut self.stack, ArithmeticMethod::Mod)?,

        // Comparison
        ByteCode::Eq => ComparisonInstruction::eval(&mut self.stack, ComparisonMethod::Equal)?,
        ByteCode::Neq => ComparisonInstruction::eval(&mut self.stack, ComparisonMethod::NotEqual)?,
        ByteCode::Lt => ComparisonInstruction::eval(&mut self.stack, ComparisonMethod::LessThan)?,
        ByteCode::Leq => {
          ComparisonInstruction::eval(&mut self.stack, ComparisonMethod::LessThanEqual)?
        }
        ByteCode::Gt => {
          ComparisonInstruction::eval(&mut self.stack, ComparisonMethod::GreaterThan)?
        }
        ByteCode::Geq => {
          ComparisonInstruction::eval(&mut self.stack, ComparisonMethod::GreaterThanEqual)?
        }
      }
    }

    Ok(())
  }
}

impl Default for VM {
  fn default() -> Self {
    Self::new()
  }
}
