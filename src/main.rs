use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let grepConfig: Config = parse_config(&args);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    Config {
        query: args[1].clone(),
        filename: args[2].clone(),
    }
}