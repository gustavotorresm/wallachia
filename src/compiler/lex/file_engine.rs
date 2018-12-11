use std::io::{BufReader, BufRead};
use std::fs::File;
use std::boxed::Box;
use std::any::Any;

use compiler::event_driven_module::engine::*;
use compiler::lex::actions::FileActions;
use compiler::lex::events::{file_open_event, file_read_event, word_line_event};

pub struct FileEngine;

impl FileEngine {
  pub fn new() -> FileEngine {
    return FileEngine{};
  }

  fn open_file(&self, path: String, output: &mut EngineQueue, time: usize) -> () {
    let mut file = File::open(&path);
    match file {
      Ok(file) => output.push(file_read_event(file, time)),
      Err(e) => panic!("Could not open file {}: {}", path, e),
    }
  }

  fn read(&self, file: File, output: &mut EngineQueue, time: usize) -> () {
    let file = BufReader::new(file);

    for (i, line) in file.lines().enumerate() {
      match line {
        Ok(line) => output.push(word_line_event(line, i + 1, time)),
        Err(error) => panic!("Error while reading line {}: {}", i, error),
      }
    }
  }

  fn handle_action(&mut self,
                   action: FileActions,
                   data: Box<Any>,
                   output: &mut EngineQueue,
                   time: usize) -> () {
    match action {
      FileActions::Open => self.open_file(*data.downcast::<String>().unwrap(), output, time),
      FileActions::Read => self.read(*data.downcast::<File>().unwrap(), output, time),
      _ => panic!("Not yet implemented"),
    }
  }
}

impl Engine for FileEngine {
  fn consume(&mut self,
             event: Event,
             output_queue: &mut EngineQueue,
             time: usize) {
    match event.action.downcast::<FileActions>() {
      Err(error) => panic!("Wrong event consumed by FileEngine: {:?}", error),
      Ok(action) => {
        self.handle_action(*action, event.data, output_queue, time);
      },
    }
  }
}
