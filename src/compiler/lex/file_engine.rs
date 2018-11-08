use std::string::String;
use std::fs::File;
use std::boxed::Box;

use compiler::event_driven_module::engine::*;
use compiler::event_driven_module::actions::FileActions;

pub struct FileEngine {}

impl FileEngine {
  pub fn new() -> FileEngine {
    return FileEngine{};
  }
}

impl Engine for FileEngine {
  fn consume(&self, event: Event) -> () {
    println!("{:?}: {:?}", event.action, event.data);
    return();
  }
}
