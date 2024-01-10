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
    let mut sum = 0;

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        
        let winning_nr = parse_nr(caps.get(1).unwrap().as_str());
        let drawn_nr = parse_nr(caps.get(2).unwrap().as_str());

        let points = winning_nr.iter().filter(|nr| drawn_nr.contains(nr)).count(); 

        sum += if points == 0 { 0 } else { 2u32.pow((points - 1) as u32) };
    }
    println!("Sum: {}", sum); // 25231
}
