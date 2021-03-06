use figlet_rs::FIGfont;
use std::env;
use std::fmt;
use std::str;
use std::time::Duration;
use std::thread;
use timer::Timer as TimerLib;
use chrono;

#[derive(Debug, Clone)]
pub struct ParserError;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing arguments")
    }
}

#[derive(Debug, Clone)]
pub struct Timer {
    pub seconds: usize,
    pub minutes: usize,
    pub hours: usize,
}

pub struct Config {
    pub timer: Timer,
}

impl Config {
    /// returns new `Config`
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        match args.next() {
            Some(arg) => Ok(Config::parse_arg(&arg).expect("Error parsing time string")),
            None => return Err("Didn't get a countdown"),
        }
    }

    /// Parses the argument into hours, minutes and seconds
    ///
    /// Returns `Config`
    ///
    /// The`parse_time_string` function will throw `ParserError`
    fn parse_arg(time_string: &str) -> Result<Config, ParserError> {
        let mut hours: usize = 0;
        let mut minutes: usize = 0;
        let mut seconds: usize = 0;

        let mut s = String::new();
        // slice string at h, m and s
        for char in time_string.chars() {
            match char {
                'h' => {
                    hours = s.parse::<usize>().unwrap();
                    s = String::new();
                }
                'm' => {
                    minutes = s.parse::<usize>().unwrap();
                    s = String::new();
                }
                's' => {
                    seconds = s.parse::<usize>().unwrap();
                    s = String::new();
                }
                _ => s.push(char),
            };
        }

        Ok(Config {
            timer: Timer {
                seconds,
                minutes,
                hours,
            },
        })
    }
}

impl Timer {
    /// counts down timer by one second
    ///
    /// Returns `false` when timer runs out
    fn tick(&mut self) -> bool {
        if self.seconds > 0 {
            self.seconds = self.seconds - 1;
        } else {
            if self.minutes > 0 {
                self.minutes = self.minutes - 1;
                self.seconds = 59;
            } else {
                if self.hours > 0 {
                    self.hours = self.hours - 1;
                    self.minutes = 59;
                    self.seconds = 59;
                } else {
                    return false;
                }
            }
        }

        true
    }

    /// Returns remaining time in clock format
    fn as_string(&self) -> String {
        let mut time_string = String::new();

        if self.hours > 0 {
            time_string.push_str(format!("{}h", self.hours).as_str());
        }

        if self.minutes > 0 {
            time_string.push_str(format!("{}m", self.minutes).as_str());
        }

        if self.seconds > 0 {
            time_string.push_str(format!("{}s", self.seconds).as_str());
        }

        time_string
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let font = FIGfont::standand().unwrap();
        let figure = font.convert(self.as_string().as_str());

        match figure {
            Some(s) => write!(f, "{}", s),
            None => write!(f, "")
        }
    }
}

pub fn run(config: Config) {
    let mut timer = config.timer;

    let timer_lib = TimerLib::new();

    let total_time = timer.seconds + timer.minutes * 60 + timer.hours * 60 * 60;

    println!("{}", timer);
    let _guard = timer_lib.schedule_repeating(chrono::Duration::seconds(1), move || {
        timer.tick();
        // clear screen
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", timer);
    });

    thread::sleep(Duration::from_secs(total_time as u64));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use more_asserts;

    #[test]
    fn test_tick() {
        // return false when timer runs out
        let mut timer = Timer {
            seconds: 0,
            minutes: 0,
            hours: 0,
        };

        assert_eq!(timer.tick(), false);

        // return true if timer isn't timed out
        let mut timer = Timer {
            seconds: 2,
            minutes: 0,
            hours: 0,
        };

        assert_eq!(timer.tick(), true);
    }

    #[test]
    fn ten_second_timer_should_run_for_around_ten_seconds() {
        let config = Config {
            timer: Timer {
                seconds: 10,
                minutes: 0,
                hours: 0,
            },
        };
        let required_time = Duration::from_secs(10);
        let delta = Duration::from_millis(10);
        let estimated_time = required_time + delta;

        let start_time = Instant::now();
        run(config);
        let elapsed_time = start_time.elapsed();

        more_asserts::assert_le!(elapsed_time, estimated_time);
    }
}
