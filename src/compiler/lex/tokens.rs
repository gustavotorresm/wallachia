use std::string::String;

#[derive(Debug, Clone)]
pub enum Number {
  Integer(isize),
  Float(f32),
}

#[derive(Debug, Clone)]
pub enum Name {
  Variable(String),
  Function(String),
  Name(String),
}

#[derive(Debug, Clone)]
pub enum Word {
  Keyword(String),
  Name(Name),
}

#[derive(Debug, Clone)]
pub enum Symbol {
  PlusSign,
  MinusSign,
  ProductSign,
  DivisionSign,
  ExponationSign,
  OpenParentheses,
  CloseParentheses,
}

#[derive(Debug, Clone)]
pub enum Control {
  Newline,
}

#[derive(Debug, Clone)]
pub enum Token {
  Number(Number),
  Word(Word),
  String(String),
  Symbol(String),
  Control(Control),
}

impl PartialEq for Token {
  fn eq(&self, other: &Token) -> bool {
    true
  }
}

impl Eq for Token {}


pub fn new_integer(value: isize) -> Token {
  return Token::Number(Number::Integer(value));
}
pub fn new_float(value: f32) -> Token {
  return Token::Number(Number::Float(value));
}
pub fn new_string(value: String) -> Token {
  return Token::String(value);
}
pub fn new_keyword(value: String) -> Token {
  return Token::Word(Word::Keyword(value));
}
pub fn new_variable(value: String) -> Token {
  return Token::Word(Word::Name(Name::Variable(value)));
}
pub fn new_function(value: String) -> Token {
  return Token::Word(Word::Name(Name::Function(value)));
}
pub fn new_name(value: String) -> Token {
  return Token::Word(Word::Name(Name::Name(value)));
}
pub fn new_newline() -> Token {
  return Token::Control(Control::Newline);
}
pub fn new_symbol(value: String) -> Token {
  return Token::Symbol(value);
}
