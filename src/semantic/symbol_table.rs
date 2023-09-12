use std::{collections::HashMap, fmt::Display};

use crate::lexer::tokens::span_to_tuple;

use super::errors::SemanticError;

/// The Semantic analysis of the language must keep track of the current
/// stack with the values, types and scopes.
/// As well as a table of symbols and their types.

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  I32(i32),
  I64(i64),
  F32(f32),
  F64(f64),
  Bool(bool),
  String(String),
}

impl From<&str> for Value {
  fn from(type_name: &str) -> Self {
    match type_name {
      "i32" => Value::I32(0),
      "i64" => Value::I64(0_i64),
      "f32" => Value::F32(0_f32),
      "f64" => Value::F64(0_f64),
      "bool" => Value::Bool(false),
      "string" => Value::String("".to_string()),
      _ => Value::I32(0),
    }
  }
}

impl From<(&str, &str)> for Value {
  fn from((value, type_name): (&str, &str)) -> Self {
    match type_name {
      "i32" => Value::I32(value.parse().unwrap()),
      "i64" => Value::I64(value.parse().unwrap()),
      "f32" => Value::F32(value.parse().unwrap()),
      "f64" => Value::F64(value.parse().unwrap()),
      "bool" => Value::Bool(value.parse().unwrap()),
      "string" => Value::String(value.to_string()),
      _ => Value::I32(0),
    }
  }
}

impl Default for Value {
  fn default() -> Self {
    Value::I32(0)
  }
}

impl Value {
  pub fn compare_type_to(&self, other: &Value) -> bool {
    matches!(
      (self, other),
      (Value::I32(_), Value::I32(_))
        | (Value::I64(_), Value::I64(_))
        | (Value::F32(_), Value::F32(_))
        | (Value::F64(_), Value::F64(_))
        | (Value::Bool(_), Value::Bool(_))
        | (Value::String(_), Value::String(_))
    )
  }

  pub fn get_type(&self) -> String {
    match self {
      Value::I32(_) => "i32",
      Value::I64(_) => "i64",
      Value::F32(_) => "f32",
      Value::F64(_) => "f64",
      Value::Bool(_) => "bool",
      Value::String(_) => "string",
    }
    .to_string()
  }

  pub fn cast_to(&self, type_name: &str) -> Result<Value, String> {
    match (type_name, self) {
      ("i32", Value::I32(value)) => Ok(Value::I32(*value)),
      ("i32", Value::F32(value)) => Ok(Value::I32(*value as i32)),
      _ => Err(format!(
        "Cannot cast value {:?} to type {}",
        self, type_name
      )),
    }
  }

  pub fn times(&self, other: &Value) -> Result<Value, ()> {
    match (self, other) {
      (Value::I32(value), Value::I32(other)) => Ok(Value::I32(value * other)),
      (Value::I64(value), Value::I64(other)) => Ok(Value::I64(value * other)),
      (Value::F32(value), Value::F32(other)) => Ok(Value::F32(value * other)),
      (Value::F64(value), Value::F64(other)) => Ok(Value::F64(value * other)),
      _ => Err(()),
    }
  }

  pub fn plus(&self, other: &Value) -> Result<Value, ()> {
    match (self, other) {
      (Value::I32(value), Value::I32(other)) => Ok(Value::I32(value + other)),
      (Value::I64(value), Value::I64(other)) => Ok(Value::I64(value + other)),
      (Value::F32(value), Value::F32(other)) => Ok(Value::F32(value + other)),
      (Value::F64(value), Value::F64(other)) => Ok(Value::F64(value + other)),
      (Value::String(value), Value::String(other)) => {
        Ok(Value::String(format!("{}{}", value, other)))
      }
      _ => Err(()),
    }
  }

  pub fn minus(&self, other: &Value) -> Result<Value, ()> {
    match (self, other) {
      (Value::I32(value), Value::I32(other)) => Ok(Value::I32(value - other)),
      (Value::I64(value), Value::I64(other)) => Ok(Value::I64(value - other)),
      (Value::F32(value), Value::F32(other)) => Ok(Value::F32(value - other)),
      (Value::F64(value), Value::F64(other)) => Ok(Value::F64(value - other)),
      _ => Err(()),
    }
  }

  pub fn get_value(&self) -> String {
    match self {
      Value::I32(value) => value.to_string(),
      Value::I64(value) => value.to_string(),
      Value::F32(value) => value.to_string(),
      Value::F64(value) => value.to_string(),
      Value::Bool(value) => value.to_string(),
      Value::String(value) => value.to_string(),
    }
  }
}

/// The scope is an enum that can be either a global scope or a local scope
/// The local scope is a vector of strings that contains the names of the
/// variables in the scope
#[derive(Default, Debug, Clone, PartialEq)]
pub enum Scope {
  #[default]
  Global,
  Local(usize),
}

/// Symbol information
/// The symbol information is stored in a struct
/// The symbol name, the type, the scope, the value and the position
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Symbol {
  pub name: String,
  pub scope: Scope,
  pub value: Value,
  pub position: (u32, u32),
}

