fn get_dif(input: &[i32]) -> Vec<i32> {
    input.windows(2).map(|x| x[1] - x[0]).collect()
}

fn get_next(mut input: Vec<i32>) -> Vec<i32> {
    let n = input.len();
    input.push(input[n - 1]);

    let dif = get_dif(&input);

    if dif.iter().all(|&x| x == 0) {
        return input;
    }

    let dif = get_next(dif);
    input[n] += dif[dif.len() - 1];

    input
}

fn main() {
    let input = include_str!("../input.txt");

    let sum: i32 = input
        .lines()
        .map(|line| {
            get_next(
                line.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )
            .last()
            .cloned()
            .unwrap()
        })
        .sum();

    println!("{}", sum); // 1806615041
}
