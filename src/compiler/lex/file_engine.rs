use std::string::String;
use std::fs::File;
use std::boxed::Box;
use std::any::Any;

use compiler::event_driven_module::engine::*;
use compiler::event_driven_module::actions::FileActions;

pub struct FileEngine {}

impl FileEngine {
  pub fn new() -> FileEngine {
    return FileEngine{};
  }

  fn handle_action(&self, action: FileActions, data: Box<Any>) -> Event {

    panic!("Optimistic panic");
  }
}

impl Engine for FileEngine {
  fn consume(&self, event: Event) -> Result<(), String> {
    match event.action.downcast_ref::<FileActions>() {
      None => panic!("Wrong event consumed by FileEngine: {:?}", event),
      Some(action) => {
        self.handle_action(*action, event.data);
        Ok(())
      },
    }
  }
}
