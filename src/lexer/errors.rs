use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum LexerError {
  #[error(transparent)]
  #[diagnostic(code(file_read::io_error))]
  IoError(#[from] std::io::Error),

  #[error("Unsupported format")]
  #[diagnostic(code(lexer_error::unsupported_format))]
  UnsupportedFormat {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label("This bit here")]
    extension_src: (usize, usize),
  },
}
