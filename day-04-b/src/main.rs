use regex::Regex;

fn parse_nr(s: &str) -> Vec<u32> {
    s.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn main() {
    let input = include_str!("../input.txt");
    let re = Regex::new(r"Card\s+\d+: (.+) \| (.+)").unwrap();
    let mut multiplier = vec![1; input.lines().count()];
    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        let caps = re.captures(line).unwrap();

        let winning_nr = parse_nr(caps.get(1).unwrap().as_str());
        let drawn_nr = parse_nr(caps.get(2).unwrap().as_str());

        let matches = winning_nr.iter().filter(|nr| drawn_nr.contains(nr)).count();

        for j in i + 1..=matches + i {
            multiplier[j] += multiplier[i];
        }

        sum += multiplier[i];
    }
    println!("Sum: {}", sum); // 9721255
}
