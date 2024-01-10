fn main() {
    let input = include_str!("../input.txt");

    let product: usize = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .zip(input.lines().nth(1).unwrap().split_whitespace().skip(1))
        .map(|(time, distance)| {
            let time: u32 = time.parse().unwrap();
            let distance: u32 = distance.parse().unwrap();

            (0..=time).filter(|x| (time - x) * x > distance).count()
        })
        .product();

    println!("Product: {}", product); // 3316275
}
