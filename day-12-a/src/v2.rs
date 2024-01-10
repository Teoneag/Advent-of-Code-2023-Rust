// still slower than a snail on a coffee break

fn main() {
    let input = include_str!("../input.txt");

    let mut sum = 0;

    for line in input.lines() {
        let mut parts = line.split(" ");
        let springs: Vec<_> = parts.next().unwrap().chars().collect();
        let nrs: Vec<usize> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|nr| nr.parse().unwrap())
            .collect();

        let mut stack = vec![(0, springs.clone())];

        while let Some((i, mut springs)) = stack.pop() {
            if i == springs.len() {
                let springs: String = springs.iter().collect();
                let parts: Vec<usize> = springs
                    .split('.')
                    .map(|part| part.len())
                    .filter(|x| *x != 0)
                    .collect();
                if parts.eq(&nrs) {
                    sum += 1;
                }
                continue;
            }

            match springs[i] {
                '?' => {
                    springs[i] = '.';
                    stack.push((i + 1, springs.clone()));
                    springs[i] = '#';
                    stack.push((i + 1, springs));
                }
                _ => {
                    stack.push((i + 1, springs));
                }
            }
        }
    }

    println!("Sum: {}", sum); // 7843
}
