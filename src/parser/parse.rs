use miette::Result as MietteResult;
use std::fmt;
use std::{collections::VecDeque, fmt::Display};

use crate::{
  grammar::Symbol,
  lexer::{
    tokens::{span_to_tuple, Token},
    PileToken,
  },
  parser::{errors::ParseError, Action},
};

use super::SLR::SLR;

#[derive(Debug)]
pub enum ParseTreeNode {
  Terminal(Token),
  NonTerminal(Symbol, Vec<ParseTreeNode>),
}

impl fmt::Display for ParseTreeNode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "ParseTreeNode")?;
    write_node(f, self, "", true)
  }
}

fn write_node(
  f: &mut fmt::Formatter,
  node: &ParseTreeNode,
  prefix: &str,
  is_last: bool,
) -> fmt::Result {
  let symbol = match node {
    ParseTreeNode::Terminal(token) => format!("\x1B[1m{}\x1B[0m", token),
    ParseTreeNode::NonTerminal(symbol, _) => symbol.to_string(),
  };
  let (node_prefix, child_prefix) = if is_last {
    ("\x1B[33m└─\x1B[0m", "  ")
  } else {
    ("\x1B[33m├─\x1B[0m", "\x1B[33m│ \x1B[0m")
  };
  writeln!(f, "{}{}{}", prefix, node_prefix, symbol)?;
  let child_count = match node {
    ParseTreeNode::Terminal(_) => 0,
    ParseTreeNode::NonTerminal(_, children) => children.len(),
  };
  for (i, child) in node_children(node).iter().enumerate() {
    let child_prefix = format!("{}{}", prefix, child_prefix);
    let is_last = i == child_count - 1;
    write_node(f, child, &child_prefix, is_last)?;
  }
  Ok(())
}

fn node_children(node: &ParseTreeNode) -> Vec<&ParseTreeNode> {
  match node {
    ParseTreeNode::Terminal(_) => vec![],
    ParseTreeNode::NonTerminal(_, children) => children.iter().collect(),
  }
}

impl SLR {
  pub fn parse(&self, tokens: Vec<PileToken>, source_code: &str) -> MietteResult<Option<AstNode>> {
    // A type to store either a usize or a Symbol
    #[derive(Debug, Clone)]
    enum StackItem {
      State(usize),
      Symbol(Symbol),
    }

    // The stack
    let mut stack: VecDeque<StackItem> = VecDeque::new();
    let mut parse_stack: Vec<ParseTreeNode> = Vec::new();
    stack.push_back(StackItem::State(0));

    // The input
    let mut input = tokens.into_iter();
    let mut next = input.next();

    loop {
      let top = stack.back().unwrap();

      if let StackItem::State(state) = top {
        if next.is_none() {
          // TODO: No more tokens to parse error
        }
        let PileToken {
          token: current_token,
          slice: _,
          span,
        } = next.clone().expect("No more tokens to parse");

        let symbol = if let Token::EndOfInput = current_token {
          Symbol::End
        } else {
          Symbol::Terminal(current_token.get_token_type_only())
        };

        let action = self
          .action_table
          .get(&(*state, symbol.clone()))
          .unwrap_or_else(|| panic!("No action for state {} and symbol {:?}", state, symbol));

        match action {
          Action::Shift(shift_state) => {
            let symbol = if let Token::End = current_token {
              Symbol::End
            } else {
              Symbol::Terminal(current_token.to_string())
            };
            parse_stack.push(ParseTreeNode::Terminal(current_token));
            stack.push_back(StackItem::Symbol(symbol));
            stack.push_back(StackItem::State(*shift_state));
            next = input.next();
          }
          Action::Reduce(reduce_state) => {
            let (lhs, rhs) = &self.grammar.productions[*reduce_state];

            // Normal stack
            let mut to_pop = rhs.len() * 2;
            while to_pop > 0 {
              stack.pop_back();
              to_pop -= 1;
            }

            // Parse stack
            let mut children = Vec::new();
            for _ in 0..rhs.len() {
              let node = parse_stack.pop().unwrap();
              children.push(node);
            }

            children.reverse();

            let node = ParseTreeNode::NonTerminal(lhs.clone(), children);
            parse_stack.push(node);

            let top = if let StackItem::State(state) = stack.back().unwrap() {
              state
            } else {
              &(0_usize)
              // TODO: Stack top is not a state error
            };

            let state = self
              .goto_table
              .get(&(*top, lhs.clone()))
              .expect("No goto for this state and symbol");

            stack.push_back(StackItem::Symbol(lhs.clone()));
            stack.push_back(StackItem::State(*state));
          }
          Action::Accept => {
            // println!("Accept");
            break;
          }
          Action::Error => {
            let expected_tokens = self.find_expected_symbol(*state);

            return Err(ParseError::UnexpectedToken {
              input: source_code.to_string(),
              extension_src: span_to_tuple(span),
              advice: format!(
                "Expected one of the following tokens: {:?} but got {:?}",
                expected_tokens, current_token
              ),
            })?;
          }
        }
      } else {
        // TODO: Stack top is not a state error
        // return Err("Stack top is not a state".to_string());
      }
    }

    let ast = parse_ast(&parse_stack[0]);

    Ok(Some(ast[0].clone()))
  }

