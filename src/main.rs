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
            format!("{:.2}us", duration.as_micros())
        } else if duration < Duration::from_secs(1) {
            format!("{:.2}ms", duration.as_millis())
        } else {
            format!("{:.2}s", duration.as_secs())
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
        .arg(Arg::new("day").required(true))
        .get_matches();

    let test_mode = matches.get_one::<bool>("test").unwrap_or(&false);
    let day = match matches.get_one::<String>("day") {
        Some(str_data) => str_data.parse::<u32>().unwrap(),
        None => panic!("Invalid day given"),
    };

    println!("Starting with Test: {:?} for day {:02}", test_mode, day);
    let total = MeasureTime::start();

    let cwd = env::current_dir().unwrap();
    let filename = match test_mode {
        true => format!("day{:02}_test.txt", day),
        false => format!("day{:02}.txt", day),
    };
    let filename = cwd.join("inputs").join(filename);
    println!("Reading file {}", filename.to_str().unwrap());

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let day_fn = aoc2022::get_day(day);

    let pt1 = MeasureTime::start();
    println!("----- part 1");
    day_fn.0(contents.clone());
    println!("----- end part one in {}", pt1.duration());

    println!("----- part 2");
    let pt2 = MeasureTime::start();
    day_fn.1(contents.clone());
    println!("----- end part two in {}", pt2.duration());

    println!("Finished in {}", total.duration());
}
