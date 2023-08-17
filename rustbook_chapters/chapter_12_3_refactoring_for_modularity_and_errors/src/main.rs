use std::env;
use std::process;

use chapter_12_3_refactoring_for_modularity_and_errors::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = chapter_12_3_refactoring_for_modularity_and_errors::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

