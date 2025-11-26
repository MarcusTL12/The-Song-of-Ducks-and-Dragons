use ndarray::ArrayViewMut2;

use crate::{Quest, QuestResult, util::input_to_grid_mut};

pub const PARTS: Quest = [part1, part2, part3];

fn recurse(grid: &mut ArrayViewMut2<u8>, pos: [usize; 2]) -> usize {
    let x = grid[pos];
    grid[pos] = 0;

    let mut exploded = 1;

    for dir in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
        let mut new_pos = pos;

        for (q, dq) in new_pos.iter_mut().zip(&dir) {
            *q = (*q as isize + dq) as usize;
        }

        if let Some(&y) = grid.get(new_pos)
            && y != 0
            && y <= x
        {
            exploded += recurse(grid, new_pos);
        }
    }

    exploded
}

fn part1(mut input: String) -> QuestResult {
    let input = unsafe { input.as_mut_vec() };

    input.push(b'\n');
    let mut grid = input_to_grid_mut(input);

    let ans = recurse(&mut grid, [0, 0]);

    QuestResult::Number(ans as i64)
}

fn part2(mut input: String) -> QuestResult {
    let input = unsafe { input.as_mut_vec() };

    input.push(b'\n');
    let mut grid = input_to_grid_mut(input);

    let (h, w) = grid.dim();

    let ans = recurse(&mut grid, [0, 0]) + recurse(&mut grid, [h - 1, w - 1]);

    QuestResult::Number(ans as i64)
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
