use std::collections::HashMap;

use bitarray::BitArray;
use ndarray::ArrayViewMut2;

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

// fn look_for_pattern(grid: ArrayView2<u8>, pattern: ArrayView2<u8>) -> bool {
//     pattern.indexed_iter().all(|((k, l), &x)| {
//         *unsafe { grid.get([13 + k, 13 + l]).unwrap_unchecked() } == x
//     })
// }

const W: usize = 34;
const N: usize = (W * W).div_ceil(usize::BITS as usize);

const RIGHT_PAD: BitArray<N> = BitArray([
    0xfffffffbfffffffe,
    0xffffffbfffffffef,
    0xfffffbfffffffeff,
    0xffffbfffffffefff,
    0xfffbfffffffeffff,
    0xffbfffffffefffff,
    0xfbfffffffeffffff,
    0xbfffffffefffffff,
    0xfffffffeffffffff,
    0xffffffeffffffffb,
    0xfffffeffffffffbf,
    0xffffeffffffffbff,
    0xfffeffffffffbfff,
    0xffeffffffffbffff,
    0xfeffffffffbfffff,
    0xeffffffffbffffff,
    0xffffffffbfffffff,
    0xfffffffbfffffffe,
    0xf,
]);

const LEFT_PAD: BitArray<N> = BitArray([
    0xfffffffdffffffff,
    0xffffffdffffffff7,
    0xfffffdffffffff7f,
    0xffffdffffffff7ff,
    0xfffdffffffff7fff,
    0xffdffffffff7ffff,
    0xfdffffffff7fffff,
    0xdffffffff7ffffff,
    0xffffffff7fffffff,
    0xfffffff7fffffffd,
    0xffffff7fffffffdf,
    0xfffff7fffffffdff,
    0xffff7fffffffdfff,
    0xfff7fffffffdffff,
    0xff7fffffffdfffff,
    0xf7fffffffdffffff,
    0x7fffffffdfffffff,
    0xfffffffdffffffff,
    0x7,
]);

const GRID_PAD: BitArray<N> = BitArray([
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xf,
]);

fn do_round_static(grid: BitArray<N>) -> BitArray<N> {
    let diag1 = (grid << (W + 1) as u32) & RIGHT_PAD;
    let diag2 = (grid << (W - 1) as u32) & LEFT_PAD;
    let diag3 = (grid >> (W - 1) as u32) & RIGHT_PAD;
    let diag4 = (grid >> (W + 1) as u32) & LEFT_PAD;

    (!grid ^ diag1 ^ diag2 ^ diag3 ^ diag4) & GRID_PAD
}

fn part3(input: String) -> QuestResult {
    let mut grid = BitArray::<N>::new();
    let mut pattern = BitArray::<N>::new();
    let mut stencil = BitArray::<N>::new();

    for (i, l) in input.lines().enumerate() {
        for (j, x) in l.chars().enumerate() {
            pattern.set((i + 13) * W + j + 13, x == '#');
            stencil.set((i + 13) * W + j + 13, true);
        }
    }

    let mut total = 0;

    let mut seen = HashMap::new();

    seen.insert(grid, (0, total));

    let mut i = 0;

    const TARGET: usize = 1_000_000_000;

    let mut has_warped = false;

    while i < TARGET {
        grid = do_round_static(grid);

        if (grid & stencil) == pattern {
            total += grid.count_ones();
        }

        i += 1;

        if !has_warped && let Some((j, then_total)) = seen.get(&grid) {
            has_warped = true;
            let batch_size = i - j;
            let batch_gain = total - then_total;

            let rounds_left = TARGET - i;
            let full_batches = rounds_left / batch_size;

            i += full_batches * batch_size;
            total += full_batches * batch_gain;
        }

        seen.insert(grid, (i, total));
    }

    QuestResult::Number(total as i64)
}
