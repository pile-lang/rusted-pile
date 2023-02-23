use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
  #[error("Unexpected token")]
  #[diagnostic(code(parse_error::unexpected_token))]
  UnexpectedToken {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label("Unexpected token")]
    extension_src: (usize, usize),
  },
}
