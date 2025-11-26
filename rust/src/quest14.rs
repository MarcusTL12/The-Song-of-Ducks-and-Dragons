use ndarray::{Array2, ArrayView2, ArrayViewMut2};

use crate::{Quest, QuestResult, util::input_to_grid_mut};

pub const PARTS: Quest = [part1, part2, part3];

fn do_round(mut grid: ArrayViewMut2<u8>) {
    let (h, w) = grid.dim();

    for i in 0..w {
        for j in 0..h {
            let num_diag: u8 = [[-1, -1], [-1, 1], [1, -1], [1, 1]]
                .into_iter()
                .filter_map(|[di, dj]| {
                    let new_i = (i as isize + di) as usize;
                    let new_j = (j as isize + dj) as usize;

                    grid.get([new_i, new_j]).map(|x| x & 1)
                })
                .sum();

            let x = &mut grid[[i, j]];

            match (*x ^ num_diag) & 1 {
                0 => *x |= 2,
                1 => *x &= !2,
                _ => unreachable!(),
            }
        }
    }

    for x in grid.iter_mut() {
        *x >>= 1;
    }
}

fn part1(mut input: String) -> QuestResult {
    let input = unsafe { input.as_mut_vec() };
    input.push(b'\n');

    let mut grid = input_to_grid_mut(input);

    for x in grid.iter_mut() {
        *x = match *x {
            b'.' => 0,
            b'#' => 1,
            _ => panic!(),
        }
    }

    let mut ans = 0;

    for _ in 0..10 {
        do_round(grid.view_mut());

        ans += grid.iter().map(|&x| x as u32).sum::<u32>()
    }

    QuestResult::Number(ans as i64)
}

fn part2(mut input: String) -> QuestResult {
    let input = unsafe { input.as_mut_vec() };
    input.push(b'\n');

    let mut grid = input_to_grid_mut(input);

    for x in grid.iter_mut() {
        *x = match *x {
            b'.' => 0,
            b'#' => 1,
            _ => panic!(),
        }
    }

    let mut ans = 0;

    for _ in 0..2025 {
        do_round(grid.view_mut());

        ans += grid.iter().map(|&x| x as u32).sum::<u32>()
    }

    QuestResult::Number(ans as i64)
}

fn look_for_pattern(grid: ArrayView2<u8>, pattern: ArrayView2<u8>) -> bool {
    pattern
        .indexed_iter()
        .all(|((k, l), &x)| grid[[13 + k, 13 + l]] == x)
}

fn part3(mut input: String) -> QuestResult {
    let input = unsafe { input.as_mut_vec() };
    input.push(b'\n');
    let mut pattern = input_to_grid_mut(input);
    for x in pattern.iter_mut() {
        *x = match *x {
            b'.' => 0,
            b'#' => 1,
            _ => panic!(),
        }
    }

    let mut grid = Array2::from_elem([34, 34], 0u8);

    let mut ans = 0;

    for _ in 0..1_000_000 {
        do_round(grid.view_mut());

        if look_for_pattern(grid.view(), pattern.view()) {
            ans += grid.iter().map(|&x| x as usize).sum::<usize>()
        }
    }

    QuestResult::Number(ans as i64)
}
