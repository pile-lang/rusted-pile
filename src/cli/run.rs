use clap::Args;

use crate::interpreter::vm;
use miette::Result as MietteResult;

use super::PileCompiler;

#[derive(Args)]
pub struct Run {
  #[arg(required = true, short, long)]
  pub filename: String,
}

impl PileCompiler {
  pub fn run(Run { filename }: &Run) -> MietteResult<(), Box<dyn std::error::Error>> {
    vm::VMInterpreter::run(filename)?;

    Ok(())
  }
}
