use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum SemanticError {
  #[error(transparent)]
  #[diagnostic(code(file_read::io_error))]
  StringError(#[from] std::io::Error),

  /// Stack Errors
  #[error("Unimplemented")]
  #[diagnostic(code(semantic_error::variable_type_mismatch))]
  Unimplemented {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label = "Here"]
    extension_src: (usize, usize),
  },

  #[error("Empty Stack")]
  #[diagnostic(code(semantic_error::empty_stack))]
  EmptyStack {
    #[source_code]
    input: String,

    #[help]
    advice: String,
  },

  #[error("Invalid Cast")]
  #[diagnostic(code(semantic_error::invalid_cast))]
  InvalidCast {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label = "Here"]
    extension_src: (usize, usize),
  },

  /// Variable errors

  #[error("Duplicate Variable")]
  #[diagnostic(code(semantic_error::duplicate_variable))]
  DupplicateVariable {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label = "First declared here"]
    first_extension_src: (usize, usize),

    #[label = "Trying to declare again here"]
    extension_src: (usize, usize),
  },

  #[error("Variable not declared")]
  #[diagnostic(code(semantic_error::variable_not_declared))]
  VariableNotDeclared {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label = "Here"]
    extension_src: (usize, usize),
  },

  #[error("Invalid Assignment")]
  #[diagnostic(code(semantic_error::variable_type_mismatch))]
  VariableTypeMismatch {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label = "Here"]
    extension_src: (usize, usize),
  },

  #[error("Operators type differ")]
  #[diagnostic(code(semantic_error::operators_type_differ))]
  OperatorsTypeDiffer {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label = "Here"]
    extension_src: (usize, usize),
  },

  #[error("Invalid Operator")]
  #[diagnostic(code(semantic_error::invalid_operator))]
  InvalidOperator {
    #[source_code]
    input: String,

    #[help]
    advice: String,

    #[label = "Here"]
    extension_src: (usize, usize),
  },
}
