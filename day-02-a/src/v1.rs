use std::cmp::max;

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let re = Regex::new(r"(\d+) (.+)").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let parts = line[8..].split("; ");
        let mut nr_red = 0;
        let mut nr_green = 0;
        let mut nr_blue = 0;
        for part in parts {
            let parts = part.split(", ");
            for part in parts {
                let words = re.captures(part).unwrap();
                let nr = words.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let colour = words.get(2).unwrap().as_str();
                match colour {
                    "red" => nr_red = max(nr_red, nr),
                    "green" => nr_green = max(nr_green, nr),
                    "blue" => nr_blue = max(nr_blue, nr),
                    _ => {}
                }
            }
        }
        sum += nr_red * nr_green * nr_blue;
    }

    println!("{}", sum); // 59795
}
