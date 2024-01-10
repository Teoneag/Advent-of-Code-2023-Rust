fn get_dif(input: &[i32]) -> Vec<i32> {
    input.windows(2).skip(1).map(|x| x[1] - x[0]).collect()
}

fn get_prev(mut input: Vec<i32>) -> Vec<i32> {
    input.insert(0, input[0]);

    let dif = get_dif(&input);

    if dif.iter().all(|&x| x == 0) {
        return input;
    }

    let dif = get_prev(dif);
    input[0] -= dif[0];

    input
}

fn main() {
    let input = include_str!("../input.txt");

    let sum: i32 = input
        .lines()
        .map(|line| {
            get_prev(
                line.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )[0]
        })
        .sum();

    println!("{}", sum); // 1211
}
