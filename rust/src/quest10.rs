use ndarray::ArrayViewMut2;

use crate::{Quest, QuestResult, util::input_to_grid_mut};

pub const PARTS: Quest = [part1, part2, part3];

fn recurse(
    grid: &mut ArrayViewMut2<u8>,
    n: usize,
    [i, j]: [usize; 2],
) -> usize {
    if n == 0 {
        return 0;
    }

    let mut sheep = 0;

    for [nx, ny] in [[1, 2], [2, 1]] {
        for sx in [-1, 1] {
            let dx = nx * sx;
            let x = (i as isize + dx) as usize;

            for sy in [-1, 1] {
                let dy = ny * sy;
                let y = (j as isize + dy) as usize;

                if let Some(tile) = grid.get_mut([x, y]) {
                    if *tile == b'S' {
                        *tile = b'X';
                        sheep += 1;
                    }
                    sheep += recurse(grid, n - 1, [x, y]);
                }
            }
        }
    }

    sheep
}

fn part1(input: String) -> QuestResult {
    let mut input = input.into_bytes();
    input.push(b'\n');
    let mut grid = input_to_grid_mut(&mut input);

    let ((i, j), _) = grid.indexed_iter().find(|&(_, &x)| x == b'D').unwrap();

    let sheep = recurse(&mut grid, 4, [i, j]);

    QuestResult::Number(sheep as i64)
}

fn propagate_dragon(grid: &mut ArrayViewMut2<u8>) -> usize {
    let (h, w) = grid.dim();

    for i in 0..h {
        for j in 0..w {
            if (grid[[i, j]] & 0b0000_0001) == 0 {
                continue;
            }

            grid[[i, j]] &= 0b1111_1110;

            for [nx, ny] in [[1, 2], [2, 1]] {
                for sx in [-1, 1] {
                    let dx = nx * sx;
                    let x = (j as isize + dx) as usize;

                    for sy in [-1, 1] {
                        let dy = ny * sy;
                        let y = (i as isize + dy) as usize;

                        if let Some(tile) = grid.get_mut([y, x]) {
                            *tile |= 0b0000_1000;
                        }
                    }
                }
            }
        }
    }

    let mut eaten = 0;

    for x in grid.iter_mut() {
        if (*x & 0b0000_1000) != 0 {
            if (*x & 0b0000_0110) == 0b0000_0010 {
                eaten += 1;
                *x = (*x & 0b1111_0101) | 0b0000_0001;
            } else {
                *x = (*x & 0b1111_0111) | 0b0000_0001;
            }
        }
    }

    eaten
}

fn propagate_sheep(grid: &mut ArrayViewMut2<u8>) -> usize {
    let (h, w) = grid.dim();

    let mut eaten = 0;

    for i in (1..h).rev() {
        for j in 0..w {
            let mut tile = grid[[i, j]];
            if (grid[[i - 1, j]] & 0b0000_0010) != 0 {
                tile |= 0b0000_0010;
                if (tile & 0b0000_0101) == 0b0000_0001 {
                    eaten += 1;
                    tile &= 0b1111_1101;
                }
            } else {
                tile &= 0b1111_1101;
            }

            grid[[i, j]] = tile;
        }
    }

    for j in 0..w {
        grid[[0, j]] &= 0b1111_1101;
    }

    eaten
}

fn part2(input: String) -> QuestResult {
    let mut input = input.into_bytes();
    input.push(b'\n');

    for x in &mut input {
        *x = match *x {
            b'.' => 0b0000_0000,
            b'D' => 0b0000_0001,
            b'S' => 0b0000_0010,
            b'#' => 0b0000_0100,
            _ => *x,
        }
    }

    let mut grid = input_to_grid_mut(&mut input);

    let mut eaten = 0;

    for _ in 0..20 {
        eaten += propagate_dragon(&mut grid);
        eaten += propagate_sheep(&mut grid);
    }

    // for r in grid.rows() {
    //     for x in r {
    //         print!(
    //             "{}",
    //             match x {
    //                 0b0000_0000 => '.',
    //                 0b0000_0001 => 'D',
    //                 0b0000_0010 => 'S',
    //                 0b0000_0011 => 'X',
    //                 0b0000_0100 => '#',
    //                 0b0000_0101 => 'd',
    //                 0b0000_0110 => 's',
    //                 0b0000_0111 => '+',
    //                 _ => '!',
    //             }
    //         )
    //     }
    //     println!();
    // }

    QuestResult::Number(eaten as i64)
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
