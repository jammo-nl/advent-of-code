use array_tool::vec::Intersect;

pub fn part1(input: String) -> String {
    let sections = parse_sections(input);
    format!("{}", sections.get_contained())
}

pub fn part2(input: String) -> String {
    let sections = parse_sections(input);
    format!("{}", sections.get_overlaps())
}

struct Sections {
    sections: Vec<SectionPair>,
}
impl Sections {
    fn new() -> Self {
        Self { sections: vec![] }
    }
    fn add(&mut self, section_pair: SectionPair) {
        self.sections.push(section_pair);
    }
    fn get_contained(&self) -> u32 {
        let mut count = 0;
        for section in &self.sections {
            if section.contained() {
                count += 1;
            }
        }
        count
    }
    fn get_overlaps(&self) -> u32 {
        let mut count = 0;
        for section in &self.sections {
            if section.overlaps() {
                count += 1;
            }
        }
        count
    }
}
struct SectionPair {
    team1: Vec<u32>,
    team2: Vec<u32>,
}
impl SectionPair {
    fn new(pairs: &str) -> Self {
        let parts: Vec<&str> = pairs.split(',').collect();
        let p1: Vec<u32> = parts[0]
            .split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|d| d.parse::<u32>().unwrap())
            .collect();
        let p2: Vec<u32> = parts[1]
            .split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|d| d.parse::<u32>().unwrap())
            .collect();

        Self {
            team1: (p1[0]..=p1[1]).collect(),
            team2: (p2[0]..=p2[1]).collect(),
        }
    }
    fn contained(&self) -> bool {
        let v = self.team1.intersect(self.team2.clone());
        // if the length is the same as team1 or team2 it means there is one contained by the other
        v.len() == self.team1.len() || v.len() == self.team2.len()
    }
    fn overlaps(&self) -> bool {
        let v = self.team1.intersect(self.team2.clone());
        // if the length is the same as team1 or team2 it means there is one contained by the other
        v.len() > 0
    }
}

fn parse_sections(input: String) -> Sections {
    let mut sections = Sections::new();
    for line in input.lines() {
        sections.add(SectionPair::new(line));
    }

    sections
}
