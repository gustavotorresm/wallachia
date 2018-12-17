use std::vec::Vec;
use std::string::String;

use compiler::event_driven_module::engine::{EngineQueue};
use compiler::lex::tokens::*;

pub mod actions;
pub mod events;

fn is_keyword(token: &Token, keyword: &str) -> bool {
  if let Token::Word(Word::Keyword(value)) = token {
    return value.as_str() == keyword;
  }
  return false;
}

#[derive(Clone)]
pub enum Parsers {
  Root,
  Statement,
  Back,
}

#[derive(Debug, Clone)]
enum GrammarParserStates {
  S0,
  S1,
  S2,
}

#[derive(Clone)]
struct GrammarParser {
  state: GrammarParserStates,
}

impl GrammarParser {
  fn consume(&mut self, token: Token) -> Option<Parsers> {
    println!("Root {:?}, {:?}", self.state, token);
    match self.state {
      GrammarParserStates::S0 => {
        if let Token::Number(Number::Integer(x)) = token {
          self.state = GrammarParserStates::S1;
        } else {
          panic!("Unexpected token {:?}", token);
        }
      },
      GrammarParserStates::S1 => {
        self.state = GrammarParserStates::S2;
        return Some(Parsers::Statement);
      },
      GrammarParserStates::S2 => {
        if let Token::Control(Control::Newline) = token {
          self.state = GrammarParserStates::S0;
        } else {
          panic!("Unexpected token {:?}. Expected new line.", token);
        }
      },
    }
    return None;
  }
}

#[derive(Debug, Clone)]
enum StatementParserStates {
  S0,
  S1,
  S2,
  REMARK,
  DONE,
}

#[derive(Clone)]
struct StatementParser {
  state: StatementParserStates,
}

impl StatementParser {
  fn consume(&mut self, token: Token) -> Option<Parsers> {
    println!("Statement {:?}, {:?}", self.state, token);
    match self.state {
      StatementParserStates::S0 => {
        if is_keyword(&token, "RETURN") || is_keyword(&token, "END") {
          self.state = StatementParserStates::DONE;
        } else if is_keyword(&token, "REM") {
          self.state = StatementParserStates::REMARK
        } else {
          panic!("Unexpected token {:?}", token);
        }
      },
      StatementParserStates::S1 => {
      },
      StatementParserStates::S2 => {
        if let Token::Control(Control::Newline) = token {
          self.state = StatementParserStates::S0;
        } else {
          panic!("Unexpected token {:?}. Expected new line.", token);
        }
      },
      StatementParserStates::REMARK => {
        if let Token::Control(Control::Newline) = token {
          self.state = StatementParserStates::S0;
          return Some(Parsers::Back);
        }
      },
      StatementParserStates::DONE => {
        self.state = StatementParserStates::S0;
        return Some(Parsers::Back);
      }
    }
    return None;
  }
}

#[derive(Clone)]
pub struct Parser {
  root_parser: GrammarParser,
  statement_parser: StatementParser,
  stack: Vec<Parsers>,
}

impl Parser {
  pub fn new() -> Parser {
    Parser {
      root_parser: GrammarParser{ state: GrammarParserStates::S0 },
      statement_parser: StatementParser { state: StatementParserStates::S0 },
      stack: Vec::new(),
    }
  }

  pub fn run(&mut self, mut tokens: EngineQueue) -> () {
    self.stack.push(Parsers::Root);

    while let Some(event) = tokens.pop() {
      let token = *event.data.downcast::<Token>().unwrap();
      self.consume(token);
    }
  }

  pub fn consume(&mut self, token: Token) -> () {
    let token_copy = token.clone();
    let maybe_stack =
      match self.stack.last().unwrap() {
        Parsers::Root => self.root_parser.consume(token),
        Parsers::Statement => self.statement_parser.consume(token),
        _ => panic!("Invalid parser"),
      };

    match maybe_stack {
      Some(Parsers::Back) => {
        self.stack.pop();
        self.consume(token_copy);
      },
      Some(parser) => {
        self.stack.push(parser);
        self.consume(token_copy);
      },
      _ => {},
    }
  }
}
