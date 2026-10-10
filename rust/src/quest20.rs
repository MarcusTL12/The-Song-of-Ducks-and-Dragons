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

        [h - 1 - x - y, y]
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

        [2 * h - 1 - y - x, y]
    }
}

fn which_triangle(h: usize, [y, x]: [usize; 2]) -> Option<bool> {
    ((0..h).contains(&y) && (0..h).contains(&x)).then(|| x + y < h)
}

fn next_positions(
    h: usize,
    [y, x]: [usize; 2],
) -> impl Iterator<Item = [usize; 2]> {
    let [v, u] = [h - 1 - y, h - 1 - x];

    let cur_tri = which_triangle(h, [y, x]).unwrap();

    let candidates = [[v, u], [v + 1, u], [v, u + 1]];

    [[y, x]]
        .into_iter()
        .chain(
            candidates
                .into_iter()
                .filter(move |&p| which_triangle(h, p) == Some(!cur_tri)),
        )
        .map(move |p| rotate(h, p))
}

fn part3(input: String) -> QuestResult {
    let w = input.split('\n').next().unwrap().len();
    let h = w.div_ceil(2);

    let mut grid = Array2::from_elem([h, h], 255u8);

    for (y, l) in input.split('\n').enumerate() {
        for (x, c) in l.chars().filter(|&c| c != '.').enumerate() {
            grid[pack(h, [x % 2, y, x / 2])] = match c {
                '#' | 'S' => 0,
                'T' => 1,
                'E' => 2,
                _ => panic!(),
            };
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(([h - 1, 0], 0));

    while let Some((pos, l)) = queue.pop_front() {
        for newpos in next_positions(h, pos) {
            match grid[newpos] {
                0 => {}
                1 => {
                    grid[newpos] = 0;
                    queue.push_back((newpos, l + 1));
                }
                2 => return Number(l + 1),
                _ => panic!(),
            }
        }
    }

    panic!()
}
