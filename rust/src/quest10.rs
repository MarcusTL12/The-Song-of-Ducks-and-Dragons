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

fn part1(mut input: String) -> QuestResult {
    input.push('\n');
    let mut grid = input_to_grid_mut(unsafe { input.as_bytes_mut() });

    let ((i, j), _) = grid.indexed_iter().find(|&(_, &x)| x == b'D').unwrap();

    let sheep = recurse(&mut grid, 4, [i, j]);

    QuestResult::Number(sheep as i64)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
