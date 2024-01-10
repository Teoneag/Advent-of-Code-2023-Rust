use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let re = Regex::new(r"Game (\d+): (.+)").unwrap();
    let re2 = Regex::new(r"; |, ").unwrap();
    let re3 = Regex::new(r"(\d+) (.+)").unwrap();

    let mut sum = 0;

    for line in input.lines() {
        let parts = re.captures(line).unwrap();
        let id = parts.get(1).unwrap().as_str();
        let id = id.parse::<i32>().unwrap();

        let mut ok = true;
        let parts = re2.split(parts.get(2).unwrap().as_str());
        for part in parts {
            let words = re3.captures(part).unwrap();
            let nr = words.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let colour = words.get(2).unwrap().as_str();
            match colour {
                "red" => {
                    if nr > 12 {
                        ok = false;
                        break;
                    }
                }
                "green" => {
                    if nr > 13 {
                        ok = false;
                        break;
                    }
                }
                "blue" => {
                    if nr > 14 {
                        ok = false;
                        break;
                    }
                }
                _ => {}
            }
        }

        if ok {
            sum += id;
        }
    }
    println!("{}", sum); // 2617
}
