use compiler::lex::file_engine::FileEventData;

#[derive(Debug, Eq, PartialEq)]
pub enum DataTypes {
  FileEvent(FileEventData),
}
