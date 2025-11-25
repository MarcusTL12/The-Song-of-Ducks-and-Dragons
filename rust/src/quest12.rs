use ndarray::ArrayViewMut2;

use crate::{Quest, QuestResult, util::input_to_grid_mut};

pub const PARTS: Quest = [part1, part2, part3];

fn recurse(grid: &mut ArrayViewMut2<u8>, pos: [usize; 2]) -> usize {
    let x = grid[pos];
    grid[pos] = 0;

    let mut exploded = 0;

    for dir in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
        let mut new_pos = pos;

        for (q, dq) in new_pos.iter_mut().zip(&dir) {
            *q = (*q as isize + dq) as usize;
        }

        if let Some(&y) = grid.get(pos) {

        }
    }

    exploded
}

fn part1(mut input: String) -> QuestResult {
    let input = unsafe { input.as_mut_vec() };

    input.push(b'\n');
    let grid = input_to_grid_mut(input);

    todo!()
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
