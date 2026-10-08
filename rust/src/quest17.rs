use std::{
    collections::{HashMap, HashSet, VecDeque},
    f64::consts::PI,
};

use priority_queue::PriorityQueue;

use crate::{
    Quest, QuestResult,
    util::{input_to_grid, input_to_grid_mut},
};

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

fn part3(mut input: String) -> QuestResult {
    let inpbytes = unsafe { input.as_mut_vec() };
    inpbytes.push(b'\n');
    let mut grid = input_to_grid_mut(inpbytes);

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

    let mut queue = PriorityQueue::new();
    let mut seen = HashSet::new();
    let mut exact_angles = HashMap::new();
    let mut backtrack = HashMap::new();

    let start_state = (
        s,
        s[0].abs_diff(v[0]).pow(2) + s[1].abs_diff(v[1]).pow(2),
        0i64,
    );
    queue.push(start_state, 0);
    exact_angles.insert(start_state, 0.0f64);

    const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    let (mut state, l) = loop {
        let Some(((pos, minr, ang), l)) = queue.pop() else {
            panic!("No path found!!")
        };

        seen.insert((pos, minr, ang));
        let exang = exact_angles[&(pos, minr, ang)];

        if pos == s && exang.abs() > PI {
            break ((pos, minr, ang), l);
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
                b'0'..=b'9' => l - (grid[newpos] - b'0') as i64,
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

            if queue.get_priority(&newk).is_none_or(|&oldl| oldl < newl) {
                backtrack.insert(newk, (pos, minr, ang));
            }

            queue.push_increase(newk, newl);
            exact_angles.insert(newk, new_exang);
        }
    };

    dbg!(l);

    grid[state.0] = b'#';

    while state != start_state {
        state = backtrack[&state];

        grid[state.0] = b'#';
    }

    let mut queue = VecDeque::new();
    queue.push_back(v);

    let mut maxr2 = 0;

    while let Some(pos) = queue.pop_front() {
        let r2 = pos[0].abs_diff(v[0]).pow(2) + pos[1].abs_diff(v[1]).pow(2);

        maxr2 = maxr2.max(r2);

        for dir in DIRS {
            let newpos = [
                (pos[0] as isize + dir[0]) as usize,
                (pos[1] as isize + dir[1]) as usize,
            ];

            match grid[newpos] {
                b'.' | b'#' => {}
                _ => {
                    queue.push_back(newpos);
                    grid[newpos] = b'.';
                }
            }
        }
    }

    dbg!(maxr2);

    println!("{input}");

    todo!()
}
