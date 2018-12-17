use std::boxed::Box;

use compiler::event_driven_module::engine::Event;
use compiler::lex::tokens::Token;
use compiler::parser::actions::ParserActions;

pub fn new_token_event(token: Token, instant: usize) -> Event {
  return Event {
    priority: instant,
    action: Box::new(ParserActions::NewToken),
    data: Box::new(token),
  }
}
