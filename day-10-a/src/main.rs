fn main() {
    let input = include_str!("../input.txt");

    let matrix: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().collect())
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

    let mut dir = {
        if matches!(matrix[i - 1][j], '|' | '7' | 'F') {
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

    let mut nr = 0;

    while matrix[i][j] != 'S' {
        nr += 1;
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

    println!("{}", nr / 2 + 1); // 7097
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
