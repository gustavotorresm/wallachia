#[derive(Debug, PartialEq, Eq)]
pub enum Actions {
  FileAction(FileActions),
  TokenAction(TokenActions),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileActions {
  Open,
  Read,
  Close,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenActions {
  SplitLine,
}
