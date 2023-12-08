use std::collections::HashMap;
use std::ops::Range;

struct Convert {
    tables: HashMap<String, (Vec<Range<usize>>, Vec<Range<usize>>)>,
    list: Vec<(Vec<Range<usize>>, Vec<Range<usize>>)>,
}
impl Convert {
    fn new() -> Self {
        Self {
            tables: HashMap::new(),
            list: vec![],
        }
    }
    fn create_translation_list(&mut self) {
        let conv = [
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ];
        for conv in conv {
            self.list.push(self.tables.get(conv).unwrap().clone());
        }
    }
    fn get_seed_location(&self, seed: usize) -> usize {
        let mut target = seed;
        for conv in &self.list {
            target = self.convert_item(target, conv);
        }
        target
    }
    fn add_mapping(&mut self, target: &str, start_in: usize, start_out: usize, size: usize) {
        let from = start_in..start_in + size + 1;
        let to = start_out..start_out + size + 1;
        if !self.tables.contains_key(target) {
            self.tables.insert(target.to_string(), (vec![], vec![]));
        }

        if let Some(item) = self.tables.get_mut(target) {
            item.0.push(from);
            item.1.push(to);
        }
    }
    fn convert(input: usize, from: &Range<usize>, to: &Range<usize>) -> Result<usize, usize> {
        if from.contains(&input) {
            Ok(to.start + (input - from.start))
        } else {
            Err(input)
        }
    }
    fn convert_item(
        &self,
        input: usize,
        converter: &(Vec<Range<usize>>, Vec<Range<usize>>),
    ) -> usize {
        for i in 0..converter.0.len() {
            match Convert::convert(input, &converter.0[i], &converter.1[i]) {
                Ok(result) => return result,
                Err(_) => (),
            }
        }
        input
    }
}
#[aoc2023::main(05)]
fn solution(input: &str) -> (usize, usize) {
    let mut total_p1 = usize::MAX;
    let mut total_p2 = usize::MAX;

    let mut seeds: Vec<usize> = vec![];
    let mut converter = Convert::new();

    let mut process = "".to_string();
    for line in input.lines() {
        if line.starts_with("seeds:") {
            seeds = line
                .replace("seeds:", "")
                .split(' ')
                .filter_map(|i| i.parse().ok())
                .collect();
        } else if line.ends_with(" map:") {
            process = line.replace(" map:", "");
        } else if !line.is_empty() {
            let input: Vec<usize> = line.split(' ').filter_map(|i| i.parse().ok()).collect();
            converter.add_mapping(&process, input[1], input[0], input[2])
        }
    }
    converter.create_translation_list();

    // process lines
    for seed in &seeds {
        let loc = converter.get_seed_location(*seed);
        if loc < total_p1 {
            total_p1 = loc;
        }
    }

    for (idx, start) in seeds.iter().enumerate() {
        if idx % 2 == 0 {
            for seed in *start..(*start + seeds[idx + 1]) {
                let loc = converter.get_seed_location(seed);
                if loc < total_p2 {
                    total_p2 = loc;
                }
            }
        }
    }

    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 35);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 46);
        Ok(())
    }
}
