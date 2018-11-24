use std::string::String;

pub mod event_driven_module;
pub mod lex;

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
  let compiler = create_compiler();

  compiler.lexer.run(file_path);
  return Ok(());
}
