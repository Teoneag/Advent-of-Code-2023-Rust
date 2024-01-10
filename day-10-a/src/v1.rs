// not working

fn main() {
    // .....
    // .F-7.
    // .|.|.
    // .L-J.
    // .....

    let pipes = [
        ('F', vec![(1, 0), (0, 1)]),
        ('-', vec![(0, -1), (0, 1)]),
        ('7', vec![(0, -1), (1, 0)]),
        ('|', vec![(-1, 0), (1, 0)]),
        ('L', vec![(-1, 0), (0, 1)]),
        ('J', vec![(0, -1), (-1, 0)]),
        ('S', vec![(0, -1), (0, 1), (-1, 0), (1, 0)]),
    ];

    let input = include_str!("../input.txt");

    let chars_matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut dis_matrix = chars_matrix
        .iter()
        .map(|line| line.iter().map(|_| -1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (i_start, j_start) = chars_matrix
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .enumerate()
                .find_map(|(j, &c)| if c == 'S' { Some((i, j)) } else { None })
        })
        .unwrap();

    dis_matrix[i_start][j_start] = 0;

    let mut queue = vec![((i_start, j_start), 0)];

    while let Some(((i, j), nr)) = queue.first() {
        let ((i, j), nr) = ((i.clone(), j.clone()), nr.clone());
        queue.remove(0);
        let c = chars_matrix[i][j];
        if c == '.' {
            continue;
        }

        dis_matrix[i][j] = nr;

        for (c_pipe, neighbour) in &pipes {
            if c != *c_pipe {
                continue;
            }

            for (di, dj) in neighbour {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni >= 0
                    && ni < chars_matrix.len() as i32
                    && nj >= 0
                    && nj < chars_matrix[0].len() as i32
                {
                    let (ni, nj) = (ni as usize, nj as usize);
                    if dis_matrix[ni][nj] == -1 {
                        queue.push(((ni, nj), nr + 1));
                    }
                }
            }
        }
    }

    println!(
        "{}",
        dis_matrix
            .iter()
            .map(|line| line
                .iter()
                .map(|&x| match x {
                    -1 => '.'.to_string(),
                    _ => x.to_string(),
                })
                .collect::<Vec<_>>()
                .join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    );

    let max = dis_matrix
        .iter()
        .map(|line| line.iter().max().unwrap())
        .max()
        .unwrap();

    println!("{}", max); // 7097 - correct
    // 7088 - too low
    // 14174 - incorect

    // let (i_max, j_max) = dis_matrix
    //     .iter()
    //     .enumerate()
    //     .find_map(|(i, line)| {
    //         line.iter()
    //             .enumerate()
    //             .find_map(|(j, c)| if c == max { Some((i, j)) } else { None })
    //     })
    //     .unwrap();

    // println!("{}, {}", i_max, j_max);

    // println!(
    //     "{} {} {}\n{} {} {}\n{} {} {}",
    //     dis_matrix[i_max - 1][j_max - 1],
    //     dis_matrix[i_max - 1][j_max],
    //     dis_matrix[i_max - 1][j_max + 1],
    //     dis_matrix[i_max][j_max - 1],
    //     dis_matrix[i_max][j_max],
    //     dis_matrix[i_max][j_max + 1],
    //     dis_matrix[i_max + 1][j_max - 1],
    //     dis_matrix[i_max + 1][j_max],
    //     dis_matrix[i_max + 1][j_max + 1]
    // );
}
