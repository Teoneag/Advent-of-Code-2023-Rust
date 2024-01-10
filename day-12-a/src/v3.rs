// only slow: 54s

use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let mut sum = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let springs = parts.next().unwrap();
        let nrs: Vec<usize> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|nr| nr.parse().unwrap())
            .collect();

        let mut springs_mask = 0;

        let mut values = 0;

        for (i, c) in springs.chars().rev().enumerate() {
            if c == '?' {
                springs_mask |= 1 << i;
            } else if c == '#' {
                values |= 1 << i;
            }
        }

        
        let mut hash_set = HashSet::new();

        let n = 1 << springs.len();

        for i in 0..n {
            let result = (i as u32 & springs_mask) ^ values;
            hash_set.insert(result);
        }

        for hash in hash_set {
            let consecutive_hashes: Vec<usize> = format!("{:b}", hash)
                .split('0')
                .filter(|&s| !s.is_empty())
                .map(|s| s.len())
                .collect();

            if nrs == consecutive_hashes {
                sum += 1;
            }
        }
    }

    println!("Sum: {}", sum); // 7843 - 54.3s
}
