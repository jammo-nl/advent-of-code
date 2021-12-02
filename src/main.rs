use std::env;
mod tools;

mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();

    let runner = match args[1].as_str() {
        "day01" => day01::run_all(),
        "day02" => day02::run_all(),
        _ => panic!("Incorrect input")
    };

    println!("{}", runner);
}
