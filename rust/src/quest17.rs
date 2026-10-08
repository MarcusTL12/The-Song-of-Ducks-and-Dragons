use std::{
    collections::{HashMap, HashSet, VecDeque},
    f64::consts::PI,
    io::stdin,
};

use ndarray::{Array2, ArrayView2};
use priority_queue::PriorityQueue;

use crate::{
    Quest,
    QuestResult::{self, Text},
    util::input_to_grid,
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

fn get_minimum_radius(state: &[[usize; 2]], [vy, vx]: [usize; 2]) -> u64 {
    state
        .iter()
        .map(|&[y, x]| {
            let dx = x as i64 - vx as i64;
            let dy = y as i64 - vy as i64;
            (dx * dx + dy * dy) as u64
        })
        .min()
        .unwrap()
}

// Returns None if inconclusive (Could become valid)
// Some(true) if valid loop
// Some(false) if somehow invalid
fn analyze_state(
    state: &[[usize; 2]],
    s: [usize; 2],
    [vy, vx]: [usize; 2],
    grid: ArrayView2<u8>,
) -> Option<bool> {
    let time: i64 = state
        .iter()
        .map(|&i| {
            let n = grid[i];
            match n {
                b'S' | b'@' => 0,
                b'0'..=b'9' => (n - b'0') as i64,
                _ => panic!("{}", n as char),
            }
        })
        .sum();

    let r = time / 30;

    for &[y, x] in state {
        let dx = x as i64 - vx as i64;
        let dy = y as i64 - vy as i64;

        if dx * dx + dy * dy <= r * r {
            return Some(false);
        }
    }

    if state.len() > 1 && state.iter().last().cloned().unwrap() == s {
        let shape = grid.shape();
        let mut seen = Array2::from_elem([shape[0], shape[1]], false);

        let mut queue = VecDeque::new();
        queue.push_back([vy, vx]);
        seen[[vy, vx]] = true;

        const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

        while let Some([y, x]) = queue.pop_front() {
            for dir in DIRS {
                let newpos = [
                    (y as isize + dir[0]) as usize,
                    (x as isize + dir[1]) as usize,
                ];

                match seen.get_mut(newpos) {
                    None => return Some(false),
                    Some(x) => {
                        if !*x {
                            *x = true;
                        }
                        if !state.contains(&newpos) {
                            queue.push_back(newpos);
                        }
                    }
                }
            }
        }
    }

    None
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

fn compute_winding_angle(state: &[[usize; 2]], [vy, vx]: [usize; 2]) -> f64 {
    let vx = vx as f64;
    let vy = vy as f64;

    state
        .array_windows()
        .map(|&[[y1, x1], [y2, x2]]| {
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
        })
        .sum()
}

fn compute_winding_number(state: &[[usize; 2]], v: [usize; 2]) -> i64 {
    let t = compute_winding_angle(state, v);

    (t / (2.0 * PI) + 0.0001).floor() as i64
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

    let mut queue = PriorityQueue::new();
    let mut seen = HashSet::new();

    let mut exact_angles = HashMap::new();

    let state = (
        s,
        s[0].abs_diff(v[0]).pow(2) + s[1].abs_diff(v[1]).pow(2),
        0i64,
    );
    queue.push(state, 0);
    exact_angles.insert(state, 0.0f64);

    const DIRS: [[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    while let Some(((pos, minr, ang), l)) = queue.pop() {
        // println!("{:?}, {l}", (pos, minr, ang));

        // println!("{}, {}", k.1, k.2);

        seen.insert((pos, minr, ang));
        let exang = exact_angles[&(pos, minr, ang)];

        // let result = analyze_state(&state, s, v, grid);

        // println!("{result:?}");

        // let mut tmp = String::new();
        // stdin().read_line(&mut tmp).unwrap();

        if pos == s && exang.abs() > PI {
            return Text(format!("{:?}", (pos, minr, exang, l)));
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

            queue.push_increase(newk, newl);
            exact_angles.insert(newk, new_exang);
        }

        // match result {
        //     Some(true) => todo!("{state:?}"),
        //     Some(false) => {} // Do not explore further
        //     None => {
        //         // Explore further

        //         let pos = state.last().unwrap();

        //         for dir in DIRS {
        //             let newpos = [
        //                 (pos[0] as isize + dir[0]) as usize,
        //                 (pos[1] as isize + dir[1]) as usize,
        //             ];

        //             // Simple optimization, backtracking the last step will
        //             // never be good
        //             if grid.get(newpos).is_none()
        //                 || state.iter().rev().nth(1).cloned() == Some(newpos)
        //             {
        //                 continue;
        //             }

        //             let mut newstate = state.clone();
        //             newstate.push(newpos);

        //             let n = grid[newpos];
        //             let newl = match n {
        //                 b'0'..=b'9' => l - (grid[newpos] - b'0') as i64,
        //                 b'S' | b'@' => l,
        //                 _ => panic!(),
        //             };

        //             if seen.contains(&(
        //                 newpos,
        //                 get_minimum_radius(&newstate, v),
        //                 compute_winding_number(&state, v),
        //             )) {
        //                 continue;
        //             }

        //             queue.push_increase(newstate, newl);
        //         }
        //     }
        // }
    }

    todo!()
}
