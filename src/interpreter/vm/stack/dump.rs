use crate::interpreter::vm::value::Value;

pub struct DumpInstruction;

impl DumpInstruction {
  pub fn eval(stack: &mut Vec<Value>) -> anyhow::Result<()> {
    // take the top of the stack and print it
    let top = stack.pop().ok_or(anyhow::anyhow!("Dump on empty stack"))?;

    println!("{}", top);

    Ok(())
  }
}
