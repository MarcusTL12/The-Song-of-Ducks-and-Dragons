use std::collections::VecDeque;

use crate::{
    Quest,
    QuestResult::{self, Number},
    util::{input_to_grid, input_to_grid_mut},
};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(mut input: String) -> QuestResult {
    input.push('\n');
    let grid = input_to_grid(input.as_bytes());

    let ans: u64 = grid
        .indexed_iter()
        .filter(|&(_, &x)| x == b'T')
        .map(|((y, x), _)| {
            grid.get([y, x + 1]).is_some_and(|&v| v == b'T') as u64
                + (!(x + y).is_multiple_of(2)
                    && grid.get([y + 1, x]).is_some_and(|&v| v == b'T'))
                    as u64
        })
        .sum();

    Number(ans as i64)
}

fn part2(input: String) -> QuestResult {
    let mut input = input.into_bytes();
    input.push(b'\n');
    let mut grid = input_to_grid_mut(&mut input);

    let s = grid
        .indexed_iter()
        .find_map(|(yx, &v)| (v == b'S').then_some(yx))
        .unwrap();

    let mut queue = VecDeque::new();
    queue.push_back((s, 0));

    while let Some((pos, d)) = queue.pop_front() {
        let dirs = if (pos.0 + pos.1).is_multiple_of(2) {
            [(0, -1), (0, 1), (-1, 0)]
        } else {
            [(0, -1), (0, 1), (1, 0)]
        };

        for dir in dirs {
            let newpos = (
                (pos.0 as isize + dir.0) as usize,
                (pos.1 as isize + dir.1) as usize,
            );

            match grid.get(newpos) {
                Some(b'T') => {
                    queue.push_back((newpos, d + 1));
                    grid[newpos] = b'#';
                }
                Some(b'E') => {
                    return Number(d + 1);
                }
                _ => {}
            }
        }
    }

    panic!()
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
