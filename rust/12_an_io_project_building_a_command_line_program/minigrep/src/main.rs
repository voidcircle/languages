use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let configuration: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(error_message) = minigrep::run(configuration) {
        eprintln!("Application Error: {error_message}");
        process::exit(1);
    }
}
