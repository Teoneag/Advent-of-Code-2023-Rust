fn main() {
    let input = include_str!("../input.txt");

    let mut matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut empty_rows = vec![true; matrix.len()];
    let mut empty_cols = vec![true; matrix[0].len()];

    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, c) in row.iter_mut().enumerate() {
            if *c == '#' {
                empty_rows[i] = false;
                empty_cols[j] = false;
            }
        }
    }

    let n = matrix.len();

    for (i, ok) in empty_rows.iter().rev().enumerate() {
        if *ok {
            matrix.insert(n - i, vec!['.'; matrix[0].len()]);
        }
    }

    let m = matrix[0].len();

    for (j, ok) in empty_cols.iter().rev().enumerate() {
        if *ok {
            for row in matrix.iter_mut() {
                row.insert(m - j, '.');
            }
        }
    }

    // for row in matrix.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }

    let galaxies_indexes: Vec<_> = matrix
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect();

    let mut sum = 0;

    for (i, (i1, j1)) in galaxies_indexes.iter().enumerate() {
        for j in i + 1..galaxies_indexes.len() {
            let (i2, j2) = galaxies_indexes[j];
            sum += (*i1 as isize - i2 as isize).abs() + (*j1 as isize - j2 as isize).abs();
        }
    }

    println!("{}", sum); // 9805264
}
