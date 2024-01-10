fn main() {
    let input = include_str!("../input.txt");

    let times = input.lines().nth(0).unwrap().split_whitespace().skip(1);
    let distances = input.lines().nth(1).unwrap().split_whitespace().skip(1);

    let mut product = 1;

    for (time, distance) in times.zip(distances) {
        let time: u32 = time.parse().unwrap();
        let distance: u32 = distance.parse().unwrap();

        let nr = (0..=time).filter(|x| ((time - x) * x > distance)).count();

        product *= nr;
    }

    println!("Product: {}", product); // 3316275
}
