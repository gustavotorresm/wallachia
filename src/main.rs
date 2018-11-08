extern crate wallachia;

use std::env;
use wallachia::config;

fn main() {
  let config = config::parse_args(env::args());

  println!("{:?}", config.file_path);
}