  pub fn find_expected_symbol(&self, state: usize) -> Vec<String> {
    let mut expected_tokens = Vec::new();
    for ((current_state, current_symbol), value) in self.action_table.iter() {
      if *current_state == state {
        match value {
          Action::Shift(_) => {
            expected_tokens.push(current_symbol.to_string());
          }
          Action::Reduce(_) => {
            expected_tokens.push(current_symbol.to_string());
          }
          Action::Error => {}
          Action::Accept => {}
        }
      }
    }
    expected_tokens
  }
}

// Binary ast node
#[derive(Debug, Clone)]
pub struct AstNode {
  pub symbol: Symbol,
  pub children: Vec<AstNode>,
  pub token: Token,
}

pub struct AstNodeIter<'a> {
  nodes: Vec<&'a AstNode>,
}

pub struct AstNodeDetailedIter<'a> {
  stack: Vec<(&'a AstNode, usize, bool)>, // (node, depth, is_last)
}

impl<'a> AstNode {
  pub fn iter(&'a self) -> AstNodeIter<'a> {
    AstNodeIter { nodes: vec![self] }
  }

  pub fn detailed_iter(&'a self) -> AstNodeDetailedIter<'a> {
    AstNodeDetailedIter {
      stack: vec![(self, 0, true)],
    }
  }
}

impl<'a> Iterator for AstNodeIter<'a> {
  type Item = &'a AstNode;

  fn next(&mut self) -> Option<Self::Item> {
    let node = self.nodes.pop()?;
    self.nodes.extend(node.children.iter().rev()); // reverse to maintain order due to stack nature of vec
    Some(node)
  }
}

impl<'a> Iterator for AstNodeDetailedIter<'a> {
  type Item = (&'a AstNode, usize, bool);

  fn next(&mut self) -> Option<Self::Item> {
    let (node, depth, is_last) = self.stack.pop()?;

    for (i, child) in node.children.iter().enumerate().rev() {
      let is_last = i == 0;
      self.stack.push((child, depth + 1, is_last));
    }

    Some((node, depth, is_last))
  }
}

impl Display for AstNode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "AST")?;
    for (node, depth, is_last) in self.detailed_iter() {
      let prefix = if depth == 0 {
        "".to_string()
      } else {
        format!(
          "{}{}",
          "  ".repeat(depth - 1),
          if is_last {
            "\x1B[33m└─\x1B[0m"
          } else {
            "\x1B[33m├─\x1B[0m"
          }
        )
      };
      writeln!(f, "{}{}", prefix, node.symbol)?;
    }
    Ok(())
  }
}

fn parse_ast(node: &ParseTreeNode) -> Vec<AstNode> {
  // Iterate for each through the leaves of the tree, if the leave is a Integer push it to the
  // stack, if it is a operator pop the last two elements of the stack and create a new node
  // with the operator and the two elements as children. Append the new node to the stack.

  let mut stack: Vec<AstNode> = Vec::new();

  // Iterate over the tree inorder
  let mut traverse_stack: Vec<&ParseTreeNode> = Vec::new();
  let mut current_node = node;
  loop {
    if let ParseTreeNode::NonTerminal(_, children) = current_node {
      for child in children.iter().rev() {
        traverse_stack.push(child);
      }
    } else if let ParseTreeNode::Terminal(token) = current_node {
      match token {
        Token::String(string) => {
          stack.push(AstNode {
            symbol: Symbol::Terminal(string.to_string()),
            children: Vec::new(),
            token: token.clone(),
          });
        }
        Token::Integer(integer) => {
          stack.push(AstNode {
            symbol: Symbol::Terminal(integer.to_string()),
            children: Vec::new(),
            token: token.clone(),
          });
        }
        Token::Float(float) => {
          stack.push(AstNode {
            symbol: Symbol::Terminal(float.to_string()),
            children: Vec::new(),
            token: token.clone(),
          });
        }
        Token::ArithmeticOp { .. } => {
          let right = stack.pop().unwrap();
          let left = stack.pop().unwrap();
          stack.push(AstNode {
            symbol: Symbol::Terminal(token.to_string()),
            children: vec![left, right],
            token: token.clone(),
          });
        }
        Token::StackOps { .. } => {
          stack.push(AstNode {
            symbol: Symbol::Terminal(token.to_string()),
            children: Vec::new(),
            token: token.clone(),
          });
        }
        Token::ComparisonOp { .. } => {
          let right = stack.pop().unwrap();
          let left = stack.pop().unwrap();
          stack.push(AstNode {
            symbol: Symbol::Terminal(token.to_string()),
            children: vec![left, right],
            token: token.clone(),
          });
        }
        _ => {}
      }
    }

    if let Some(next_node) = traverse_stack.pop() {
      current_node = next_node;
    } else {
      break;
    }
  }

  // If there is more than on element on the stack create a new node with the symbol Program
  // and the stack as children
  stack = vec![AstNode {
    symbol: Symbol::NonTerminal("R".to_string()),
    children: stack,
    token: Token::Program,
  }];

  stack
}
