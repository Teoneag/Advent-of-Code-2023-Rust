use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let re = Regex::new(r"Card\s+\d+: (.+) \| (.+)").unwrap();
    let mut multiplier = vec![1; input.lines().count()];
    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        let caps = re.captures(line).unwrap();
        let winning_nr = caps
            .get(1)
            .unwrap()
            .as_str()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let drawn_nr = caps
            .get(2)
            .unwrap()
            .as_str()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut matches = 0;
        for nr in winning_nr {
            if drawn_nr.contains(&nr) {
                matches += 1;
            }
        }
        // println!("matches: {}", matches);

        for j in i + 1..=matches + i {
            // println!("j: {}", j);
            multiplier[j] += multiplier[i];
        }

        // println!("multiplier: {:?}", multiplier);

        sum += multiplier[i];
    }
    println!("Sum: {}", sum); // 25231
}
