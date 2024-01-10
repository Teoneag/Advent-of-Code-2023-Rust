// in progress, theoretically crazy efficient
use std::collections::HashMap;

// fn get_nr_of_combinations_and_index_big(
//     springs: &Vec<&str>,
//     mut i: usize,
//     nrs: &Vec<usize>,
//     mut j: usize,
// ) -> (u64, usize) {
//     let mut i_c = 0;
//     let n = springs.get(i).unwrap().len();

//     // #??
//     // ?#?
//     // ??#
//     // #?#

//     // 1 2 3
//     // _ _ _ _ _ _ _ _ _ _

//     // while i_c + nrs.get(j).unwrap() < n {

//     // }

//     (0, j)
// }

fn get_new_indexes(string: &str, nrs: &Vec<usize>, index: usize) -> HashMap<usize, u64> {
    let n = string.len();
    let current = nrs.get(index).unwrap();

    print!("\n\nString: {}, index: {}", string, index);

    let mut hash_map = HashMap::new();
    if *current > n {
        println!("There is no more space");
        let entry = hash_map.entry(index).or_insert(0);
        *entry += 1;
        return hash_map;
    }

    let chars: Vec<_> = string.chars().collect();

    let mut i = 0;
    loop {
        print!("\ni: {}; ", i);
        let mut nr_min = 0;
        while nr_min < *current && nr_min + i < n {
            if *chars.get(nr_min + i).unwrap() == '?' {
                nr_min += 1;
            } else {
                while (*chars.get(nr_min + i).unwrap() == '#') && (nr_min + i < n) {
                    nr_min += 1;
                }
            }
        }

        if *chars.get(i + i).unwrap_or(&'.') == '#' {
            nr_min = 0;
        }

        print!("nr_min: {}; ", nr_min);

        let (new_index, new_string) = if nr_min == *current {
            println!("index + 1");
            if nr_min + i + 1 < n {
                (index + 1, &string[nr_min + i + 1..])
            } else {
                (index + 1, "")
            }
        } else {
            println!("index normal");
            (index, "")
        };

        let new_indexes = get_new_indexes(new_string, nrs, new_index);
        println!("new_indexes: {:?}", new_indexes);
        for (key, value) in new_indexes {
            let entry = hash_map.entry(key).or_insert(0);
            *entry += value;
        }

        if chars.get(i).unwrap_or(&'.') == &'#' {
            println!("break because of #");
            break;
        }
        i += 1;
        if i > n {
            break;
        }
    }

    hash_map
}

fn main() {
    let input = include_str!("../input.txt");

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let springs: Vec<&str> = parts.next().unwrap().split('.').collect();
        let nrs: Vec<usize> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|nr| nr.parse().unwrap())
            .collect();

        println!("springs: {:?}", springs);
        println!("nrs: {:?}", nrs);

        let map = get_new_indexes(&springs.get(0).unwrap(), &nrs, 0);

        println!("This is the map: {:?}", map);

        // 15 is the max number of consecutive hashes
        // let mut cache: HashMap<String, HashSet<i8, i8>> = HashMap::new(); // string, start_nr, nr_of_nr
        //                                                                   // ex: "???", (1, 2) for ???.### 1,1,3

        // let mut j = 0;
        // for i in 0..springs.len() {
        //     if !cache.contains_key(springs[i]) {
        //         // add
        //     }
        // }
    }
}
