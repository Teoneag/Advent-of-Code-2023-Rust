fn main() {
    let input = include_str!("../input.txt");
    let mut seeds: Vec<(u64, bool)> = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|x| (x.parse::<u64>().unwrap(), false))
        .collect();

    for part in input.split(":").skip(2) {
        let lines = part
            .lines()
            .filter(|x| !x.is_empty() && x.chars().next().unwrap().is_digit(10))
            .collect::<Vec<&str>>();
        seeds
            .iter_mut()
            .for_each(|(_, is_modified)| *is_modified = false);

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let nrs = line
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let start = nrs[1];
            let end = nrs[1] + nrs[2] - 1;

            seeds
                .iter_mut()
                .filter(|&&mut (x, is_modified)| !is_modified && x >= start && x <= end)
                .for_each(|(x, is_modified)| {
                    *x = nrs[0] + *x - nrs[1];
                    *is_modified = true;
                });

        }
    }
    
    println!("{}", seeds.iter().min_by_key(|(x, _)| x).unwrap().0); // 199602917
}
