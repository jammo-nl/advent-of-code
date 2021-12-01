use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _runner = match &args[1] {
        _ => panic!("Incorrect input")
    };
}
