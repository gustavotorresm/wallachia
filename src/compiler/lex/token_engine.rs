use std::any::Any;
use std::boxed::Box;
use std::string::String;

use regex::Regex;
use lazy_static::*;

use compiler::event_driven_module::engine::*;
use super::actions::TokenActions;
use super::events::{tokenize_event};

pub enum Number {
  Integer(isize),
  Float(f32),
}

pub enum Token {
  Number(Number),
}

pub struct TokenEngine;

type SplitLineData = (usize, String);

impl TokenEngine {
  pub fn new() -> TokenEngine {
    return TokenEngine{};
  }

  fn split_line(&self, data: SplitLineData, output: &mut EngineQueue, time: usize) -> () {
    lazy_static! {
      static ref RE: Regex = Regex::new(r#"(".*?"|[^\s]*)"#).unwrap();
    }
    let (num, line) = data;

    for (i, cap) in RE.captures_iter(&line).enumerate() {
      output.push(tokenize_event(cap[1].to_string(), time + 10 * num + i));
    }

    output.push(tokenize_event("\n".to_string(), time + 10 *(num + 1) - 1));
  }

  fn classify(&self, data: String, output: &mut EngineQueue, time: usize) -> () {
    println!("{} - {}", time, data);
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
