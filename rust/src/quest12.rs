use ndarray::{ArrayView2, ArrayViewMut2};
use rayon::prelude::*;

use crate::{Quest, QuestResult, util::input_to_grid_mut};

pub const PARTS: Quest = [part1, part2, part3];

fn recurse(grid: &mut ArrayViewMut2<u8>, pos: [usize; 2]) -> usize {
    let x = grid[pos];

    if x == 0 {
        return 0;
    }

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

fn optimize_fireball(grid: ArrayView2<u8>) -> [usize; 2] {
    let (h, w) = grid.dim();

    (0..w)
        .into_par_iter()
        .flat_map(|j| (0..h).into_par_iter().map(move |i| [i, j]))
        .max_by_key(|&pos| {
            let mut grid_copy = grid.to_owned();

            recurse(&mut grid_copy.view_mut(), pos)
        })
        .unwrap()
}

fn part3(mut input: String) -> QuestResult {
    let input = unsafe { input.as_mut_vec() };
    input.push(b'\n');

    let mut grid = input_to_grid_mut(input);

    let mut optimizer_grid = grid.to_owned();
    let p1 = optimize_fireball(optimizer_grid.view());
    recurse(&mut optimizer_grid.view_mut(), p1);
    let p2 = optimize_fireball(optimizer_grid.view());
    recurse(&mut optimizer_grid.view_mut(), p2);
    let p3 = optimize_fireball(optimizer_grid.view());

    let ans = recurse(&mut grid, p1)
        + recurse(&mut grid, p2)
        + recurse(&mut grid, p3);

    QuestResult::Number(ans as i64)
}
