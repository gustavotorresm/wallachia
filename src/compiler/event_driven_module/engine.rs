use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering};
use std::any::Any;
use super::actions::Actions;
use super::events::DataTypes;

type CodeLine = String;
pub type EngineQueue<D = DataTypes> = BinaryHeap<Event<D>>;

#[derive(PartialEq, Eq, Debug)]
pub struct Event<Data = DataTypes> {
  pub priority: usize,
  pub action: Actions,
  pub data: Data,
}

impl Ord for Event {
  fn cmp (&self, other: &Event) -> Ordering {
    return self.priority.cmp(&other.priority);
  }
}

impl PartialOrd for Event {
  fn partial_cmp (&self, other: &Event) -> Option<Ordering> {
    return Some(self.cmp(other));
  }
}


pub trait Engine<Input> {
  fn consume(&self, event: Event) -> ();
}
