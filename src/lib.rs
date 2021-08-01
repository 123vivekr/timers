use std::error::Error;
use std::env;
use std::fmt;
use std::str;
use std::thread;
use std::time::{Duration};
use figlet_rs::FIGfont;

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
    pub hours: usize
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
                },
                'm' => {
                    minutes = s.parse::<usize>().unwrap();
                    s = String::new();
                },
                's' => {
                    seconds = s.parse::<usize>().unwrap();
                    s = String::new();
                },
                _ => s.push(char),
            };
        }

        Ok(Config {
            timer: Timer {
                seconds,
                minutes,
                hours
            }
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


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut time_left = config.timer.clone();
    loop {
        // clear terminal
        print!("\x1B[2J\x1B[1;1H");
        
        let font = FIGfont::standand().unwrap();
        let figure = font.convert(time_left.as_string().as_str());
        println!("{}", figure.unwrap());

        thread::sleep(Duration::from_millis(1000));

        // exit loop when timer runs out
        if !time_left.tick() {
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick() {
        // return false when timer runs out
        let mut timer = Timer {
            seconds:0,
            minutes:0,
            hours:0,
        };

        assert_eq!(timer.tick(), false);

        // return true if timer isn't timed out
        let mut timer = Timer {
            seconds:2,
            minutes:0,
            hours:0,
        };

        assert_eq!(timer.tick(), true);
    }
}