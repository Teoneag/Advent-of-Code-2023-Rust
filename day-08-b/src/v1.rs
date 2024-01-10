// takes forever

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

    let mut indexes_network: Vec<usize> = network
        .iter()
        .enumerate()
        .filter(|(_, (n, _, _))| n.ends_with("A"))
        .map(|(i, _)| i)
        .collect();

    // println!("{:?}", indexes_network);

    let mut index_map = 0;

    let mut nr = 0;

    loop {
        let mut ok = true;

        for index_network in indexes_network.clone() {
            if !network[index_network].0.ends_with("Z") {
                ok = false;
                break;
            }
        }

        if ok {
            break;
        }

        for (i, index_network) in indexes_network.clone().into_iter().enumerate() {
            let (_, left, right) = network[index_network];

            match instructions.chars().nth(index_map).unwrap() {
                'L' => {
                    indexes_network[i] = network.iter().position(|(n, _, _)| n == &left).unwrap()
                }
                'R' => {
                    indexes_network[i] = network.iter().position(|(n, _, _)| n == &right).unwrap()
                }
                _ => (),
            }
        }

        index_map = (index_map + 1) % instructions.len();

        nr += 1;
    }

    println!("{}", nr); 
}
