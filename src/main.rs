use std::env;
use std::process;

use timers::Config;

fn main () {
    // clear terminal
    print!("\x1B[2J\x1B[1;1H");

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = timers::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}