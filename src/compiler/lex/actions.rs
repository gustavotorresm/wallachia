#[derive(Debug, PartialEq, Eq)]
pub enum Actions {
  FileAction(FileActions),
  WordAction(WordActions),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileActions {
  Open,
  Read,
  Close,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WordActions {
  SplitLine,
}
