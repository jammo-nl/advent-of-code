pub fn part1(input: String) -> String {
    let stacks = parse_stack_file(input, false);
    format!("{}", stacks.get_top_crates())
}

pub fn part2(input: String) -> String {
    let stacks = parse_stack_file(input, true);
    format!("{}", stacks.get_top_crates())
}

struct Stacks {
    data: Vec<Vec<char>>,
}
impl Stacks {
    fn new() -> Self {
        Self { data: vec![] }
    }
    fn setup_stacks(&mut self, data: &str) {
        // we have the definition as first now, so just process it
        let mut defined = false;
        for line in data.lines() {
            if !defined {
                for _ in 0..(line.len() + 1) / 4 {
                    self.data.push(vec![]);
                }
                defined = true;
            } else {
                for i in 0..self.data.len() {
                    let chr = line.as_bytes()[i * 4 + 1] as char;
                    if chr != ' ' {
                        self.data.get_mut(i).unwrap().push(chr)
                    }
                }
            }
        }
    }
    fn process(&mut self, line: &str, move_multi: bool) {
        let commands = line.split(' ').collect::<Vec<&str>>();
        let num = commands[1].parse::<usize>().unwrap();
        let from = commands[3].parse::<usize>().unwrap();
        let to = commands[5].parse::<usize>().unwrap();

        let orig = self.data.get_mut(from - 1).unwrap();
        let mut mv = orig.split_off(orig.len() - num);

        // simulate moving one at a time
        if !move_multi {
            mv.reverse();
        }

        self.data.get_mut(to - 1).unwrap().append(&mut mv);
    }
    fn get_top_crates(&self) -> String {
        let mut data = "".to_string();
        for stack in &self.data {
            data = format!("{}{}", data, stack.last().unwrap())
        }
        data
    }
}

fn parse_stack_file(input: String, move_multi: bool) -> Stacks {
    let mut stacks = Stacks::new();
    let mut processing_stacks = true;
    let mut stack: String = "".into();
    for line in input.lines() {
        // after an empty line start processing
        if line.is_empty() && processing_stacks {
            stacks.setup_stacks(&stack);
            processing_stacks = false;
            continue;
        }

        if processing_stacks {
            stack = format!("{}\n{}", line, stack);
        } else {
            stacks.process(line, move_multi);
        }
    }

    stacks
}
