use super::value::Value;

pub struct ComparisonInstruction;

pub enum ComparisonMethod {
  LessThan,
  LessThanEqual,
  GreaterThan,
  GreaterThanEqual,
  Equal,
  NotEqual,
}

impl ComparisonInstruction {
  fn push_into_stack<F: FnOnce(Value, Value) -> bool>(
    stack: &mut Vec<Value>,
    comparison: F,
  ) -> anyhow::Result<()> {
    let rhs = stack
      .pop()
      .ok_or(anyhow::anyhow!("Comparison on empty stack"))?;
    let lhs = stack
      .pop()
      .ok_or(anyhow::anyhow!("Comparison on empty stack"))?;

    let result = comparison(lhs, rhs);

    stack.push(Value::Bool(result));

    Ok(())
  }

  pub fn eval(stack: &mut Vec<Value>, instruction: ComparisonMethod) -> anyhow::Result<()> {
    match instruction {
      ComparisonMethod::LessThan => {
        Self::push_into_stack(stack, |lhs: Value, rhs: Value| (lhs < rhs))
      }
      ComparisonMethod::LessThanEqual => {
        Self::push_into_stack(stack, |lhs: Value, rhs: Value| (lhs <= rhs))
      }
      ComparisonMethod::GreaterThan => {
        Self::push_into_stack(stack, |lhs: Value, rhs: Value| (lhs > rhs))
      }
      ComparisonMethod::GreaterThanEqual => {
        Self::push_into_stack(stack, |lhs: Value, rhs: Value| (lhs >= rhs))
      }
      ComparisonMethod::Equal => {
        Self::push_into_stack(stack, |lhs: Value, rhs: Value| (lhs == rhs))
      }
      ComparisonMethod::NotEqual => {
        Self::push_into_stack(stack, |lhs: Value, rhs: Value| (lhs != rhs))
      }
    }
    .map_err(|err| anyhow::anyhow!(format!("Error while comparing: {}", err)))?;

    Ok(())
  }
}
