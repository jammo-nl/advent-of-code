use std::process::Command;

fn main() {
    let cmd = Command::new("cargo")
        .args(&[
            "run",
            "--release",
            "-p",
            "aoc2022",
            "--bin",
            "aoc2022",
            "--",
            "4",
        ])
        .output()
        .unwrap();
    let output = String::from_utf8(cmd.stdout).unwrap();
    println!("{}", output);
}
