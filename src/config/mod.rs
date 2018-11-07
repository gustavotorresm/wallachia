use std::env;
use super::FilePath;

#[derive(Debug)]
pub struct Config {
  pub file_path: FilePath,
}

impl Config {
  pub fn new(file_path: FilePath) -> Config {
    Config { file_path }
  }
}

pub fn parse_args(args: env::Args) -> Config {
  match args.skip(1).next() {
    Some(path) => return Config::new(path),
    None => panic!("Usa direito"),
  }

}
