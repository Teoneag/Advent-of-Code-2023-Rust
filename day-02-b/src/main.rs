use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let re = Regex::new(r"Game (\d+): (.+)").unwrap();
    let re2 = Regex::new(r"; |, ").unwrap();
    let re3 = Regex::new(r"(\d+) (.+)").unwrap();

    let mut sum = 0;

    'outer: for line in input.lines() {
        let parts = re.captures(line).unwrap();
        let id = parts.get(1).unwrap().as_str().parse::<i32>().unwrap();

        for part in re2.split(parts.get(2).unwrap().as_str()) {
            let words = re3.captures(part).unwrap();
            let nr = words.get(1).unwrap().as_str().parse::<i32>().unwrap();
            match words.get(2).unwrap().as_str() {
                "red" if nr > 12 => continue 'outer,
                "green" if nr > 13 => continue 'outer,
                "blue" if nr > 14 => continue 'outer,
                _ => {}
            }
        }

        sum += id;
    }
    println!("{}", sum); // 2617
}
