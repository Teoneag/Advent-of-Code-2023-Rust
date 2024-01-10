fn nr_touches_char(matrix: &Vec<Vec<char>>, i: i32, j: i32, j_start: i32) -> bool {
    let indices = vec![(i, j_start - 1), (i, j)]
        .into_iter()
        .chain((j_start - 1..=j).flat_map(|k| vec![(i - 1, k), (i + 1, k)]))
        .filter(|&(i, j)| i >= 0 && j >= 0 && i < matrix.len() as i32 && j < matrix[0].len() as i32)
        .collect::<Vec<_>>();

    indices.iter().any(|&(i, j)| {
        let x = matrix[i as usize][j as usize];
        !x.is_digit(10) && x != '.'
    })
}

fn main() {
    let input = include_str!("../input.txt");
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;

    for (i, line) in matrix.iter().enumerate() {
        let mut nr = 0;
        let mut j_start = 0;

        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                if nr == 0 {
                    j_start = j;
                }
                nr = nr * 10 + c.to_digit(10).unwrap();

                if j != line.len() - 1 {
                    continue;
                }
            }

            if nr == 0 {
                continue;
            }

            if nr_touches_char(&matrix, i as i32, j as i32, j_start as i32) {
                sum += nr;
            }

            nr = 0;
        }
    }

    println!("{}", sum); // 539590
}
