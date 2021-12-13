use std::env;
use std::process;
use mgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} ", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = mgrep::run(config) {
        println!("Error:{}", e);
        process::exit(1);
    }
}

