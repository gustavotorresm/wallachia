use std::io::{BufReader, BufRead};
use std::fs::File;
use std::boxed::Box;
use std::any::Any;

use compiler::event_driven_module::engine::*;
use compiler::event_driven_module::actions::FileActions;

pub struct FileEngine;

fn read_event(mut file: File, instant: usize) -> Event {
  return Event {
    priority: instant,
    action: Box::new(FileActions::Read),
    data: Box::new(file),
  }
}

impl FileEngine {
  pub fn new() -> FileEngine {
    return FileEngine{};
  }

  fn open_file(&self, path: String, output: &mut EngineQueue, time: usize) -> () {
    let mut file = File::open(&path);
    match file {
      Ok(file) => output.push(read_event(file, time)),
      Err(e) => panic!("Could not open file {}: {}", path, e),
    }
  }

  fn read(&self, file: File, output: &mut EngineQueue, time: usize) -> () {
    let file = BufReader::new(file);

    for (i, line) in file.lines().enumerate() {
      output.push()
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
             time: usize) -> Option<EngineQueue> {
    match event.action.downcast::<FileActions>() {
      Err(event) => panic!("Wrong event consumed by FileEngine: {:?}", event),
      Ok(action) => {
        self.handle_action(*action, event.data, output_queue, time);
      },
    }
    return None;
  }
}
