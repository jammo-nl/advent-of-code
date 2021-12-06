use crate::tools::read_lines;

pub fn run_all(input: &str) -> String {
    let part1 = part1(read_lines(input));
    let part2 = part2(read_lines(input));

    format!("
Result part 1: {}
Result part 2: {}
    ", part1, part2)
}

pub fn part1(input: Vec<String>) -> usize {
    let mut fish: Vec<u8> = input[0].split(",").map(|num| num.parse::<u8>().unwrap()).collect();

    // loop 80 days..
    for _ in 0..80 {
        let mut new_fish: Vec<u8> = vec!();
        for i in 0..fish.len() {
            if fish[i] == 0 {
                new_fish.push(8);
                fish[i] = 6;
            } else {
                fish[i] -= 1;
            }
        }
        fish.append(&mut new_fish);
    }

    fish.len()
}
pub fn part2(input: Vec<String>) -> usize {
    let fish: Vec<u8> = input[0].split(",").map(|num| num.parse::<u8>().unwrap()).collect();
    let mut fishes: [usize;9] = [0,0,0,0,0,0,0,0,0];

    // keep track of number of fishes by lifetime
    for f in fish {
        fishes[f as usize] += 1;
    }

    // loop 256 days..
    for _ in 0..256 {
        let f = fishes[0];
        for i in 1..9 {
            fishes[i-1] = fishes[i];
        }
        fishes[6] += f as usize;
        fishes[8] = f as usize;
    }

    println!("{:?}", fishes);
    fishes.iter().sum()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = "./inputs/day06_test.txt";
    
    use super::*;
    #[test]
    fn test_part1() {
        let output = part1(read_lines(INPUT));
        assert_eq!(output, 5934)
    }

    #[test]
    fn test_part2() {
        let output = part2(read_lines(INPUT));
        assert_eq!(output, 26984457539)
    }
}
