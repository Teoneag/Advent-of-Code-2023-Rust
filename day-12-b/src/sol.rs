use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

fn main() {
    let input = include_str!("../input.txt");

    println!("Part two: {}", solve(parse_input_part_two(input))); 
    // 10153896718999
    // 235311470119

}

fn parse_input_part_two(input: &str) -> Vec<ConditionRecord> {
    input
        .lines()
        .map(unfold)
        .map(ConditionRecord::from)
        .collect()
}

fn unfold(input: &str) -> String {
    let mut parts = input.split_whitespace();

    let springs = parts.next().unwrap();
    let springs = (0..5).map(|_| springs).join("?");

    let groups = parts.next().unwrap();
    let groups = (0..5).map(|_| groups).join(",");

    format!("{} {}", springs, groups)
}

fn solve(records: Vec<ConditionRecord>) -> String {
    let mut cache: HashMap<ConditionRecord, usize> = HashMap::new();

    records
        .iter()
        .map(|c| c.possible_arrangements(&mut cache))
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ConditionRecord {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl From<&str> for ConditionRecord {
    
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();

        let springs: Vec<Spring> = parts.next().unwrap().chars().map(Spring::from).collect();
        let groups: Vec<usize> = parts
            .next()
            .unwrap()
            .split(",")
            .map(|c| c.parse().unwrap())
            .collect();

        ConditionRecord::new(springs, groups)
    }
}

impl From<String> for ConditionRecord {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl ConditionRecord {
    fn new(springs: Vec<Spring>, groups: Vec<usize>) -> Self {
        Self { springs, groups }
    }

    fn possible_arrangements(&self, cache: &mut HashMap<ConditionRecord, usize>) -> usize {
        if let Some(&solutions) = cache.get(self) {
            return solutions;
        }

        if self.groups.is_empty() {
            let v = match self.springs.iter().any(|c| *c == Spring::Damaged) {
                true => 0,
                false => 1,
            };

            cache.insert(self.clone(), v);

            return v;
        }

        let needed_space = self.groups.iter().sum::<usize>() + self.groups.len() - 1;
        if self.springs.len() < needed_space {
            cache.insert(self.clone(), 0);

            return 0;
        }

        let first = self.springs[0];
        if first == Spring::Operational {
            let result = Self::new(self.springs[1..].to_vec(), self.groups.clone())
                .possible_arrangements(cache);
            cache.insert(self.clone(), result);

            return result;
        }

        let group = self.groups[0];
        let are_all_non_operational = self.springs[..group]
            .iter()
            .all(|c| *c != Spring::Operational);
        let end = (group + 1).min(self.springs.len());

        let mut solutions: usize = 0;

        if are_all_non_operational
            && ((self.springs.len() > group && self.springs[group] != Spring::Damaged)
                || self.springs.len() <= group)
        {
            solutions += Self::new(self.springs[end..].to_vec(), self.groups[1..].to_vec())
                .possible_arrangements(cache);
        }

        if first == Spring::Unknown {
            solutions += Self::new(self.springs[1..].to_vec(), self.groups.clone())
                .possible_arrangements(cache);
        }

        cache.insert(self.clone(), solutions);

        solutions
    }
}

#[derive(PartialEq, Debug, Eq, Clone, Hash, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Damaged,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            _ => panic!("Could not resolve spring"),
        }
    }
}

impl Display for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Spring::Operational => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        };
        write!(f, "{}", c)
    }
}
