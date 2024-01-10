fn main() {
    let input = include_str!("../input.txt");

    let mut matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let n = matrix.len();
    let m = matrix[0].len();

    let empty_rows: Vec<_> = matrix.iter().map(|r| r.iter().all(|&c| c == '.')).collect();

    let empty_cols: Vec<_> = (0..m).map(|j| matrix.iter().all(|r| r[j] == '.')).collect();

    for (i, ok) in empty_rows.iter().rev().enumerate() {
        if *ok {
            matrix.insert(n - i, vec!['.'; matrix[0].len()]);
        }
    }

    for (j, ok) in empty_cols.iter().rev().enumerate() {
        if *ok {
            for row in matrix.iter_mut() {
                row.insert(m - j, '.');
            }
        }
    }

    let galaxies: Vec<_> = matrix
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect();

    let sum: isize = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, &(i1, j1))| {
            galaxies[i + 1..]
                .iter()
                .map(move |&(i2, j2)| (i1, j1, i2, j2))
        })
        .map(|(i1, j1, i2, j2)| {
            (i1 as isize - i2 as isize).abs() + (j1 as isize - j2 as isize).abs()
        })
        .sum();

    println!("{}", sum); // 9805264
}
