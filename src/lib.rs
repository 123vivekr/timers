use std::error::Error;
use std::env;
use std::fmt;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ParserError;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing arguments")
    }
}

pub struct Config {
    pub seconds: usize
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let seconds= match args.next() {
            Some(arg) => Config::parse_time_string(&arg).expect("Error parsing time string"),
            None => return Err("Didn't get a countdown"),
        };

        Ok(Config {
            seconds,
        })
    }

    /// Parses the time argument into hours, minutes and seconds
    /// 
    /// h -> hours
    /// m -> minutes
    /// s -> seconds
    ///
    /// Examples:
    /// 1h, 2m, 3s (integral)
    /// 1h2m, 10m10s (combined)
    ///
    /// The`parse_time_string` function will throw `ParserError`
    fn parse_time_string(time_string: &str) -> Result<usize, ParserError> {
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

        let seconds = hours * 60 * 60 + minutes * 60 + seconds;

        Ok(seconds)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    loop {
        thread::sleep(Duration::from_millis(1000));
        let elapsed_time = start_time.elapsed().as_secs() as usize;

        println!("{}", config.seconds - elapsed_time);

        // clear terminal
        print!("\x1B[2J\x1B[1;1H");

        if elapsed_time >= config.seconds {
            return Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time_string() {
        // checking with integral timestamp
        assert_eq!(Config::parse_time_string("1h").unwrap(), 60 * 60);
        assert_eq!(Config::parse_time_string("1m").unwrap(), 60);
        assert_eq!(Config::parse_time_string("1s").unwrap(), 1);

        // checking with combined timestamp
        assert_eq!(Config::parse_time_string("1h2m").unwrap(), 60 * 60 + 2 * 60);
        assert_eq!(Config::parse_time_string("10m10s").unwrap(), 10 * 60 + 10);
    }
}