/// Symbol table
/// Use a hash map to store the symbols and their informations
/// The key is the symbol name and the value is the symbol information
#[derive(Default, Debug)]
pub struct SymbolTable {
  pub symbols: HashMap<(String, usize), Symbol>,
  pub current_scope: usize,
  pub source: String,
}

impl From<usize> for Scope {
  fn from(scope: usize) -> Self {
    match scope {
      0 => Scope::Global,
      _ => Scope::Local(scope),
    }
  }
}

impl Display for SymbolTable {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (key, value) in &self.symbols {
      writeln!(
        f,
        "Symbol: {:?} - {:?} - {:?} - {:?} - {:?}",
        key, value.name, value.scope, value.value, value.position
      )?;
    }
    Ok(())
  }
}

impl SymbolTable {
  /// Create a new symbol table
  pub fn new(source: String) -> Self {
    SymbolTable {
      symbols: HashMap::new(),
      current_scope: 0,
      source,
    }
  }

  pub fn enter_scope(&mut self) {
    self.current_scope += 1;
  }

  pub fn exit_scope(&mut self) {
    self.current_scope -= 1;
  }

  pub fn define(
    &mut self,
    name: &str,
    value: Value,
    (row, col): (usize, usize),
  ) -> Result<Option<Symbol>, SemanticError> {
    if let Some(symbol) = self.symbols.get(&(name.to_string(), self.current_scope)) {
      return Err(SemanticError::DupplicateVariable {
        input: self.source.clone(),
        advice: "This variable was already declared!".to_string(),
        extension_src: span_to_tuple(row..col),
        first_extension_src: span_to_tuple(symbol.position.0 as usize..symbol.position.1 as usize),
      });
    }

    Ok(self.symbols.insert(
      (name.to_string(), self.current_scope),
      Symbol {
        name: name.to_string(),
        scope: self.current_scope.into(),
        value,
        position: (row as u32, col as u32),
      },
    ))
  }

  fn symbol_at_scope(&self, name: &str, scope: usize) -> Option<Symbol> {
    self.symbols.get(&(name.to_string(), scope)).cloned()
  }

  pub fn lookup(&self, name: &str) -> Option<Symbol> {
    for scope in (0..=self.current_scope).rev() {
      match self.symbol_at_scope(name, scope) {
        Some(symbol) => return Some(symbol),
        None => continue,
      }
    }
    None
  }

  pub fn update_variable(&mut self, name: &str, value: Value) -> Result<(), String> {
    self
      .symbols
      .get_mut(&(name.to_string(), self.current_scope))
      .map(|symbol| {
        symbol.value = value;
        Ok(())
      })
      .unwrap_or(Err(format!("Variable {} is not yet declared", name)))
  }
}

#[cfg(test)]
mod semantic_tests {
  use super::*;

  #[test]
  fn test_global_scope() -> Result<(), Box<dyn std::error::Error>> {
    let mut symbol_table = SymbolTable::new(String::from("a"));

    symbol_table.define("a", Value::I32(1), (0, 0));

    let var_a = symbol_table.lookup("a").expect("Variable a not found");

    assert_eq!(var_a.scope, Scope::Global);

    Ok(())
  }

  #[test]
  fn test_local_scope() -> Result<(), Box<dyn std::error::Error>> {
    let mut symbol_table = SymbolTable::new(String::from("a b"));

    symbol_table.define("a", Value::I32(1), (0, 0));
    symbol_table.enter_scope();
    symbol_table.define("b", Value::I32(2), (2, 0));

    let var_a = symbol_table.lookup("a").expect("Variable a not found");
    let var_b = symbol_table.lookup("b").expect("Variable b not found");

    assert_eq!(var_a.scope, Scope::Global);
    assert_eq!(var_b.scope, Scope::Local(1));

    Ok(())
  }

  #[test]
  fn test_local_scope_exit() -> Result<(), Box<dyn std::error::Error>> {
    let mut symbol_table = SymbolTable::new(String::from("a b"));

    symbol_table.define("a", Value::I32(1), (0, 0));
    symbol_table.enter_scope();
    symbol_table.define("b", Value::I32(2), (2, 0));
    symbol_table.exit_scope();

    let var_a = symbol_table.lookup("a").expect("Variable a not found");
    let var_b = symbol_table.lookup("b");

    assert_eq!(var_a.scope, Scope::Global);
    assert_eq!(var_b, None);

    Ok(())
  }

  #[test]
  fn test_local_preference() -> Result<(), Box<dyn std::error::Error>> {
    let mut symbol_table = SymbolTable::new(String::from("a a"));

    symbol_table.define("a", Value::I32(1), (0, 0));
    symbol_table.enter_scope();
    symbol_table.define("a", Value::I32(2), (2, 0));

    let var_a = symbol_table.lookup("a").expect("Variable a not found");

    assert_eq!(var_a.scope, Scope::Local(1));
    assert_eq!(var_a.value, Value::I32(2));

    symbol_table.exit_scope();

    let var_a = symbol_table.lookup("a").expect("Variable a not found");

    assert_eq!(var_a.scope, Scope::Global);
    assert_eq!(var_a.value, Value::I32(1));

    Ok(())
  }
}
