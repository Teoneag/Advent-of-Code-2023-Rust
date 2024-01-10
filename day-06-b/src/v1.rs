fn main() {
    let input = include_str!("../input.txt");

    let times = input.lines().nth(0).unwrap().split_whitespace().skip(1);
    let distances = input.lines().nth(1).unwrap().split_whitespace().skip(1);

    let time = times.collect::<Vec<_>>().join("").parse::<u128>().unwrap();

    let distance = distances
        .collect::<Vec<_>>()
        .join("")
        .parse::<u128>()
        .unwrap();

    let nr = (0..=time).filter(|x| (time - x) * x > distance).count();

    println!("Nr: {}", nr); // 27102791
}
