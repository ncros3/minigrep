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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok( Config { query, filename} )
    }
}