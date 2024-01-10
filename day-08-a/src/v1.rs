fn main() {
    let regex = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    let input = include_str!("../input.txt");

    let mut lines = input.lines();

    let instructions = lines.next().unwrap();

    lines.next(); // Skip empty line

    let network: Vec<(&str, &str, &str)> = lines
        .map(|line| {
            let caps = regex.captures(line).unwrap();

            (
                caps.get(1).unwrap().as_str(),
                caps.get(2).unwrap().as_str(),
                caps.get(3).unwrap().as_str(),
            )
        })
        .collect();

    let mut index_network = network
        .iter()
        .position(|(name, _, _)| name == &"AAA")
        .unwrap();

    let mut index_map = 0;

    let mut nr = 0;

    while network[index_network].0 != "ZZZ" {
        let (name, left, right) = network[index_network];

        match instructions.chars().nth(index_map).unwrap() {
            'L' => index_network = network.iter().position(|(n, _, _)| n == &left).unwrap(),
            'R' => index_network = network.iter().position(|(n, _, _)| n == &right).unwrap(),
            _ => (),
        }

        index_map += 1;
        if index_map == instructions.len() {
            index_map = 0;
        }

        nr += 1;
    }

    println!("{}", nr); // 18023
}
