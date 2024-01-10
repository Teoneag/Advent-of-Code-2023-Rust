use std::collections::HashMap;
use std::thread;

fn process_line(line: &str) -> u64 {

    let mut parts = line.split_whitespace();

    let springs = parts.next().unwrap();
    let springs: Vec<&str> = springs.split('.').filter(|x| !x.is_empty()).collect();

    let nrs = parts.next().unwrap();
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

fn main() {
    let input = include_str!("../input.txt");

    let mut handles = vec![];
    let lines = input.lines();
    for line in lines {
        let handle = thread::spawn(move || process_line(line));
        handles.push(handle);
    }

    let mut sum = 0;
    for handle in handles {
        sum += handle.join().unwrap();
    }

    println!("Sum: {}", sum); // 7843
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new_indexes_0() {
        let nrs = vec![1, 1];

        let expected_result = {
            let mut hashmap = HashMap::new();
            hashmap.insert(0, 1);
            hashmap.insert(1, 3);
            hashmap.insert(2, 1);
            hashmap
        };

        let result = get_new_indexes("???", &nrs, 0);

        assert_eq!(result, Some(expected_result));
    }

    #[test]
    fn test_get_new_indexes_1() {
        let nrs = vec![1, 1];

        let expected_result = {
            let mut hashmap = HashMap::new();
            hashmap.insert(1, 1);
            hashmap.insert(2, 1);
            hashmap
        };

        let result = get_new_indexes("#??", &nrs, 0);

        assert_eq!(result, Some(expected_result));
    }

    #[test]
    fn test_get_new_indexes_2() {
        let nrs = vec![1, 1];

        let expected_result = {
            let mut hashmap = HashMap::new();
            hashmap.insert(1, 1);
            hashmap.insert(2, 1);
            hashmap
        };

        let result = get_new_indexes("??#", &nrs, 0);

        assert_eq!(result, Some(expected_result));
    }

    #[test]
    fn test_get_new_indexes_3() {
        let nrs = vec![1, 1];

        let expected_result = {
            let mut hashmap = HashMap::new();
            hashmap.insert(1, 1);
            hashmap
        };

        let result = get_new_indexes("?#?", &nrs, 0);

        assert_eq!(result, Some(expected_result));
    }

    #[test]
    fn test_get_new_indexes_4() {
        let nrs = vec![3];

        let expected_result = {
            let mut hashmap = HashMap::new();
            hashmap.insert(1, 1);
            hashmap
        };

        let result = get_new_indexes("###", &nrs, 0);

        assert_eq!(result, Some(expected_result));
    }
}