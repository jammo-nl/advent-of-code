#[derive(Debug, Clone)]
struct Vec2 {
    x: usize,
    y: usize,
}
impl Vec2 {
    fn distance(&self, target: &Vec2) -> usize {
        let p1 = if target.x > self.x {
            target.x - self.x
        } else {
            self.x - target.x
        };
        let p2 = if target.y > self.y {
            target.y - self.y
        } else {
            self.y - target.y
        };
        p1 + p2
    }
}

fn expand(
    galaxies: &Vec<Vec2>,
    empty_lines: &Vec<usize>,
    empty_cols: &Vec<usize>,
    grow_incr: usize,
) -> Vec<Vec2> {
    galaxies
        .clone()
        .iter_mut()
        .map(|i| Vec2 {
            x: i.x + empty_cols.iter().filter(|item| **item < i.x).count() * (grow_incr - 1),
            y: i.y + empty_lines.iter().filter(|item| **item < i.y).count() * (grow_incr - 1),
        })
        .collect()
}
#[aoc2023::main(11)]
fn solution(input: &str) -> (usize, usize) {
    let mut galaxies = vec![];
    let mut empty_lines = vec![];
    let mut empty_cols = vec![];
    let mut occupied_cols = vec![];
    let mut total_cols = 0;
    for (y, line) in input.lines().enumerate() {
        total_cols = line.len();
        for (x, item) in line.chars().enumerate() {
            if item == '#' {
                galaxies.push(Vec2 { x, y });
                if !occupied_cols.contains(&x) {
                    occupied_cols.push(x);
                }
            }
        }

        if line.replace('.', "").is_empty() {
            empty_lines.push(y);
        }
    }
    for i in 0..total_cols {
        if !occupied_cols.contains(&i) {
            empty_cols.push(i);
        }
    }

    let expanded = expand(&galaxies, &empty_lines, &empty_cols, 2);
    let total_p1 = expanded
        .iter()
        .enumerate()
        .map(|(num, item)| {
            let mut totals = 0;
            for sub in num..expanded.len() {
                totals += item.distance(&expanded[sub]);
            }
            totals
        })
        .sum();

    let expanded = expand(&galaxies, &empty_lines, &empty_cols, 1000000);
    let total_p2 = expanded
        .iter()
        .enumerate()
        .map(|(num, item)| {
            let mut totals = 0;
            for sub in num..expanded.len() {
                totals += item.distance(&expanded[sub]);
            }
            totals
        })
        .sum();

    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

    #[test]
    fn test_solution1() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).0, 374);
        Ok(())
    }

    #[test]
    fn test_solution2() -> Result<(), String> {
        assert_eq!(solution(TEST_INPUT.trim()).1, 82000210);
        Ok(())
    }
}
