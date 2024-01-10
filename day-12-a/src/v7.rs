use std::collections::{HashMap, HashSet};
use std::thread;

// returns HashMap with key: finishing index of nrs, value: nr of combinations
fn get_new_indexes(string: &str, nrs: &Vec<usize>, index: usize) -> HashMap<usize, u64> {
    let mut springs_mask = 0;
    let mut values = 0;
    for (i, c) in string.chars().rev().enumerate() {
        if c == '?' {
            springs_mask |= 1 << i;
        } else if c == '#' {
            values |= 1 << i;
        }
    }

    let mut hash_set = HashSet::new();
    let n: i32 = 1 << string.len();
    for i in 0..n {
        let result = (i as u32 & springs_mask) ^ values;
        hash_set.insert(result);
    }

    let mut hash_map = HashMap::new();
    for hash in hash_set {
        let consecutive_hashes: Vec<_> = format!("{:b}", hash)
            .split('0')
            .filter(|&s| !s.is_empty())
            .map(|s| s.len())
            .collect();

        let i = consecutive_hashes.len();
        if index + i > nrs.len() {
            continue;
        }

        if &nrs[index..index + i] == consecutive_hashes {
            let entry = hash_map.entry(index + i).or_insert(0);
            *entry += 1;
        }
    }

    hash_map
}

fn get_nr_comb(springs: &Vec<&str>, i_springs: usize, nrs: &Vec<usize>, i_nrs: usize) -> u64 {
    let hash_map = get_new_indexes(springs.get(i_springs).unwrap(), nrs, i_nrs);

    if i_springs + 1 == springs.len() {
        return hash_map
            .iter()
            .filter(|(key, _)| **key == nrs.len())
            .map(|(_, value)| value)
            .sum();
    }

    hash_map
        .into_iter()
        .map(|(key, value)| value * get_nr_comb(springs, i_springs + 1, nrs, key))
        .sum()
}

fn process_line(line: &str) -> u64 {
    let mut parts = line.split_whitespace();
    
    let springs: Vec<&str> = parts
        .next()
        .unwrap()
        .split('.')
        .filter(|x| !x.is_empty())
        .collect();

    let nrs: Vec<usize> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|nr| nr.parse().unwrap())
        .collect();

    get_nr_comb(&springs, 0, &nrs, 0)
}

fn main() {
    let input = include_str!("../input.txt");

    let mut handles = vec![];
    for line in input.lines() {
        let handle = thread::spawn(move || process_line(line));
        handles.push(handle);
    }

    let mut sum = 0;
    for handle in handles {
        sum += handle.join().unwrap();
    }

    println!("Sum: {}", sum); // 7843 - 0.3 s
}
