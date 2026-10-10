use std::collections::VecDeque;

use ndarray::Array2;

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

fn pack(h: usize, [p, y, x]: [usize; 3]) -> [usize; 2] {
    if p == 0 {
        [y, x]
    } else {
        [h - 1 - y, h - 1 - x]
    }
}

fn rotate(h: usize, [y, x]: [usize; 2]) -> [usize; 2] {
    if x + y < h {
        // y * yvec + x * xvec -> [0, h - 1] + y * (-xvec) + x * zvec
        // yvec = [1, 0]
        // xvec = [0, 1]
        // zvec = [1, -1]

        // y * yvec + x * xvec = [y, x]
        // [0, h - 1] - y * (-xvec) + x * zvec = [x, h - 1 - x - y]

        [x, h - 1 - x - y]
    } else {
        // u = h - 1 - x
        // v = h - 1 - y
        // [h - 1, h - 1] + u * uvec + v * vvec
        // -> [h - 1, 1] + u * wvec + v * (-uvec)

        // uvec = [0, -1]
        // vvec = [-1, 0]
        // wvec = [-1, 1]

        // [h - 1, h - 1] + u * uvec + v * vvec =
        // [h - 1 - v, h - 1 - u] = [y, x]

        // [h - 1, 1] + u * wvec + v * (-uvec)
        // = [h - 1 - u, 1 + v + u] = [x, 1 + h - 1 - y + h - 1 - x]
        // = [x, 2h - 1 - y - x]

        [x, 2 * h - 1 - y - x]
    }
}

fn part3(input: String) -> QuestResult {
    let w = input.split('\n').next().unwrap().len();
    let h = w.div_ceil(2);

    dbg!(w);
    dbg!(h);

    let mut grid = Array2::from_elem([h, h], 255u8);

    for (y, l) in input.split('\n').enumerate() {
        for (x, c) in l.chars().filter(|&c| c != '.').enumerate() {
            grid[pack(h, [x % 2, y, x / 2])] = if c != '#' { 1 } else { 0 };
        }
    }

    let mut s = [h - 2, 2];

    grid[s] = 2;

    for i in 3..=4 {
        s = rotate(h, s);
        grid[s] = i;
    }

    println!("{grid}");

    todo!()
}
