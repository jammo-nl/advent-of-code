use std::env;
use std::time::Instant;
use std::collections::BTreeMap;
mod tools;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    let now = Instant::now();

    let days = BTreeMap::from([
        ("day01", day01::run_all as for<'r> fn(&'r str) -> String),
        ("day02", day02::run_all),
        ("day03", day03::run_all),
        ("day04", day04::run_all),
        ("day05", day05::run_all),
        ("day06", day06::run_all),
        ("day07", day07::run_all),
        ("day08", day08::run_all),
        ("day09", day09::run_all),
        ("day10", day10::run_all),
    ]);

    let args: Vec<String> = env::args().collect();
    let mut input: &str = "";
    let mut day = "";

    if args.len() > 1 {
        day = &args[1];
        input = "./inputs/{DAY}.txt";
    }
    if args.len() > 2 {
        // if we have a argument, use the testfile, this way I can
        // switch between the real and test file for development / test puproses
        input = "./inputs/{DAY}_test.txt";
    }

    let mut result: String = "".into();
    if day == "all" {
        for (day, func) in days {
            let file = input.to_string().replace("{DAY}", day);
            println!("running day {} with file {}", day, file);
            result = format!("{}\n{}:{}", result, day, func(&file));
        }
    } else {
        let file = input.to_string().replace("{DAY}", day);
        println!("running day {} with file {}", day, file);
        result = days[day](&file);
    }

    println!("{}", result);
    println!("Finished in {} ms\n", now.elapsed().as_millis());
}
