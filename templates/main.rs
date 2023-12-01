use std::process::Command;

fn main() {
    let paths = std::fs::read_dir(format!("{}/src/bin/", env!("CARGO_MANIFEST_DIR"))).unwrap();
    let mut run = vec![];

    for path in paths {
        run.push(
            path.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".rs", ""),
        );
    }

    for day in run {
        let cmd = Command::new("cargo")
            .args(["run", "--release", "-p", "aoc{YEAR}", "--bin", &day])
            .output()
            .unwrap();

        println!("----------------------------------------------");
        println!("|              {}                         |", day);
        println!("----------------------------------------------");

        let output = String::from_utf8(cmd.stdout).unwrap();
        let is_empty = output.is_empty();

        println!(
            "{}",
            if is_empty {
                "Not solved."
            } else {
                output.trim()
            }
        )
    }
}
