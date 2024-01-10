// YOU NEED 34 GB of RAM to run this program, so it's not really a solution, but it works :D
// This is the error you get otherwise:
// memory allocation of 34359738368 bytes failed
// error: process didn't exit successfully: `target\debug\day-05-b.exe` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)

fn main() {
    let input = include_str!("../input.txt");

    let seeds_pairs: Vec<u64> = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let mut seeds:Vec<(u64, bool)> = Vec::new();
    for i in (0..seeds_pairs.len()).step_by(2) {
        let start = seeds_pairs.get(i).unwrap();
        let end = seeds_pairs.get(i + 1).unwrap();

        for j in *start..*start + *end {
            seeds.push((j, true));
        }
    }

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
