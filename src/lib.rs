
use figlet_rs::FIGfont;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::str;
use std::thread;
use std::time::Duration;
use timer::Timer as TimerLib;

#[derive(Debug, Clone)]
pub struct ParserError;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing arguments")
    }
}

#[derive(Debug, Clone)]
struct Timer {
    seconds: usize,
    minutes: usize,
    hours: usize,
}

pub struct Config {
    timer: Timer,
    output_filename: Option<String>,
}

impl Config {
    /// Returns new `Config`
    pub fn new(time_string: String, output_filename: Option<String>) -> Config {
        // TODO: handle error gracefully
        let timer = Config::parse_arg(&time_string).expect("Error parsing time string");

        Config {
            timer,
            output_filename,
        }
    }

    /// Parses the time string into hours, minutes and seconds
    ///
    /// Returns `Config`
    ///
    /// The`parse_time_string` function will throw `ParserError`
    fn parse_arg(time_string: &str) -> Result<Timer, ParserError> {
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

        Ok(Timer {
            seconds,
            minutes,
            hours,
        })
    }
}

impl Timer {
    /// counts down timer by one second
    ///
    /// Returns `false` when timer runs out
    #[inline]
    fn tick(&mut self) -> bool {
        if self.seconds > 0 {
            self.seconds -= 1;
        } else if self.minutes > 0 {
            self.minutes -= 1;
            self.seconds = 59;
        } else if self.hours > 0 {
            self.hours -= 1;
            self.minutes = 59;
            self.seconds = 59;
        } else {
            return false;
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

    // Return total time in seconds
    fn total_time(&self) -> usize {
        self.hours * 3600 + self.minutes * 60 + self.seconds
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let font = FIGfont::standand().unwrap();
        let figure = font.convert(self.as_string().as_str());

        match figure {
            Some(s) => write!(f, "{}", s),
            None => write!(f, ""),
        }
    }
}

pub fn run(config: Config) {
    let timer_lib = TimerLib::new();
    let mut timer = config.timer;

    // create backup for output file
    let timer_backup = timer.clone();

    let total_time = timer.total_time();

    println!("{}", timer);
    let _guard = timer_lib.schedule_repeating(chrono::Duration::seconds(1), move || {
        timer.tick();
        // clear screen
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", timer);
    });

    thread::sleep(Duration::from_secs(total_time as u64));

    if let Some(output_filename) = config.output_filename {
        write_to_output_file(output_filename, timer_backup);
    }
}

fn write_to_output_file(output_filename: String, timer: Timer) {
    let end_time = chrono::Local::now();
    let start_time = end_time - chrono::Duration::seconds(timer.total_time() as i64);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(output_filename)
        .unwrap();

    if let Err(e) = writeln!(
        file,
        "{}:{}:{},{},{}",
        timer.hours,
        timer.minutes,
        timer.seconds,
        start_time.to_rfc3339(),
        end_time.to_rfc3339()
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::path::Path;
    use std::time::Instant;
    use tempfile::tempdir;

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
            output_filename: None,
        };
        let required_time = Duration::from_secs(10);
        let delta = Duration::from_millis(20);
        let estimated_time = required_time + delta;

        let start_time = Instant::now();
        run(config);
        let elapsed_time = start_time.elapsed();

        more_asserts::assert_le!(elapsed_time, estimated_time);
    }

    #[test]
    fn create_file_if_no_file_exists_and_output_filename_provided() {
        let dir = tempdir().unwrap();

        let output_filename = String::from("test_file.txt");
        let output_file_path = format!("{}/{}", dir.path().display(), output_filename);

        println!("{}", output_file_path);

        assert!(!Path::new(&output_file_path).exists());

        let config = Config {
            timer: Timer {
                seconds: 1, // arbitrary time length
                minutes: 0,
                hours: 0,
            },
            output_filename: Some(output_file_path.clone()),
        };

        run(config);

        assert!(Path::new(&output_file_path).exists());
    }
}
