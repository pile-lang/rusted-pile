use serde::{Deserialize, Serialize};

use crate::{
  lexer::tokens::{ArithmeticOperators, ComparisonOperators, StackOperators, Token},
  parser::parse::AstNode,
};
use std::{collections::HashMap, fs::File};

use super::CodeGenerator;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ByteCode {
  // Stack manipulation
  PushInt(i32),
  PushFloat(f32),
  PushStr(String),
  PushBool(bool),
  Pop,
  Dump,
  Dup,

  // Arithmetic
  Add,
  Sub,
  Mul,
  Div,
  Mod,

  // Comparison
  Eq,
  Neq,
  Lt,
  Gt,
  Leq,
  Geq,

  // Branching
  JumpIfNotTrue(usize),
  Jump(usize),

  // Ignore
  Ignore,
}

pub struct VMCodeGenerator {
  branching_blocks: Vec<usize>,
  instructions_count: usize,
  bytecode: Vec<ByteCode>,
}

impl VMCodeGenerator {
  pub fn new() -> Self {
    Self {
      branching_blocks: vec![],
      instructions_count: 0,
      bytecode: vec![],
    }
  }

  pub fn generate_two_children_code(
    &mut self,
    left: &AstNode,
    right: &AstNode,
    opcode: ByteCode,
  ) -> anyhow::Result<Vec<ByteCode>> {
    let mut bytecode = self.generate_byte_code(left)?;
    bytecode.append(&mut self.generate_byte_code(right)?);
    bytecode.push(opcode);
    self.instructions_count += 2;
    Ok(bytecode)
  }

  pub fn generate_byte_code(&mut self, ast: &AstNode) -> anyhow::Result<Vec<ByteCode>> {
    match ast.token.clone() {
      Token::Program => {
        for child in ast.children.iter() {
          self.instructions_count += 1;
          let mut bytecode = self.generate_byte_code(child)?;

          self.bytecode.append(&mut bytecode);
        }

        Ok(self.bytecode.clone())
      }
      Token::Integer(value) => Ok(vec![ByteCode::PushInt(value)]),
      Token::Float(value) => Ok(vec![ByteCode::PushFloat(value)]),
      Token::String(value) => Ok(vec![ByteCode::PushStr(value)]),
      Token::StackOps(operator) => match operator {
        StackOperators::Dump => Ok(vec![ByteCode::Dump]),
        StackOperators::Dup => Ok(vec![ByteCode::Dup]),
        _ => Err(anyhow::anyhow!(
          "Currently unsupported token: {:?}",
          ast.token
        )),
      },
      Token::ArithmeticOp(operator) => {
        use ArithmeticOperators::*;
        use ByteCode::*;

        if let Some(opcode) = vec![
          (Plus, Add),
          (Times, Mul),
          (Minus, Sub),
          (Divide, Div),
          (Modulo, Mod),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>()
        .get(&operator)
        {
          self.generate_two_children_code(&ast.children[0], &ast.children[1], opcode.clone())
        } else {
          Err(anyhow::anyhow!(
            "Currently unsupported token: {:?}",
            ast.token
          ))
        }
      }
      Token::ComparisonOp(operator) => {
        use ByteCode::*;
        use ComparisonOperators::*;

        if let Some(opcode) = vec![
          (EqualTo, Eq),
          (NotEqualTo, Neq),
          (LessThan, Lt),
          (GreaterThan, Gt),
          (LessThanOrEqualTo, Leq),
          (GreaterThanOrEqualTo, Geq),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>()
        .get(&operator)
        {
          self.generate_two_children_code(&ast.children[0], &ast.children[1], opcode.clone())
        } else {
          Err(anyhow::anyhow!(
            "Currently unsupported token: {:?}",
            ast.token
          ))
        }
      }
      Token::If => {
        let mut bytecode = vec![];
        let mut condition = self.generate_byte_code(&ast.children[0])?;

        self.branching_blocks.push(self.instructions_count);
        bytecode.append(&mut condition);
        bytecode.push(ByteCode::JumpIfNotTrue(usize::MAX)); // Placeholder position for now
        self.instructions_count += 1;

        Ok(bytecode)
      }
      Token::Else => {
        if let Some(if_intruction_location) = self.branching_blocks.pop() {
          if let ByteCode::JumpIfNotTrue(_) = &mut self.bytecode[if_intruction_location] {
            self.bytecode[if_intruction_location] =
              ByteCode::JumpIfNotTrue(self.instructions_count);
          }
        } else {
          return Err(anyhow::anyhow!("Mismatched 'else'"));
        }
        self.branching_blocks.push(self.instructions_count - 1);

        Ok(vec![ByteCode::Jump(usize::MAX)])
      }
      Token::End => {
        if let Some(branch_intruction_location) = self.branching_blocks.pop() {
          match &mut self.bytecode[branch_intruction_location] {
            ByteCode::JumpIfNotTrue(_) => {
              self.bytecode[branch_intruction_location] =
                ByteCode::JumpIfNotTrue(self.instructions_count);
            }
            ByteCode::Jump(_) => {
              self.bytecode[branch_intruction_location] = ByteCode::Jump(self.instructions_count);
            }
            _ => {
              return Err(anyhow::anyhow!("Mismatched 'end' (1)"));
            }
          }

          Ok(vec![ByteCode::Ignore])
        } else {
          Err(anyhow::anyhow!("Mismatched 'end' (2)"))
        }
      }
      _ => Err(anyhow::anyhow!(
        "Currently unsupported token: {:?}",
        ast.token
      )),
    }
  }

  pub fn encode_byte_code(bytecode: Vec<ByteCode>) -> anyhow::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(&bytecode).unwrap();

    use std::io::Write;

    let mut file =
      File::create("bytecode.bin").map_err(|e| anyhow::anyhow!("Error creating file: {}", e))?;
    file
      .write_all(&encoded)
      .map_err(|e| anyhow::anyhow!("Error writing to file: {}", e))?;

    Ok(())
  }
}

impl Default for VMCodeGenerator {
  fn default() -> Self {
    Self::new()
  }
}

impl CodeGenerator for VMCodeGenerator {
  fn generate(&mut self, ast: AstNode) -> anyhow::Result<()> {
    let mut generator = VMCodeGenerator::new();
    let bytecode = generator.generate_byte_code(&ast)?;
    println!("{:?}", bytecode);
    VMCodeGenerator::encode_byte_code(bytecode)?;

    Ok(())
  }
}
