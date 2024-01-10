fn main() {
    let input = include_str!("../input.txt");

    let matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let m = matrix[0].len();

    let empty_rows: Vec<_> = matrix.iter().map(|r| r.iter().all(|&c| c == '.')).collect();

    let empty_cols: Vec<_> = (0..m).map(|j| matrix.iter().all(|r| r[j] == '.')).collect();

    let mut galaxies: Vec<_> = matrix
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect();

    for (i_mut, _) in galaxies.iter_mut() {
        let i_coppy = i_mut.clone();

        for i in 0..i_coppy {
            if empty_rows[i] {
                *i_mut += 1;
            }
        }
    }

    for (_, j_mut) in galaxies.iter_mut() {
        let j_coppy = j_mut.clone();

        for j in 0..j_coppy {
            if empty_cols[j] {
                *j_mut += 1;
            }
        }
    }

    let mut sum = 0;

    for (i, (i1, j1)) in galaxies.iter().enumerate() {
        for j in i + 1..galaxies.len() {
            let (i2, j2) = galaxies[j];
            sum += (*i1 as isize - i2 as isize).abs() + (*j1 as isize - j2 as isize).abs();
        }
    }

    println!("{}", sum); // 9805264
}
