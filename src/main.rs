use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let grepConfig= Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", grepConfig.query);
    println!("In file {}", grepConfig.filename);

    if let Err(e) = run(grepConfig) {
        println!("Application error : {}", e);

        process::exit(1);
    }
}