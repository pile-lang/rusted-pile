#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Int(i32),
  Float32(f32),
  Float64(f64),
  Bool(bool),
  Str(String),
}

impl PartialOrd for Value {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (Value::Int(lhs), Value::Int(rhs)) => lhs.partial_cmp(rhs),
      (Value::Float32(lhs), Value::Float32(rhs)) => lhs.partial_cmp(rhs),
      (Value::Float64(lhs), Value::Float64(rhs)) => lhs.partial_cmp(rhs),
      (Value::Str(lhs), Value::Str(rhs)) => lhs.partial_cmp(rhs),
      (Value::Bool(lhs), Value::Bool(rhs)) => lhs.partial_cmp(rhs),
      _ => None,
    }
  }
}

impl From<i32> for Value {
  fn from(value: i32) -> Self {
    Self::Int(value)
  }
}

impl From<f32> for Value {
  fn from(value: f32) -> Self {
    Self::Float32(value)
  }
}

impl From<f64> for Value {
  fn from(value: f64) -> Self {
    Self::Float64(value)
  }
}

impl From<bool> for Value {
  fn from(value: bool) -> Self {
    Self::Bool(value)
  }
}

impl From<String> for Value {
  fn from(value: String) -> Self {
    Self::Str(value)
  }
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Int(value) => write!(f, "{}", value),
      Value::Float32(value) => write!(f, "{}", value),
      Value::Float64(value) => write!(f, "{}", value),
      Value::Bool(value) => write!(f, "{}", value),
      Value::Str(value) => write!(f, "{}", value),
    }
  }
}
