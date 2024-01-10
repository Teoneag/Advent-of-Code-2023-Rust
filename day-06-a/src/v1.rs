fn main() {
    let input = include_str!("../input.txt");

    let times = input.lines().nth(0).unwrap().split_whitespace().skip(1);
    let distances = input.lines().nth(1).unwrap().split_whitespace().skip(1);

    // println!("Time: {}", times.next().unwrap());
    // println!("Distance: {}", distances.next().unwrap());

    let mut product = 1;

    for (time, distance) in times.zip(distances) {
        let time: u32 = time.parse().unwrap();
        let distance: u32 = distance.parse().unwrap();

        // println!("Time: {}", time);
        // println!("Distance: {}", distance);
        // v = d/t => t = d/v
        // total_t = x + d / x

        let mut nr = 0;
        for x in 0..=time {
            // x = button time = speed
            let d = (time - x) * x;
            if d > distance {
                nr += 1;
            }
        }

        product *= nr;
    }

    println!("Product: {}", product); // 3316275
}
