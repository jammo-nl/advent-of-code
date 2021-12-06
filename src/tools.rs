use std::fs;
use std::str::FromStr;

// return the file content as a vector of T (Where T can be any type that implements FromStr)
pub fn read_lines<T>(filename: &str) -> Vec<T>
    where T: FromStr {
    let content = read_file(filename);
    let mut lines: Vec<T> = vec!();
    for line in content.lines() {
        if let Ok(value) = line.parse::<T>()  {
            lines.push(value);
        }
    }
    lines
}

// return the file content as a string
pub fn read_file(filename: &str) -> String {
    
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}
