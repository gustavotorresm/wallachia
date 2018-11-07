use std::string::String;
use std::fs::File;

use compiler::event_driven_module::engine::*;
use compiler::event_driven_module::actions::FileActions;

#[derive (Debug)]
pub enum FileEventData {
  FilePath(String),
  FileDescriptor(File),
}

impl PartialEq for FileEventData {
  fn eq(&self, other: &Self) -> bool {
    return true;
  }
}

impl Eq for FileEventData {};

type FileEvent = Event<FileEventData>;

pub struct FileEngine {
  input_queue: EngineQueue<FileEvent>
}

impl FileEngine {
  pub fn new() -> FileEngine {
    return FileEngine {
      input_queue: EngineQueue::<FileEvent>::new(),
    }
  }

  pub fn push_event(&self, event_type: FileActions, event_data: FileEventData) {
    self.input_queue.push(Event{
      priority: 1,
      action: event_type,
      data: event_data,
    });
  }
}

impl Engine<FileEventData> for FileEngine {
  fn consume(&self, event: Event) -> () {
    println!("{:?}: {:?}", event.action, event.data);
    return();
  }
}
