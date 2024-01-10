fn main() {
    let adiacent = vec![
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let input = include_str!("../input.txt");

    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let n = matrix.len();
    let m = matrix[0].len();

    let mut sum = 0;

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] != '*' {
                continue;
            }

            let mut values = std::collections::HashSet::<(usize, usize)>::new();

            for (x, y) in &adiacent {
                let i_new = (i as i32 + x) as usize;
                let j_new = (j as i32 + y) as usize;

                if i_new >= n || j_new >= m || !matrix[i_new][j_new].is_digit(10) {
                    continue;
                }

                let mut j_start = j_new;
                while j_start < m && matrix[i_new][j_start].is_digit(10) {
                    j_start = j_start.wrapping_sub(1);
                }
                j_start = j_start.wrapping_add(1);

                values.insert((i_new, j_start));
            }

            if values.len() != 2 {
                continue;
            }

            let mut product = 1;

            for (i, mut j) in values {
                let mut x = 0;
                while j < m && matrix[i][j].is_digit(10) {
                    x = x * 10 + matrix[i][j].to_digit(10).unwrap();
                    j = j.wrapping_add(1);
                }
                product *= x;
            }

            sum += product;
        }
    }

    println!("{}", sum); // 80703636
}
