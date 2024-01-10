fn main() {
    let input = include_str!("../input.txt");
    let mut sum = 0;
    let all_digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let mut first = 0;
        let mut last = 0;
        for (i, c) in line.char_indices() {
            if c.is_digit(10) {
                if first == 0 {
                    first = c.to_digit(10).unwrap();
                }
                last = c.to_digit(10).unwrap();
            }
            let string = line[i..].to_string();

            let x = all_digits
                .iter()
                .position(|&digit| string.starts_with(digit));

            if x != None {
                if first == 0 {
                    first = x.unwrap() as u32;
                }
                last = x.unwrap() as u32;
            }
        }
        sum += first * 10 + last;
    }

    println!("{}", sum); // 55614
}
