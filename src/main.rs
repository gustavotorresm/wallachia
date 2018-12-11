extern crate wallachia;
extern crate regex;

//#[macro_use]
//extern crate lazy_static;

use std::env;
use wallachia::config;
use wallachia::compiler;

fn main() {
  let config = config::parse_args(env::args());

  compiler::compile(config.file_path);
}
