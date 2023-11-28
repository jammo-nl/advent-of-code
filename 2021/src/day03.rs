use crate::tools::read_lines;

pub fn run_all(input: &str) -> String {
    let part1 = part1(read_lines(input));
    let part2 = part2(read_lines(input));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

struct Diagnostics {
    bitcounter: Vec<u32>,
    linecounter: u32,
    numbers: Vec<u32>
}
impl Diagnostics {
    fn new() -> Self {
        Self{bitcounter:vec!(), linecounter:0, numbers: vec!()}
    }

    fn add_bit_string(&mut self, bits: &str) {
        self.linecounter += 1;
        let item: u32 = u32::from_str_radix(bits, 2).unwrap();
        self.numbers.push(item);
        for c in 0..bits.len() {
            if self.bitcounter.len() <= c {
                self.bitcounter.push(0);
            }
            if item & (1 << c) != 0 {
                self.bitcounter[c] += 1;
            }
        }
    }

    fn gamma(&self) -> u32 {
        let res: String = self.bitcounter.iter().rev().map(|t| { if *t > self.linecounter/2 { "1".to_string() } else {"0".to_string()}}).collect::<String>();
        u32::from_str_radix(&res, 2).unwrap()
    }

    fn epsilon(&self) -> u32 {
        let res: String = self.bitcounter.iter().rev().map(|t| { if *t > self.linecounter/2 { "0".to_string() } else {"1".to_string()}}).collect::<String>();
        u32::from_str_radix(&res, 2).unwrap()
    }

    fn power_consumption(&self) -> u32 {
        self.gamma() * self.epsilon()
    }

    fn bit_filter(&self, numbers: &[u32], most_common: bool) -> u32 {
        let mut m_numbers = numbers.to_owned();
        let mut current_bit = self.bitcounter.len();

        while m_numbers.len() > 1 {
            // loop over all numbers to determine the number of times bit x is 1
            let mut counter = 0;
            for num in &m_numbers {
                if num & (1 << (current_bit-1)) != 0 {
                    counter += 1;
                }
            }

            // determine the most common bit at this positon
            // multiply by 10 as workaround for float..
            let mut bit = !most_common;
            if counter*10 >= m_numbers.len()*10 / 2 {
                bit = !bit;
            }

            // ddelete items from the list
            m_numbers.retain(|num| {
                if num & (1 << (current_bit-1)) != 0 {
                    bit
                } else {
                    !bit
                }
            });

            current_bit -= 1;
        };

        m_numbers[0]
    }

    fn oxygen_generator_rating(&self) -> u32 {
        self.bit_filter(&self.numbers, true)
    }

    fn co2_scrubber_rating(&self) -> u32 {
        self.bit_filter(&self.numbers, false)
    }

    fn life_support_rating(&self) -> u32 {
        self.oxygen_generator_rating() * self.co2_scrubber_rating()
    }
}

pub fn part1(input: Vec<String>) -> u32 {
    let mut diag = Diagnostics::new();

    for line in input {
        diag.add_bit_string(&line);
    };

    diag.power_consumption()
}
pub fn part2(input: Vec<String>) -> u32 {
    let mut diag = Diagnostics::new();

    for line in input {
        diag.add_bit_string(&line);
    };

    diag.life_support_rating()
}
