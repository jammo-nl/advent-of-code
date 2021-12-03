use std::env;
mod tools;

mod day01;
mod day02;
mod day03;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input: String = "".into();
    let mut day = "";

    if args.len() > 1 {
        day = &args[1];
        input = format!("./inputs/{}.txt", day);
    }
    if args.len() > 2 {
        // if we have a argument, use the testfile, this way I can
        // switch between the real and test file for development / test puproses
        input = format!("./inputs/{}_test.txt", day);
    }

    println!("running with file {}", input);

    let runner = match day {
        "day01" => day01::run_all(&input),
        "day02" => day02::run_all(&input),
        "day03" => day03::run_all(&input),
        _ => panic!("Incorrect input")
    };

    println!("{}", runner);
}