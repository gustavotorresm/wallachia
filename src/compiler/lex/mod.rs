use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::boxed::Box;

use std::string::String;

use ::FilePath;
use compiler::event_driven_module::engine::{Engine, EngineQueue, Event};

use self::file_engine::FileEngine;
use self::token_engine::TokenEngine;

pub mod file_engine;
pub mod token_engine;
pub mod actions;
pub mod events;
pub mod tokens;

pub struct Lexer {
  file_engine: FileEngine,
  word_engine: TokenEngine,
}

impl Lexer {
  pub fn new() -> Lexer {
    let mut file_engine = FileEngine::new();
    let mut word_engine = TokenEngine::new();
    return Lexer{ file_engine, word_engine };
  }

  pub fn run(&mut self, file_path: String) -> EngineQueue {
    let mut queue = EngineQueue::from(vec![events::initial_event(file_path)]);
    let mut output = EngineQueue::new();

    while let Some(event) = queue.pop() {
      let time = event.priority;
      if event.action.is::<actions::FileActions>() {
        &mut self.file_engine.consume(event, &mut queue, time);
      } else if event.action.is::<actions::TokenActions>() {
        &mut self.word_engine.consume(event, &mut queue, time);
      } else {
        output.push(event);
      }
    }

    return output;
  }
}
