use compiler::lex::tokens::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum ParserActions {
  NewToken(Token),
}
