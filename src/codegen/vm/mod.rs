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
}

pub struct VMCodeGenerator;

impl VMCodeGenerator {
  pub fn new() -> Self {
    Self {}
  }

  pub fn generate_two_children_code(
    left: &AstNode,
    right: &AstNode,
    opcode: ByteCode,
  ) -> anyhow::Result<Vec<ByteCode>> {
    let mut bytecode = VMCodeGenerator::generate_byte_code(left)?;
    bytecode.append(&mut VMCodeGenerator::generate_byte_code(right)?);
    bytecode.push(opcode);
    Ok(bytecode)
  }

  pub fn generate_byte_code(ast: &AstNode) -> anyhow::Result<Vec<ByteCode>> {
    match ast.token.clone() {
      Token::Program => {
        let mut bytecode = vec![];

        for child in ast.children.iter() {
          bytecode.append(&mut VMCodeGenerator::generate_byte_code(child)?);
        }

        Ok(bytecode)
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
          Self::generate_two_children_code(&ast.children[0], &ast.children[1], opcode.clone())
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
          Self::generate_two_children_code(&ast.children[0], &ast.children[1], opcode.clone())
        } else {
          Err(anyhow::anyhow!(
            "Currently unsupported token: {:?}",
            ast.token
          ))
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
    let bytecode = VMCodeGenerator::generate_byte_code(&ast)?;
    println!("{:?}", bytecode);
    VMCodeGenerator::encode_byte_code(bytecode)?;

    Ok(())
  }
}
