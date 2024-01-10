fn main() {
    let input = include_str!("../input.txt");
    let mut sum = 0;

    for line in input.lines() {
        let mut first = 0;
        let mut last = 0;
        for c in line.chars() {
            if !c.is_digit(10) {
                continue;
            }
            if first == 0 {
                first = c.to_digit(10).unwrap();
            }
            last = c.to_digit(10).unwrap();
        }
        sum += first * 10 + last;
    }
    
    println!("{}", sum); // 55488
}
