use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::boxed::Box;

use std::string::String;

use ::FilePath;
use compiler::event_driven_module::engine::{Engine, EngineQueue, Event};

use self::file_engine::FileEngine;
use self::word_engine::WordEngine;
use self::events;
use self::actions;

pub mod file_engine;
pub mod word_engine;
pub mod actions;
pub mod events;

pub struct Lexer {
  file_engine: FileEngine,
}

impl Lexer {
  pub fn new() -> Lexer {
    let mut file_engine = FileEngine::new();
    let mut word_engine = WordEngine::new();
    return Lexer{ file_engine };
  }

  pub fn run(&mut self, file_path: String) -> () {
    let mut queue = EngineQueue::from(vec![events::initial_event(file_path)]);

    while let Some(event) = queue.pop() {
      let time = event.priority;
      if let Some(_) = event.action.downcast_ref::<actions::FileActions>() {
        &mut self.file_engine.consume(event, &mut queue, time);
      }
    }
  }
}

/*
pub fn tokenize(file_path: FilePath) {
  let mut queue = EngineQueue::new();

  println!("{:?}", queue.peek());

  queue.push(Actions::FileAction(FileActions::Open));
  queue.push(Actions::FileAction(FileActions::Close));

  println!("{:?}", queue);
  println!("{:?}", queue.peek());

  let f = File::open("res/test.bas").expect("Deu ruim no servidor!");
  let f = BufReader::new(f);

  for (n,line) in f.lines().enumerate() {
    match line {
      Ok(line) => println!("{}: {}", n, line),
      _ => panic!("Deu ruim de novo"),
    }
  }
}
*/
