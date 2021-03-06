/* Used to parse cmdline args */
use std::env;
use std::process;

use minigrep::*;

fn main() {
  /* Call collect to turn the args Iterator into a collection */
  let args: Vec<String> = env::args().collect();

  let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
    eprintln! ("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  if let Err(e) = minigrep::run(config) {
    eprintln! ("Application error: {}", e);
    process::exit(1);
  }
}

