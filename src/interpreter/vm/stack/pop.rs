use crate::interpreter::vm::value::Value;

pub struct PopInstruction;

impl PopInstruction {
  pub fn eval(stack: &mut Vec<Value>) -> anyhow::Result<()> {
    stack.pop().ok_or(anyhow::anyhow!("Pop on empty stack"))?;
    Ok(())
  }
}
