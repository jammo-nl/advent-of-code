use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use itertools::Itertools;

#[derive(Debug)]
enum FsEntry {
    Dir(HashMap<String, Rc<RefCell<FsEntry>>>),
    File(u32),
}
impl FsEntry {
    fn get_dir_sizes(&self, dir: String, file: String, target: &mut HashMap<String, u32>) -> u32 {
        let mut total = 0;
        match self {
            FsEntry::Dir(data) => {
                let dir = if !file.is_empty() {
                    format!("{}{}/", dir, file)
                } else {
                    dir.to_string()
                };
                for (key, data) in data {
                    let size =
                        data.borrow()
                            .get_dir_sizes(dir.to_string(), key.to_string(), target);
                    total += size;

                    // set the total for this directory
                    target.insert(dir.to_string(), total);
                }
            }
            FsEntry::File(data) => {
                total += data;
            }
        };
        total
    }
}

pub fn part1(input: String) -> String {
    let fs = parse_fs(input);
    let mut data_map = HashMap::new();
    fs.borrow()
        .get_dir_sizes("/".to_string(), "".to_string(), &mut data_map);

    format!(
        "{}",
        data_map
            .values()
            .filter(|i| *i < &100000u32)
            .cloned()
            .sum::<u32>()
    )
}

pub fn part2(input: String) -> String {
    let fs = parse_fs(input);
    let mut data_map = HashMap::new();
    fs.borrow()
        .get_dir_sizes("/".to_string(), "".to_string(), &mut data_map);

    let total_space = 70000000u32;
    let needed_space = 30000000u32;
    let unused_space = total_space - data_map.get("/").unwrap();
    let required_space = needed_space - unused_space;

    let result = data_map
        .values()
        .filter(|i| *i >= &required_space)
        .cloned()
        .sorted()
        .collect::<Vec<u32>>()[0];

    format!("{result}")
}

fn parse_fs(input: String) -> Rc<RefCell<FsEntry>> {
    // start with a root directory.. this always exists..
    let root = Rc::new(RefCell::new(FsEntry::Dir(HashMap::new())));
    let mut fs_stack = vec![root.clone()];

    for line in input.lines() {
        if line.starts_with("$") {
            //println!("stack depth is now: {}", fs_stack.len());
            if line == "$ ls" {
                continue; // ignore
            }
            let dir_cmd = line.replace("$ cd ", "");
            if dir_cmd == "/" {
                // goto root
                fs_stack.truncate(1);
            } else if dir_cmd == ".." {
                // up 1 dir
                fs_stack.pop();
            } else {
                // goto dir...
                let current = fs_stack.pop().unwrap();
                match &mut *current.borrow_mut() {
                    FsEntry::Dir(data) => {
                        if !data.contains_key(&dir_cmd) {
                            data.insert(
                                dir_cmd.to_string(),
                                Rc::new(RefCell::new(FsEntry::Dir(HashMap::new()))),
                            );
                        }
                        fs_stack.push(current.clone());
                        fs_stack.push(data.get_mut(&dir_cmd).unwrap().clone());
                    }
                    _ => panic!("We need a directory..."),
                };
            }
        } else {
            // the only way we have something that is not a command is when it is a file..
            // goto dir...
            let current = fs_stack.pop().unwrap();
            match &mut *current.borrow_mut() {
                FsEntry::Dir(data) => {
                    if line.starts_with("dir") {
                        // println!("adding a directory...");
                    } else {
                        let line_data = line.split(" ").collect::<Vec<&str>>();
                        let size = line_data.get(0).unwrap().parse::<u32>().unwrap();
                        let filename = line_data.get(1).unwrap().to_string();
                        data.insert(
                            filename.to_string(),
                            Rc::new(RefCell::new(FsEntry::File(size))),
                        );
                    }
                    fs_stack.push(current.clone());
                }
                _ => panic!("We need a directory..."),
            };
        }
    }
    root
}
