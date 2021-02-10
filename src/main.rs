use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let grep_config= Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(grep_config) {
        println!("Application error : {}", e);
        process::exit(1);
    }
}