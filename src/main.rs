use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let grepConfig= Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", grepConfig.query);
    println!("In file {}", grepConfig.filename);

    run(grepConfig);
}

fn run( config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong went reading the file");

    println!("With text : {}", contents);
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