fn main() {
    let input = include_str!("../input.txt");

    let mut seeds: Vec<(u64, bool)> = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|x| (x.parse().unwrap(), true))
        .collect();

    for part in input.split(':').skip(2) {
        let lines: Vec<&str> = part
            .lines()
            .filter(|x| x.chars().next().map_or(false, |c| c.is_digit(10)))
            .collect();

        seeds.iter_mut().for_each(|(_, ok)| *ok = true);

        for line in lines {
            let nrs: Vec<u64> = line.split(' ').map(|x| x.parse().unwrap()).collect();

            seeds
                .iter_mut()
                .filter(|&&mut (x, ok)| ok && x >= nrs[1] && x <= nrs[1] + nrs[2] - 1)
                .for_each(|(x, ok)| {
                    *x = nrs[0] + *x - nrs[1];
                    *ok = false;
                });
        }
    }

    println!("{}", seeds.iter().min_by_key(|(x, _)| x).unwrap().0); // 199602917
}
