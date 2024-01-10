use itertools::Itertools;
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
    let n: u128 = 1 << string.len();
    for i in 0..n {
        let result = (i as u128 & springs_mask) ^ values;
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

fn process_line(string: &str, line: &str) -> u64 {
    println!("{}{}", string, line);

    let mut parts = line.split_whitespace();

    let springs = parts.next().unwrap();
    let springs = (0..5).map(|_| springs).join("?");
    let springs: Vec<&str> = springs.split('.').filter(|x| !x.is_empty()).collect();

    let nrs = parts.next().unwrap();
    let nrs = (0..5).map(|_| nrs).join("0");
    let nrs: Vec<usize> = nrs.split(',').map(|nr| nr.parse().unwrap()).collect();

    let nr = get_nr_comb(&springs, 0, &nrs, 0);
    // println!("{}: {}", line, nr);

    nr
}

fn main() {
    let input = include_str!("../input.txt");

    let mut handles = vec![];
    let lines = input.lines();
    let n = lines.clone().count();
    for (i, line) in lines.enumerate() {
        let handle = thread::spawn(move || process_line(&format!("{}/{}: ", i, n), line));
        handles.push(handle);
    }

    let mut sum = 0;
    for handle in handles {
        sum += handle.join().unwrap();
    }

    println!("Sum: {}", sum);
    // breakes at line 267/1000
}
