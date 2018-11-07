#[derive(Debug, PartialEq, Eq)]
pub enum Actions {
  FileAction(FileActions),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileActions {
  Open,
  ReadLine,
  Close,
}
