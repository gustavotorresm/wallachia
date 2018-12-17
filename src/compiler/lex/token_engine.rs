use std::any::Any;
use std::boxed::Box;
use std::string::String;

use regex::Regex;
use lazy_static::*;

use compiler::event_driven_module::engine::*;
use super::actions::TokenActions;
use super::events::{tokenize_event};
use compiler::parser::events::{new_token_event};
use super::tokens::*;

pub struct TokenEngine;

type SplitLineData = (usize, String);

fn is_keyword(string: &str) -> bool {
  let keywords = ["END", "LET", "FN", "SIN", "COS", "TAN", "ATN", "EXP", "ABS", "LOG", "SQR", "INT", "RND", "READ", "DATA", "PRINT", "GOTO", "GO", "TO", "IF", "THEN", "FOR", "STEP", "NEXT", "DIM", "DEF", "GOSUB", "RETURN", "REM", "INPUT"];
  return keywords.iter().any(|s| s == &string);
}

fn is_variable(string: &str) -> bool {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^[a-zA-Z][0-9]?$").unwrap();
  }
  return RE.is_match(string);
}

fn is_function(string: &str) -> bool {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^f[a-zA-Z][0-9]?$").unwrap();
  }
  return RE.is_match(string);
}

impl TokenEngine {
  pub fn new() -> TokenEngine {
    return TokenEngine{};
  }

  fn split_line(&self, data: SplitLineData, output: &mut EngineQueue, time: usize) -> () {
    lazy_static! {
      static ref RE: Regex = Regex::new(r#"(".*?"|[^\s]*)"#).unwrap();
    }
    let (num, line) = data;
    let mut offset = 0;

    for cap in RE.captures_iter(&line) {
      output.push(tokenize_event(cap[1].to_string(), time + 10 * num + offset));
      offset += 1;
    }

    output.push(tokenize_event("\n".to_string(), time + 10 *(num + 1) - 1));
    println!("NEWLINE {}", time + 10 * (num + 1) - 1);
    output.push(new_token_event(new_newline(), time + 10 *(num + 1) - 1));
  }

  fn classify(&self, data: String, output: &mut EngineQueue, time: usize) -> () {
    const SYMBOLS_STR: &str = r"(?:<>)|(?:>=)|(?:<=)|\$|\(|\)|\+|\-|\*|/|\^|=|<|>|;|,";
    const STRING_STR: &str = r#"(?:".*?")"#;
    const NUMBER_STR: &str = r"(^[+-]?\d+$)|(^[+-]?(\d+|\d+\.\d*|\d*\.\d+)(E[+-]\d+)?$)";

    lazy_static! {
      static ref STRIP_EXPRESSIONS: Regex =
        Regex::new(
          &format!(r#"({}|{}|{}|\w+)"#,
                   NUMBER_STR,
                   STRING_STR,
                   SYMBOLS_STR)
        ).unwrap();

      static ref INT: Regex = Regex::new(r"^[+-]?\d+$").unwrap();
      static ref FLOAT: Regex = Regex::new(r"^[+-]?(\d+|\d+\.\d*|\d*\.\d+)(E[+-]\d+)?$").unwrap();
      static ref STRING: Regex = Regex::new(r#"^"(.*)"$"#).unwrap();
      static ref WORD: Regex = Regex::new(r"^\w[\d\w]*$").unwrap();
      static ref NEWLINE: Regex = Regex::new(r"^\n$").unwrap();
      static ref SYMBOL: Regex = Regex::new(r"^\W{1,2}$").unwrap();
    }

    for (i, cap) in STRIP_EXPRESSIONS.captures_iter(&data).enumerate() {
      let token = cap[0].to_string();

      if INT.is_match(&token) {
        let value = token.parse::<isize>().unwrap();
        println!("INT {}", value);
        output.push(new_token_event(new_integer(value), time + i));
      } else if FLOAT.is_match(&token) {
        let value = token.parse::<f32>().unwrap();
        println!("FLOAT {}", value);
        output.push(new_token_event(new_float(value), time + i));
      } else if let Some(caps) = STRING.captures(&token) {
        let value = String::from(caps.get(1).unwrap().as_str());
        println!("STRING {}", value);
        output.push(new_token_event(new_string(value), time + i));
      } else if WORD.is_match(&token) {
        let value = token.clone();
        if is_keyword(&token) {
          println!("KEYWORD {}: {}", value, time + i);
          output.push(new_token_event(new_keyword(value), time + i));
        } else if is_variable(&token) {
          println!("FN {}:{}", value, time + i);
          output.push(new_token_event(new_variable(value), time + i));
        } else if is_variable(&token) {
          println!("VAR {}: {}", value, time + i);
          output.push(new_token_event(new_function(value), time + i));
        } else {
          println!("WORD {}: {}", value, time + i);
          output.push(new_token_event(new_name(value), time + i));
        }
      } else if NEWLINE.is_match(&token) {
        //output.push(new_token_event(new_newline(), time + i));
      } else if SYMBOL.is_match(&token) {
        output.push(new_token_event(new_symbol(token.clone()), time + i))
      } else {
        panic!("Unrecognized character sequence {}", token);
      }
    }
  }

  fn handle_action(&mut self,
                   action: TokenActions,
                   data: Box<Any>,
                   output: &mut EngineQueue,
                   time: usize) -> () {
    match action {
      TokenActions::SplitLine => self.split_line(*data.downcast::<SplitLineData>().unwrap(), output, time),
      TokenActions::ClassifyToken => self.classify(*data.downcast::<String>().unwrap(), output, time),
      _ => panic!("Not yet implemented"),
    }
  }
}

impl Engine for TokenEngine {
  fn consume(&mut self,
             event: Event,
             output_queue: &mut EngineQueue,
             time: usize) {
    match event.action.downcast::<TokenActions>() {
      Err(error) => panic!("Wrong event consumed by TokenEngine: {:?}", error),
      Ok(action) => {
        self.handle_action(*action, event.data, output_queue, time);
      },
    }
  }
}
