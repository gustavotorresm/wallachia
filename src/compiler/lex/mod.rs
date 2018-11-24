use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::boxed::Box;

use std::string::String;

use ::FilePath;
use compiler::event_driven_module::engine::{Engine, EngineQueue, Event};
use compiler::event_driven_module::actions::{Actions, FileActions};

use self::file_engine::FileEngine;

pub mod file_engine;

pub struct Lexer {
  file_engine: FileEngine,
}

impl Lexer {
  fn initial_event(file_path: FilePath) -> Event {
    Event {
      priority: 1,
      action: Box::new(FileActions::Open),
      data: Box::new(file_path),
    }
  }

  pub fn new() -> Lexer {
    let file_engine = FileEngine::new();
    return Lexer{ file_engine };
  }

  pub fn run(&self, file_path: String) -> () {
    let mut queue = EngineQueue::from(vec![Lexer::initial_event(file_path)]);

    while let Some(event) = queue.pop() {
      if let Some(_) = event.action.downcast_ref::<FileActions>() {
        self.file_engine.consume(event);
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
