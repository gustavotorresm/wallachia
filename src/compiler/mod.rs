use std::string::String;

pub mod event_driven_module;
pub mod lex;

struct Compiler {
  lexer: lex::Lexer,
}

static compiler: Compiler = Compiler {
  lexer: lex::Lexer::new(),
};

pub fn compile(file_path: String) -> Result<(), String>{
  compiler.lexer.run(file_path);
  return Ok(());
}
