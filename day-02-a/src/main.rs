use regex::Regex;
use std::cmp::max;

fn main() {
    let input = include_str!("../input.txt");
    let re = Regex::new(r"(\d+) (\w+),?").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let mut colors = [0; 3];

        for caps in re.captures_iter(line) {
            let nr = caps[1].parse::<i32>().unwrap();
            let index = match &caps[2] {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => continue,
            };
            colors[index] = max(colors[index], nr);
        }

        sum += colors.iter().product::<i32>();
    }

    println!("{}", sum); // 59795
}
