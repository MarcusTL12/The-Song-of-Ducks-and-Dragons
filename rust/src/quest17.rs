use std::{
    collections::{HashMap, HashSet},
    f64::consts::PI,
};

use ndarray::{Array2, ArrayView2};
use priority_queue::PriorityQueue;

use crate::{Quest, QuestResult, util::input_to_grid};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(mut input: String) -> QuestResult {
    input.push('\n');
    let grid = input_to_grid(input.as_bytes());

    let [vy, vx] = grid
        .indexed_iter()
        .find_map(|((i, j), &x)| (x == b'@').then_some([i, j]))
        .unwrap();

    const R: i64 = 10;

    let ans: u64 = grid
        .indexed_iter()
        .filter_map(|((y, x), &n)| {
            let dx = x as i64 - vx as i64;
            let dy = y as i64 - vy as i64;

            (n != b'@' && dx * dx + dy * dy <= R * R)
                .then_some((n - b'0') as u64)
        })
        .sum();

    QuestResult::Number(ans as i64)
}

fn part2(mut input: String) -> QuestResult {
    input.push('\n');
    let grid = input_to_grid(input.as_bytes());

    let [vy, vx] = grid
        .indexed_iter()
        .find_map(|((i, j), &x)| (x == b'@').then_some([i, j]))
        .unwrap();

    let mut sums = Vec::new();

    for ((y, x), &n) in grid.indexed_iter() {
        if n == b'@' {
            continue;
        }

        let dx = x as i64 - vx as i64;
        let dy = y as i64 - vy as i64;

        let r2 = dx * dx + dy * dy;

        let r = (r2 as f64).sqrt().ceil() as usize;

        while sums.len() <= r {
            sums.push(0);
        }

        sums[r] += (n - b'0') as u64;
    }

    let ans = sums
        .into_iter()
        .enumerate()
        .max_by_key(|&(_, x)| x)
        .map(|(i, x)| i as u64 * x)
        .unwrap();

    QuestResult::Number(ans as i64)
}

fn compute_winding_angle_step(
    [[y1, x1], [y2, x2]]: [[usize; 2]; 2],
    [vy, vx]: [usize; 2],
) -> f64 {
    let vx = vx as f64;
    let vy = vy as f64;

    let x1 = x1 as f64;
    let x2 = x2 as f64;
    let y1 = y1 as f64;
    let y2 = y2 as f64;

    let ax = x1 - vx;
    let ay = y1 - vy;
    let bx = x2 - vx;
    let by = y2 - vy;

    let axb = ax * by - bx * ay;
    let al2 = ax * ax + ay * ay;
    let bl2 = bx * bx + by * by;

    (axb / (al2 * bl2).sqrt()).asin()
}

const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn compute_shortest_dist_to_all(
    grid: ArrayView2<u8>,
    start: [usize; 2],
) -> Array2<i32> {
    let mut queue = PriorityQueue::new();
    let mut dists = Array2::from_elem(grid.dim(), -1);

    queue.push(start, 0);

    while let Some((pos, l)) = queue.pop() {
        let n = grid[pos];
        dists[pos] = -l
            - match n {
                b'0'..=b'9' => (n - b'0') as i32,
                b'S' | b'@' => 0,
                _ => panic!(),
            };

        for dir in DIRS {
            let newpos = [
                (pos[0] as isize + dir[0]) as usize,
                (pos[1] as isize + dir[1]) as usize,
            ];

            if grid.get(newpos).is_none() {
                continue;
            }

            if dists[newpos] != -1 {
                continue;
            }

            let n = grid[newpos];
            let newl = match n {
                b'0'..=b'9' => l - (n - b'0') as i32,
                b'S' | b'@' => l,
                _ => panic!(),
            };

            queue.push_increase(newpos, newl);
        }
    }

    dists
}

fn part3(mut input: String) -> QuestResult {
    input.push('\n');
    let grid = input_to_grid(input.as_bytes());

    let [v, s] = {
        let mut v = None;
        let mut s = None;

        for ((y, x), &n) in grid.indexed_iter() {
            match n {
                b'@' => v = Some([y, x]),
                b'S' => s = Some([y, x]),
                _ => {}
            }

            if v.is_some() && s.is_some() {
                break;
            }
        }

        [v.unwrap(), s.unwrap()]
    };

    let disthome = compute_shortest_dist_to_all(grid, s);

    // Man-dist back home as minimum remaining time
    let h = |(pos, _, _): ([usize; _], _, _)| {
        disthome[pos] as i64
        // (pos[0].abs_diff(s[0]) + pos[1].abs_diff(s[1])) as i64
        // 0
    };

    let mut queue = PriorityQueue::new();
    let mut seen = HashSet::new();
    let mut exact_angles = HashMap::new();

    let start_state = (
        s,
        s[0].abs_diff(v[0]).pow(2) + s[1].abs_diff(v[1]).pow(2),
        0i64,
    );
    queue.push(start_state, 0);
    exact_angles.insert(start_state, 0.0f64);

    let mut i = 0;

    let l = loop {
        i += 1;
        let Some(((pos, minr, ang), d)) = queue.pop() else {
            panic!("No path found!!")
        };

        let l = d + h((pos, minr, ang));

        seen.insert((pos, minr, ang));
        let exang = exact_angles[&(pos, minr, ang)];

        if pos == s && exang.abs() > PI {
            break -l;
        }

        for dir in DIRS {
            let newpos = [
                (pos[0] as isize + dir[0]) as usize,
                (pos[1] as isize + dir[1]) as usize,
            ];

            if grid.get(newpos).is_none() {
                continue;
            }

            let new_minr = minr.min(
                newpos[0].abs_diff(v[0]).pow(2)
                    + newpos[1].abs_diff(v[1]).pow(2),
            );

            let new_exang =
                exang + compute_winding_angle_step([pos, newpos], v);
            let new_ang = (new_exang * 1e6).floor() as i64;

            let n = grid[newpos];
            let newl = match n {
                b'0'..=b'9' => l - (n - b'0') as i64,
                b'S' | b'@' => l,
                _ => panic!(),
            };

            let vr = newl / 30;

            if new_minr <= (vr * vr) as usize {
                continue;
            }

            let newk = (newpos, new_minr, new_ang);

            if seen.contains(&newk) {
                continue;
            }

            queue.push_increase(newk, newl - h(newk));
            exact_angles.insert(newk, new_exang);
        }
    };

    dbg!(i);

    QuestResult::Number(l * (l / 30))
}
