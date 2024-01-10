use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let re = Regex::new(r"Card\s+\d+: (.+) \| (.+)").unwrap();
    let mut sum = 0;
    
    for line in input.lines() {
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

        let mut points = 0;
        for nr in winning_nr {
            if !drawn_nr.contains(&nr) {
                continue;
            }

            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
        sum += points;
    }
    println!("Sum: {}", sum); // 25231
}
