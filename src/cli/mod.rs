use clap::{Parser, Subcommand};

pub mod compile;
pub mod run;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  Compile(compile::Compile),
  Run(run::Run),
}

pub struct PileCompiler;
