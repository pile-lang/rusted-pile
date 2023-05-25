use inkwell::values::{BasicValueEnum, FunctionValue, IntValue, PointerValue};

pub trait ParamCast {
  fn cast(value: BasicValueEnum) -> Option<Self>
  where
    Self: Sized;
}

impl ParamCast for PointerValue<'_> {
  fn cast(value: BasicValueEnum) -> Option<Self> {
    if let BasicValueEnum::PointerValue(ptr) = value {
      Some(ptr)
    } else {
      None
    }
  }
}

impl ParamCast for IntValue<'_> {
  fn cast(value: BasicValueEnum) -> Option<Self> {
    if let BasicValueEnum::IntValue(int) = value {
      Some(int)
    } else {
      None
    }
  }
}

pub trait FunctionParams {
  fn get_param<T: ParamCast>(&self, nth: u32) -> Option<T>;
}

impl FunctionParams for FunctionValue<'_> {
  fn get_param<T: ParamCast>(&self, nth: u32) -> Option<T> {
    self.get_nth_param(nth).and_then(T::cast)
  }
}
