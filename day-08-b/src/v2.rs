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

    let indexes_network: Vec<usize> = network
        .iter()
        .enumerate()
        .filter(|(_, (n, _, _))| n.ends_with("A"))
        .map(|(i, _)| i)
        .collect();

    let mut index_map = 0;

    let nrs: Vec<u128> = indexes_network
        .clone()
        .into_iter()
        .map(|mut index| {
            let mut nr = 0;
            while !network[index].0.ends_with("Z") {
                let (_, left, right) = network[index];

                match instructions.chars().nth(index_map).unwrap() {
                    'L' => index = network.iter().position(|(n, _, _)| n == &left).unwrap(),
                    'R' => index = network.iter().position(|(n, _, _)| n == &right).unwrap(),
                    _ => (),
                }

                index_map = (index_map + 1) % instructions.len();

                nr += 1;
            }
            nr
        })
        .collect();

    let mut lcm: u128 = 1;

    for nr in nrs {
        lcm = num::integer::lcm(lcm, nr);
    }

    println!("{}", lcm); // 14449445933179
}
