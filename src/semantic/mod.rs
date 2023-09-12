use crate::{
  lexer::tokens::{ArithmeticOperators, StackOperators, Token},
  parser::parse::AstNode,
};
use miette::Result as MietteResult;

use self::{
  errors::SemanticError,
  stack_frame::StackFrame,
  symbol_table::{SymbolTable, Value},
};

pub mod ast;
pub mod errors;
pub mod stack_frame;
pub mod symbol_table;

pub struct SemanticAnalyzer {
  pub symbol_table: SymbolTable,
  stack: StackFrame,
  source_code: String,
}

impl SemanticAnalyzer {
  pub fn new(source_code: String) -> Self {
    Self {
      symbol_table: SymbolTable::new(source_code.to_string()),
      stack: Default::default(),
      source_code,
    }
  }

  pub fn analyze(&mut self, ast: &AstNode) -> MietteResult<()> {
    match ast.token.clone() {
      Token::Program => {
        for child in ast.children.iter() {
          self.analyze(child)?;
        }
      }
      Token::Integer(value) => {
        self.stack.values.push(Value::I32(value));
      }
      Token::Float(value) => self.stack.values.push(Value::F32(value)),
      Token::String(value) => self.stack.values.push(Value::String(value)),
      Token::Boolean(value) => self.stack.values.push(Value::Bool(value)),
      Token::StackOps(operator) => {
        match operator {
          StackOperators::Dump => self.stack_pop()?,
          StackOperators::Dup => self.stack_dup()?,
          StackOperators::Drop => self.stack_pop()?,
        };
      }
      Token::ArithmeticOp(operator) => {
        self.analyze(ast.children.get(0).unwrap()).unwrap(); // left side
        self.analyze(ast.children.get(1).unwrap()).unwrap(); // right side

        let left = self.stack_pop()?;
        let right = self.stack_pop()?;

        match (left, right) {
          (Value::I32(left), Value::I32(right)) => {
            self.stack.values.push(Value::I32(match operator {
              ArithmeticOperators::Plus => left + right,
              ArithmeticOperators::Minus => left - right,
              ArithmeticOperators::Times => left * right,
              ArithmeticOperators::Divide => left / right,
              ArithmeticOperators::Modulo => left % right,
            }))
          }
          (Value::F32(left), Value::F32(right)) => {
            self.stack.values.push(Value::F32(match operator {
              ArithmeticOperators::Plus => left + right,
              ArithmeticOperators::Minus => left - right,
              ArithmeticOperators::Times => left * right,
              ArithmeticOperators::Divide => left / right,
              ArithmeticOperators::Modulo => left % right,
            }))
          }
          _ => Err(SemanticError::OperatorsTypeDiffer {
            input: self.source_code.clone(),
            advice: "You can only add two values of the same type".to_string(),
            extension_src: ast.span,
          })?,
        }
      }
      _ => Err(SemanticError::Unimplemented {
        input: self.source_code.clone(),
        advice: "Semantic validation not implemented".to_string(),
        extension_src: ast.span,
      })?,
    }

    Ok(())
  }

  pub fn stack_dup(&mut self) -> MietteResult<Value> {
    match self.stack.values.clone().last() {
      Some(value) => {
        self.stack.values.push(value.clone());
        Ok(value.clone())
      }
      None => Err(SemanticError::EmptyStack {
        input: self.source_code.clone(),
        advice: "You can't dup an empty stack".to_string(),
      })?,
    }
  }

  pub fn stack_pop(&mut self) -> MietteResult<Value> {
    match self.stack.values.pop() {
      Some(value) => Ok(value),
      None => Err(SemanticError::EmptyStack {
        input: self.source_code.clone(),
        advice: "You can't pop an empty stack".to_string(),
      })?,
    }
  }
}
