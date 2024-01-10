use regex::Regex;
use std::cmp::max;

fn main() {
    let input = include_str!("../input.txt");
    let re = Regex::new(r"; |, ").unwrap();
    let re2 = Regex::new(r"(\d+) (.+)").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let (mut nr_red, mut nr_green, mut nr_blue) = (0, 0, 0);

        for part in re.split(&line[8..]) {
            let words = re2.captures(part).unwrap();
            let nr = words.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let colour = words.get(2).unwrap().as_str();
            match colour {
                "red" => nr_red = max(nr_red, nr),
                "green" => nr_green = max(nr_green, nr),
                "blue" => nr_blue = max(nr_blue, nr),
                _ => {}
            }
        }
        
        sum += nr_red * nr_green * nr_blue;
    }

    println!("{}", sum); // 59795
}
