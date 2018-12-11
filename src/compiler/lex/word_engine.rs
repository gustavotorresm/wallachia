use std::any::Any;
use std::boxed::Box;
use std::string::String;

use regex::Regex;
use lazy_static::*;

use compiler::event_driven_module::engine::*;
use super::actions::WordActions;

pub struct WordEngine;

type SplitLineData = (usize, String);

impl WordEngine {
  pub fn new() -> WordEngine {
    return WordEngine{};
  }

  fn split_line(&self, data: SplitLineData, output: &mut EngineQueue, time: usize) -> () {
    lazy_static! {
      static ref RE: Regex = Regex::new(r#"(".*"|[^\s]*)"#).unwrap();//r#"^(".*?"|[^\s]*)$"#).unwrap();
    }
    let (num, line) = data;
    println!("{}: {}", num, line);

    for cap in RE.captures_iter(&line) {
      println!("{}", &cap[1]);
    }
  }

  fn handle_action(&mut self,
                   action: WordActions,
                   data: Box<Any>,
                   output: &mut EngineQueue,
                   time: usize) -> () {
    match action {
      WordActions::SplitLine => self.split_line(*data.downcast::<SplitLineData>().unwrap(), output, time),
      _ => panic!("Not yet implemented"),
    }
  }
}

impl Engine for WordEngine {
  fn consume(&mut self,
             event: Event,
             output_queue: &mut EngineQueue,
             time: usize) {
    match event.action.downcast::<WordActions>() {
      Err(error) => panic!("Wrong event consumed by WordEngine: {:?}", error),
      Ok(action) => {
        self.handle_action(*action, event.data, output_queue, time);
      },
    }
  }
}
