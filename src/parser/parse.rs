use miette::Result as MietteResult;
use std::collections::VecDeque;

use crate::{
  grammar::Symbol,
  lexer::{
    tokens::{span_to_tuple, Token},
    PileToken,
  },
  parser::{errors::ParseError, Action},
};

use super::SLR::SLR;

impl SLR {
  pub fn parse(&self, tokens: Vec<PileToken>, source_code: &str) -> MietteResult<()> {
    // A type to store either a usize or a Symbol
    #[derive(Debug, Clone)]
    enum StackItem {
      State(usize),
      Symbol(Symbol),
    }

    // The stack
    let mut stack: VecDeque<StackItem> = VecDeque::new();
    stack.push_back(StackItem::State(0));

    // The input
    let mut input = tokens.into_iter();
    let mut next = input.next();

    loop {
      let top = stack.back().unwrap();

      if let StackItem::State(state) = top {
        if next.is_none() {
          // TODO: No more tokens to parse error
          // return Err("No more tokens to parse".to_string());
        }
        let PileToken {
          token: current_token,
          span,
          ..
        } = next.clone().expect("No more tokens to parse");

        let symbol = if let Token::EndOfInput = current_token {
          Symbol::End
        } else {
          Symbol::Terminal(current_token.get_token_type_only())
        };

        // print the queue
        // print!("Stack: ");
        // for item in stack.iter() {
        //   match item {
        //     StackItem::State(state) => print!("{} ", state),
        //     StackItem::Symbol(symbol) => print!("{} ", symbol),
        //   }
        // }
        // println!();

        let action = self
          .action_table
          .get(&(*state, symbol.clone()))
          .unwrap_or_else(|| panic!("No action for state {} and symbol {:?}", state, symbol));

        // println!(
        //   "State: {}, Token: {}, Action: {:?}",
        //   state, current_token, action
        // );

        match action {
          Action::Shift(shift_state) => {
            let symbol = if let Token::End = current_token {
              Symbol::End
            } else {
              Symbol::Terminal(current_token.to_string())
            };
            stack.push_back(StackItem::Symbol(symbol));
            stack.push_back(StackItem::State(*shift_state));
            next = input.next();
          }
          Action::Reduce(reduce_state) => {
            let (lhs, rhs) = &self.grammar.productions[*reduce_state];
            let mut to_pop = rhs.len() * 2;
            while to_pop > 0 {
              stack.pop_back();
              to_pop -= 1;
            }

            let top = if let StackItem::State(state) = stack.back().unwrap() {
              state
            } else {
              &(0_usize)
              // TODO: Stack top is not a state error
              // return Err("Stack top is not a state".to_string());
            };

            let state = self
              .goto_table
              .get(&(*top, lhs.clone()))
              .expect("No goto for this state and symbol");

            stack.push_back(StackItem::Symbol(lhs.clone()));
            stack.push_back(StackItem::State(*state));
          }
          Action::Accept => {
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

    Ok(())
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
