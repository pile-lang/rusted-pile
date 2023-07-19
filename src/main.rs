use clap::Parser;
use miette::Result as MietteResult;
use rusted_pile::cli::{Cli, Commands, PileCompiler};

fn main() -> MietteResult<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Compile(opts) => PileCompiler::compile(opts)?,
    Commands::Run(opts) => PileCompiler::run(opts)?,
  }

  Ok(())
}
