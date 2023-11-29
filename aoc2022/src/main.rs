use clap::{Arg, Command};
use std::env;
use std::fs;
use std::time::{Duration, Instant};

struct MeasureTime {
    start: Instant,
}
impl MeasureTime {
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    pub fn duration(&self) -> String {
        let duration = self.start.elapsed();
        if duration < Duration::from_millis(1) {
            format!("{}μs", duration.as_micros())
        } else if duration < Duration::from_secs(1) {
            format!("{:.3}ms", duration.as_micros() as f64 / 1000.0)
        } else {
            format!("{:.3}s", duration.as_millis() as f64 / 1000.0)
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    // parse command line for day and test
    let matches = Command::new("AoC2022")
        .arg(
            Arg::new("test")
                .short('t')
                .required(false)
                .action(clap::ArgAction::SetTrue),
        )
        .arg(Arg::new("day").required(false))
        .get_matches();

    let test_mode = matches.get_one::<bool>("test").unwrap_or(&false);
    let mut all_days = false;
    let day = match matches.get_one::<String>("day") {
        Some(str_data) => str_data.parse::<u32>().unwrap(),
        None => {
            all_days = true;
            0
        }
    };

    let mut start = day;
    let mut end = day;
    if all_days {
        start = 1;
        end = 25;
    }

    let total = MeasureTime::start();
    for day in start..=end {
        let cwd = env::current_dir().unwrap();
        let filename = match test_mode {
            true => format!("day{:02}_test.txt", day),
            false => format!("day{:02}.txt", day),
        };
        let filename = cwd.join("aoc2022/inputs").join(filename);
        let contents = match fs::read_to_string(filename) {
            Ok(data) => data,
            Err(_) => break,
        };

        let day_fn = aoc2022::get_day(day);

        let pt1 = MeasureTime::start();
        println!("Day {}", day);
        print!("----- part 1:\t");
        println!("{}", day_fn.0(contents.clone()));
        println!("----- end part one in {}", pt1.duration());
        println!("");
        print!("----- part 2:\t");
        let pt2 = MeasureTime::start();
        println!("{}", day_fn.1(contents.clone()));
        println!("----- end part two in {}", pt2.duration());
    }
    println!("");
    println!("Finished in {}", total.duration());
    println!("");
}
