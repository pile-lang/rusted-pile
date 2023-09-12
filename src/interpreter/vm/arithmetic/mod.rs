use super::value::Value;

pub struct ArithmeticInstruction;

pub enum ArithmeticMethod {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
}

impl ArithmeticInstruction {
  pub fn eval(stack: &mut Vec<Value>, instruction: ArithmeticMethod) -> anyhow::Result<()> {
    let rhs = stack.pop().ok_or(anyhow::anyhow!("Add on empty stack"))?;
    let lhs = stack.pop().ok_or(anyhow::anyhow!("Add on empty stack"))?;

    match (lhs.clone(), rhs.clone()) {
      (Value::Int(lhs), Value::Int(rhs)) => match instruction {
        ArithmeticMethod::Add => stack.push(Value::Int(lhs + rhs)),
        ArithmeticMethod::Sub => stack.push(Value::Int(lhs - rhs)),
        ArithmeticMethod::Mul => stack.push(Value::Int(lhs * rhs)),
        ArithmeticMethod::Div => {
          if rhs == 0 {
            return Err(anyhow::anyhow!("Divide by zero"));
          }
          stack.push(Value::Int(lhs / rhs))
        }
        ArithmeticMethod::Mod => {
          if rhs == 0 {
            return Err(anyhow::anyhow!("Modulo by zero"));
          }
          stack.push(Value::Int(lhs % rhs))
        }
      },
      _ => {
        return Err(anyhow::anyhow!(format!(
          "Add on non-integers: {} + {}",
          lhs, rhs
        )));
      }
    }

    Ok(())
  }
}
