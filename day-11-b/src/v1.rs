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
                *i_mut += 1000000 - 1;
            }
        }
    }

    for (_, j_mut) in galaxies.iter_mut() {
        let j_coppy = j_mut.clone();
        
        for j in 0..j_coppy {
            if empty_cols[j] {
                *j_mut += 1000000 - 1;
            }
        }
    }

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

    println!("{}", sum); // 779032247216
}
