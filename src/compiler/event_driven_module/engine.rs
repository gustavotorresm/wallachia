use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering};
use std::any::Any;
use std::boxed::Box;

type CodeLine = String;
pub type EngineQueue = BinaryHeap<Event>;

#[derive(Debug)]
pub struct Event {
  pub priority: usize,
  pub action: Box<Any>,
  pub data: Box<Any>,
}

impl PartialOrd for Event {
  fn partial_cmp (&self, other: &Event) -> Option<Ordering> {
    return Some(self.cmp(other));
  }
}

impl Ord for Event {
  fn cmp (&self, other: &Event) -> Ordering {
    return self.priority.cmp(&other.priority);
  }
}

// Events are always different 
impl PartialEq for Event {
  fn eq(&self, other: &Event) -> bool {
    false
  }
}

impl Eq for Event {}


pub trait Engine {
  fn run(&mut self, mut queue: EngineQueue, output_queue: &mut EngineQueue, time: usize) -> () {
    while let Some(x) = queue.pop() {
      self.consume(x, output_queue, time);
    }
  }

  fn consume(&mut self,
             event: Event,
             output_queue: &mut EngineQueue,
             time:usize) -> Option<EngineQueue>;
}
