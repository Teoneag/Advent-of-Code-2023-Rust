fn main() {
    let input = include_str!("../input.txt");

    let times = input.lines().nth(0).unwrap().split_whitespace().skip(1);
    let dis = input.lines().nth(1).unwrap().split_whitespace().skip(1);

    let t = times.collect::<String>().parse::<u128>().unwrap();

    let d = dis.collect::<String>().parse::<u128>().unwrap();

    let nr = (0..=t).filter(|x| (t - x) * x > d).count();

    println!("Nr: {}", nr); // 27102791
}
