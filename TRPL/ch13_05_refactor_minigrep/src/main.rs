extern crate ch13_05_refactor_minigrep;

use std::{env, process};
use ch13_05_refactor_minigrep::Config;

fn main() {
    //let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("-----------------------");
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    println!("-----------------------");
    if let Err(e) = ch13_05_refactor_minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
