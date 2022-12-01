pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(day: u32) -> (DayFn, DayFn) {
    return match day {
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}
