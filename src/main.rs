use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let grepConfig: Config = parse_config(&args);
}

fn parse_config(args: &[String]) -> Config {
    let returnConfig = Config {
        query: args[1].clone(),
        filename: args[2].clone(),
    };

    returnConfig
}
