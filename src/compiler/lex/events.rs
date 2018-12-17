use std::boxed::Box;
use std::fs::File;
use std::string::String;

use compiler::lex::actions::*;
use compiler::event_driven_module::engine::Event;
use ::FilePath;
use compiler::lex::tokens::Token;

pub fn initial_event(file_path: FilePath) -> Event {
  file_open_event(file_path, 0)
}

pub fn file_open_event(file_path: FilePath, instant: usize) -> Event {
  Event {
    priority: instant,
    action: Box::new(FileActions::Open),
    data: Box::new(file_path),
  }
}

pub fn file_read_event(mut file: File, instant: usize) -> Event {
  return Event {
    priority: instant,
    action: Box::new(FileActions::Read),
    data: Box::new(file),
  }
}

pub fn word_line_event(line: String, line_number: usize, instant: usize) -> Event {
    return Event {
        priority: instant,
        action: Box::new(TokenActions::SplitLine),
        data: Box::new((instant + line_number, line)),
    }
}

pub fn tokenize_event(word: String, instant: usize) -> Event {
  return Event {
    priority: instant,
    action: Box::new(TokenActions::ClassifyToken),
    data: Box::new(word),
  }
}

