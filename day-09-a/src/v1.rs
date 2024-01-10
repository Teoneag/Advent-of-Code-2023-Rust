fn get_dif(input: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 0..input.len() - 1 {
        result.push(input[i + 1] - input[i]);
    }

    result
}

fn get_next(input: Vec<i32>) -> Vec<i32> {
    let mut result = input.clone();
    let n = result.len();
    result.push(input[n - 1]);

    let dif = get_dif(&input);

    if dif.iter().all(|&x| x == 0) {
        return result;
    }

    let dif = get_next(dif);
    result[n] += dif[dif.len() - 1];

    result
}

fn main() {
    let input = include_str!("../input.txt");

    let mut sum = 0;

    for line in input.lines() {
        println!("{}", line);

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // println!("{:?}", get_next(numbers));
        sum += get_next(numbers).last().unwrap();
    }

    println!("{}", sum); // 1806615041
}
