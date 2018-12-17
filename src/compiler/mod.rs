use std::string::String;
use std::any::TypeId;

pub mod event_driven_module;
pub mod actions;
pub mod lex;
pub mod parser;

use compiler::lex::tokens::*;
use self::parser::Parser;
use self::parser::actions::ParserActions;

struct Compiler {
  lexer: lex::Lexer,
}

fn create_compiler() -> Compiler {
  return Compiler {
    lexer: lex::Lexer::new(),
  };
}

pub fn compile(file_path: String) -> Result<(), String>{
  println!("Compiling...");
  let mut compiler = create_compiler();
  let mut parser = Parser::new();

  let mut tokens_events = compiler.lexer.run(file_path);
  parser.run(tokens_events);

  Ok(())
}
