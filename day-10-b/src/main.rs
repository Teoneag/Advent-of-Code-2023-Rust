// theoretically should replace S with what is it (because of this changing line 76 to "J", "L", "|" breakes it)
fn main() {
    let input = include_str!("../input.txt");

    let matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut pipes: Vec<Vec<_>> = matrix
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();

    let (mut i, mut j) = matrix
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, &c)| if c == 'S' { Some((i, j)) } else { None })
        })
        .unwrap();

    pipes[i][j] = true;

    let mut dir = {
        if i > 1 && matches!(matrix[i - 1][j], '|' | '7' | 'F') {
            i -= 1;
            Dir::Up
        } else if matches!(matrix[i + 1][j], '|' | 'L' | 'J') {
            i += 1;
            Dir::Down
        } else {
            j -= 1;
            Dir::Left
        }
    };

    while matrix[i][j] != 'S' {
        pipes[i][j] = true;
        match (matrix[i][j], &dir) {
            ('|', Dir::Up) => i -= 1,
            ('|', Dir::Down) => i += 1,
            ('-', Dir::Left) => j -= 1,
            ('-', Dir::Right) => j += 1,
            ('L', Dir::Down) | ('F', Dir::Up) => {
                j += 1;
                dir = Dir::Right;
            }
            ('L', Dir::Left) | ('J', Dir::Right) => {
                i -= 1;
                dir = Dir::Up;
            }
            ('7', Dir::Up) | ('J', Dir::Down) => {
                j -= 1;
                dir = Dir::Left;
            }
            ('7', Dir::Right) | ('F', Dir::Left) => {
                i += 1;
                dir = Dir::Down;
            }
            _ => unreachable!(),
        }
    }

    let mut is_inside = false;
    let mut nr = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let c = matrix[i][j];

            if j == 0 {
                is_inside = false;
            }

            if pipes[i][j] && matches!(c, '|' | 'J' | 'L') {
                is_inside = !is_inside;
            }

            if is_inside && !pipes[i][j] && j != (matrix[i].len() - 1) {
                nr += 1;
            }
        }
    }

    println!("{}", nr); // 355
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
