use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let grepConfig= Config::new(&args);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}