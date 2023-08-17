// V.1

// use std::env;
// use std::process;

// use project_lib::Config;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let config = Config::build(&args).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });

//     if let Err(e) = project_lib::run(config) {
//         eprintln!("Application error: {e}");
//         process::exit(1);
//     }
// }


// V.2

// Передача возвращаемого значения из env::args в Config::build.
// Не скомпилируется

use std::env;
use std::process;

use project_lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = project_lib::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}