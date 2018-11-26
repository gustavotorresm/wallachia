use std::fs::File;
use std::boxed::Box;
use std::any::Any;

use compiler::event_driven_module::engine::*;
use compiler::event_driven_module::actions::FileActions;

pub struct FileEngine {
  input_queue: Option<EngineQueue>,
  output_queue: EngineQueue,
  time: usize,
}

fn read_event(mut file: File, instant: usize) -> Event {
  return Event {
    priority: instant,
    action: Box::new(FileActions::ReadLine),
    data: Box::new(file),
  }
}

impl FileEngine {
  pub fn new() -> FileEngine {
    let output_queue = EngineQueue::new();
    return FileEngine{
      input_queue: None,
      output_queue: EngineQueue::new(),
      time: 0,
    };
  }

  fn send_to_output(&mut self, event: Event) {
    self.output_queue.push(event);
  }

  fn open_file(&mut self, path: String) -> () {
    let mut file = File::open(&path);
    let time = self.time;
    match file {
      Ok(file) => self.send_to_output(read_event(file, time)),
      Err(e) => panic!("Could not open file {}: {}", path, e),
    }
  }

  fn handle_action(&mut self, action: FileActions, data: Box<Any>) -> () {
    match action {
      FileActions::Open => self.open_file(*data.downcast::<String>().unwrap()),
      _ => panic!("Not yet implemented"),
    }
  }
}

impl Engine for FileEngine {
  fn consume(&mut self, event: Event) -> () {
    match event.action.downcast::<FileActions>() {
      Err(event) => panic!("Wrong event consumed by FileEngine: {:?}", event),
      Ok(action) => {
        self.handle_action(*action, event.data);
      },
    }
  }
}
