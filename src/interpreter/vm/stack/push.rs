use crate::interpreter::vm::value::Value;

pub struct PushInstruction;

impl PushInstruction {
  pub fn eval<T: Into<Value>>(stack: &mut Vec<Value>, value: T) -> anyhow::Result<()> {
    stack.push(value.into());
    Ok(())
  }
}
