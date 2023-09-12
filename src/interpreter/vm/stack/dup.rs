use crate::interpreter::vm::value::Value;

pub struct DupInstruction;

impl DupInstruction {
  pub fn eval(stack: &mut Vec<Value>) -> anyhow::Result<()> {
    let top = stack.last().ok_or(anyhow::anyhow!("Dup on empty stack"))?;
    stack.push(top.clone());

    Ok(())
  }
}
