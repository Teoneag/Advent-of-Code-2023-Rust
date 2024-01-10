fn transform_ranges(
    ranges: Vec<(i128, i128)>,
    transforms: Vec<(i128, i128, i128)>,
) -> Vec<(i128, i128)> {
    let mut results: Vec<(i128, i128)> = Vec::new();

    for range in ranges {
        let mut stack: Vec<(i128, i128)> = Vec::new();
        stack.push(range);
        for transform in &transforms {
            if stack.is_empty() {
                break;
            }
            let (start, end) = stack.pop().unwrap();

            if start > transform.1 || end < transform.0 {
                stack.push((start, end));
                continue;
            }

            if start >= transform.0 && end <= transform.1 {
                results.push((transform.2 + start, end + transform.2));
                continue;
            }

            if start < transform.0 && end > transform.1 {
                results.push((start + transform.2, end + transform.2));
                stack.push((start, transform.0 - 1));
                stack.push((transform.1 + 1, end));
                continue;
            }

            if start >= transform.0 && end > transform.1 {
                results.push((start + transform.2, transform.1 + transform.2));
                stack.push((transform.1 + 1, end));
                continue;
            }

            results.push((transform.0 + transform.2, end + transform.2));
            stack.push((start, transform.0 - 1));
        }
        for range in stack {
            results.push(range);
        }
    }

    results
}

fn main() {
    let input = include_str!("../input.txt");

    let seeds: Vec<i128> = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let mut seeds_pairs: Vec<(i128, i128)> = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect();

    for part in input.split(':').skip(2) {
        let lines: Vec<&str> = part
            .lines()
            .filter(|x| x.chars().next().map_or(false, |c| c.is_digit(10)))
            .collect();

        let transforms: Vec<(i128, i128, i128)> = lines
            .iter()
            .map(|line| {
                let nrs: Vec<i128> = line.split(' ').map(|x| x.parse().unwrap()).collect();
                (nrs[1], nrs[1] + nrs[2] - 1, nrs[0] - nrs[1])
            })
            .collect();

        seeds_pairs = transform_ranges(seeds_pairs, transforms);
    }

    println!(
        "{}",
        seeds_pairs
            .iter()
            .filter(|(x, _)| *x > 0)
            .min_by_key(|(x, _)| x)
            .unwrap()
            .0
    ); // 2254686
}
