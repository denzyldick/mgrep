use std::env;
use std::process;
use mgrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} ", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = mgrep::run(config) {
        eprintln!("Error:{}", e);
        process::exit(1);
    }
}

