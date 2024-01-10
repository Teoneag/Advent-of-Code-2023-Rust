// idk how still too slow

use itertools::Itertools;
use std::collections::HashMap;
use std::thread;

fn process_line(string: &str, line: &str) -> u64 {
    println!("Start line: {}{}", string, line);

    let mut parts = line.split_whitespace();

    let springs = parts.next().unwrap();
    let springs = (0..5).map(|_| springs).join("?");
    let springs: Vec<&str> = springs.split('.').filter(|x| !x.is_empty()).collect();

    let nrs = parts.next().unwrap();
    let nrs = (0..5).map(|_| nrs).join(",");
    let nrs: Vec<usize> = nrs.split(',').map(|nr| nr.parse().unwrap()).collect();

    let nr = get_nr_comb(&springs, 0, &nrs, 0);

    nr
}

// returns HashMap with key: finishing index of nrs, value: nr of combinations
fn get_new_indexes(string: &str, nrs: &Vec<usize>, index: usize) -> Option<HashMap<usize, u64>> {
    let mut hash_map = HashMap::new();

    let n = string.len();
    let i_first_hash = string.find('#').unwrap_or(n);
    if index >= nrs.len() {
        if string.chars().all(|c| c == '?') {
            // we only have ?, so this is a case as well
            let entry = hash_map.entry(index).or_insert(0);
            *entry += 1;
            return Some(hash_map);
        }
        return None;
    }
    let current = nrs.get(index).unwrap();

    for i in 0..=i_first_hash {
        if i + current > n {
            if string.chars().all(|c| c == '?') {
                // we only have ?, so this is a case as well
                let entry = hash_map.entry(index).or_insert(0);
                *entry += 1;
            }
            break;
        }

        // we can fit at lease current
        if string.chars().nth(i + current).unwrap_or('.') == '#' {
            // we're in a middle of a hash
            continue;
        }

        if index + 1 == nrs.len() || i + current == n {
            // we're at the last index or we're at the last char
            if string.chars().skip(i + current).all(|c| c == '?') {
                // we only have ?, so this is a case as well
                let entry = hash_map.entry(index + 1).or_insert(0);
                *entry += 1;
            }
            continue;
        }

        let option = get_new_indexes(&string[i + current + 1..], nrs, index + 1);
        if let Some(new_hash_map) = option {
            for (key, value) in new_hash_map {
                let entry = hash_map.entry(key).or_insert(0);
                *entry += value;
            }
        }
    }

    Some(hash_map)
}

fn get_nr_comb(springs: &Vec<&str>, i_springs: usize, nrs: &Vec<usize>, i_nrs: usize) -> u64 {
    let option = get_new_indexes(springs.get(i_springs).unwrap(), nrs, i_nrs);
    if option.is_none() {
        return 0;
    }

    let hash_map = option.unwrap();
    // println!("hash_map: {:?}", hash_map);

    if i_springs + 1 == springs.len() {
        return hash_map
            .iter()
            .filter(|(key, _)| **key == nrs.len())
            .map(|(_, value)| value)
            .sum();
    }

    hash_map
        .iter()
        .map(|(key, value)| value * get_nr_comb(springs, i_springs + 1, nrs, *key))
        .sum()
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
    let mut nr = 0;
    for handle in handles {
        sum += handle.join().unwrap();
        nr += 1;
        println!("{}/{}: {}", nr, n, sum);
    }

    // 415

    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! hashmap {
        ($( $key:expr => $value:expr ),*) => {
            {
                let mut map = HashMap::new();
                $(
                    map.insert($key, $value);
                )*
                map
            }
        }
    }

    #[test]
    fn test_get_new_indexes_0() {
        let nrs = vec![1, 1];

        let expected_result = hashmap!(
            0 => 1,
            1 => 3,
            2 => 1
        );

        let result = get_new_indexes("???", &nrs, 0);

        assert_eq!(result, Some(expected_result));
    }

    #[test]
    fn test_get_new_indexes_1() {
        let nrs = vec![1, 1];

        let expected_result = hashmap!(
            1 => 1,
            2 => 1
        );

        let result = get_new_indexes("#??", &nrs, 0);

        assert_eq!(result, Some(expected_result));
    }

    #[test]
    fn test_get_new_indexes_2() {
        let nrs = vec![1, 1];

        let expected_result = hashmap!(
            1 => 1,
            2 => 1
        );

        let result = get_new_indexes("??#", &nrs, 0);

        assert_eq!(result, Some(expected_result));
    }

    #[test]
    fn test_get_new_indexes_3() {
        let nrs = vec![1, 1];

        let expected_result = hashmap!(
            1 => 1
        );

        let result = get_new_indexes("?#?", &nrs, 0);

        assert_eq!(result, Some(expected_result));
    }

    #[test]
    fn test_get_new_indexes_4() {
        let nrs = vec![3];

        let expected_result = hashmap!(
            1 => 1
        );

        let result = get_new_indexes("###", &nrs, 0);

        assert_eq!(result, Some(expected_result));
    }
}
