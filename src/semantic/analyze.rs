use std::ops::Range;

use crate::lexer::{
  tokens::{span_to_tuple, Token},
  PileToken,
};
use miette::Result as MietteResult;

use super::{
  errors::SemanticError,
  symbol_table::{Symbol, SymbolTable, Value},
  SemanticAnalyzer,
};

#[allow(clippy::result_unit_err)]
impl SemanticAnalyzer {
  pub fn new(source_code: &str) -> Self {
    SemanticAnalyzer {
      symbol_table: SymbolTable::new(source_code.to_string()),
      stack: Default::default(),
      source_code: source_code.to_string(),
    }
  }

  pub fn stack_pop(&mut self) -> MietteResult<Value> {
    match self.stack.values.pop() {
      Some(value) => Ok(value),
      None => Err(SemanticError::EmptyStack {
        input: "blabla".to_string(),
        advice: "You can't pop an empty stack".to_string(),
      })?,
    }
  }

  pub fn get_variable(&mut self, variable: &str, span: Range<usize>) -> MietteResult<Symbol> {
    match self.symbol_table.lookup(variable) {
      Some(variable) => Ok(variable),
      None => Err(SemanticError::VariableNotDeclared {
        input: self.source_code.to_string(),
        advice: "The variable is not declared in the current scope".to_string(),
        extension_src: span_to_tuple(span),
      })?,
    }
  }

  pub fn analyze(&mut self, tokens: Vec<PileToken>) -> MietteResult<()> {
    // Iterate over the tokens
    let mut tokens = tokens.into_iter();
    while let Some(PileToken { token, slice, span }) = tokens.next() {
      match token {
        Token::Integer(value) => self.stack.values.push(Value::I32(value)),
        Token::Float(value) => self.stack.values.push(Value::F32(value)),
        Token::Boolean(value) => self.stack.values.push(Value::Bool(value)),
        Token::CastOp => {
          // Get the next token
          let PileToken {
            token,
            span: type_span,
            ..
          } = tokens.next().unwrap();

          if let Token::Types(cast_type) = token {
            match self.stack_pop()?.cast_to(cast_type) {
              Ok(value) => self.stack.values.push(value),
              Err(error) => Err(SemanticError::InvalidCast {
                input: self.source_code.to_string(),
                advice: error.to_string(),
                extension_src: span_to_tuple(span.start..type_span.end),
              })?,
            }
          }
        }
        Token::DefType(definition_type) => {
          // Get the next token
          let PileToken { token, span, slice } = tokens.next().unwrap();

          // Check if the next token is an identifier
          if let Token::Identifier = token {
            // Define the variable in the symbol table
            self.symbol_table.define(
              &slice.to_string(),
              definition_type.into(),
              (span.start, span.end),
            )?;
          }
        }
        Token::AtSign => {
          // Get the next token
          let PileToken { token, span, slice } = tokens.next().unwrap();

          // Check if the next token is an identifier
          if let Token::Identifier = token {
            let stack = self.stack_pop()?;
            let identifier = self.get_variable(&slice.to_string(), span.clone())?;

            if identifier.value.compare_type_to(&stack) {
              self
                .symbol_table
                .update_variable(&slice.to_string(), stack)
                .expect("Variable has to be defined, previous check");
            } else {
              Err(SemanticError::VariableTypeMismatch {
                input: self.source_code.to_string(),
                advice:
                  "The type of the variable and the type of the value on the stack don't match"
                    .to_string(),
                extension_src: span_to_tuple(span),
              })?;
            }
          }
        }
        Token::Identifier => {
          // Check if the variable is defined and in the current scope
          let symbol = self.get_variable(&slice.to_string(), span)?;
          self.stack.values.push(symbol.value);
        }
        Token::Do | Token::If => {
          self.symbol_table.enter_scope();
        }
        Token::Else => {
          self.symbol_table.exit_scope();
          self.symbol_table.enter_scope();
        }
        Token::End => {
          self.symbol_table.exit_scope();
        }
        Token::ArithmeticOp => {
          let right = self.stack_pop()?;
          let left = self.stack_pop()?;

          match slice.as_str() {
            "+" | "-" | "*" => {
              // Left and right type must be equal
              if left.compare_type_to(&right) {
                match slice.as_str() {
                  "+" => {
                    let operation = left.plus(&right);
                    match operation {
                      Ok(value) => self.stack.values.push(value),
                      Err(_) => Err(SemanticError::InvalidOperator {
                        input: self.source_code.to_string(),
                        advice: format!(
                          "The operation {} + {} is not valid",
                          left.get_type(),
                          right.get_type()
                        ),
                        extension_src: span_to_tuple(span),
                      })?,
                    }
                  }
                  "-" => self.stack.values.push(left.minus(&right).unwrap()),
                  "*" => self.stack.values.push(left.times(&right).unwrap()),
                  _ => unreachable!(),
                }
              } else {
                Err(SemanticError::OperatorsTypeDiffer {
                  input: self.source_code.to_string(),
                  advice: format!(
                    "The types of the left and right operands differ: {} and {}",
                    left.get_type(),
                    right.get_type()
                  ),
                  extension_src: span_to_tuple(span),
                })?
              }
            }
            "/" => {}
            "%" => {}
            _ => Err(SemanticError::InvalidOperator {
              input: self.source_code.to_string(),
              advice: "The operator is not valid".to_string(),
              extension_src: span_to_tuple(span),
            })?,
          };
        }
        _ => {}
      }
    }

    println!("{}", self.symbol_table);
    Ok(())
  }
}

impl Default for SemanticAnalyzer {
  fn default() -> Self {
    Self::new("")
  }
}
