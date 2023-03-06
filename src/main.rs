use clap::{arg, Command};

use notify_rust::Notification;
use timers::Config;

fn main() {
    // clear terminal
    print!("\x1B[2J\x1B[1;1H");

    let cmd = Command::new("timer")
        .author("Vivek R, 123vivekr@gmail.com")
        .version("0.1.0")
        .args(&[
            arg!(<time_string> "Time in format HHhMMmSSs"),
            arg!(-o --output [output] "Output filename"),
        ])
        .get_matches();

    let time_string = cmd
        .get_one::<String>("time_string")
        .expect("time_string is required")
        .to_string();

    let output_filename = cmd.get_one::<String>("output").map(|s| s.to_string());

    let config = Config::new(time_string, output_filename);

    timers::run(config);

    Notification::new()
        .summary("Timer")
        .body("Time's up!")
        .icon("clock")
        .show()
        .expect("Error displaying notification");
}
