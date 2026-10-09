use std::collections::{BTreeMap, HashSet, VecDeque};

use priority_queue::PriorityQueue;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{Quest, QuestResult, util::Ranges};

pub const PARTS: Quest = [part1, part3, part3];

fn part1(input: String) -> QuestResult {
    let mut walls = HashSet::new();

    let mut dir = [0, 0];
    let mut pos = [0, 0];

    for section in input.split(',') {
        let dirchar = section.chars().next().unwrap();
        let n: usize = section[1..].parse().unwrap();

        dir = match dir {
            [0, 0] => [1, 0],
            [x, 0] => [0, x],
            [0, x] => [-x, 0],
            _ => panic!(),
        };

        if dirchar == 'L' {
            dir = [-dir[0], -dir[1]];
        }

        for _ in 0..n {
            pos[0] += dir[0];
            pos[1] += dir[1];

            walls.insert(pos);
        }
    }

    let dest = pos;

    let mut queue = VecDeque::new();
    queue.push_back(([0, 0], 0));
    walls.insert([0, 0]);
    walls.remove(&dest);

    const DIRS: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    while let Some((pos, l)) = queue.pop_front() {
        if pos == dest {
            return QuestResult::Number(l);
        }

        for dir in DIRS {
            let newpos = [pos[0] + dir[0], pos[1] + dir[1]];

            if !walls.contains(&newpos) {
                walls.insert(newpos);
                queue.push_back((newpos, l + 1));
            }
        }
    }

    panic!()
}

// Equation for line a:
// ax = ax1 * (1 - t) + ax2 * t
// ay = ay1 * (1 - t) + ay2 * t
// 0 <= t <= 1

// Equation for line b:
// bx = bx1 * (1 - s) + bx2 * s
// by = by1 * (1 - s) + by2 * s
// 0 <= s <= 1

// System of equations:
// ax = bx
// ay = by
//
// ax1 * (1 - t) + ax2 * t = bx1 * (1 - s) + bx2 * s
// ay1 * (1 - t) + ay2 * t = by1 * (1 - s) + by2 * s
//
// ax1 - ax1 * t + ax2 * t = bx1 - bx1 * s + bx2 * s
// ay1 - ay1 * t + ay2 * t = by1 - by1 * s + by2 * s
//
// (ax2 - ax1) * t - (bx2 - bx1) * s = bx1 - ax1
// (ay2 - ay1) * t - (by2 - by1) * s = by1 - ay1
//
// [(ax2 - ax1) (bx1 - bx2)] [t] = [bx1 - ax1]
// [(ay2 - ay1) (by1 - by2)] [s] = [by1 - ay1]
//
// D = (ax2 - ax1) * (by1 - by2) - (bx1 - bx2) * (ay2 - ay1)
//
// t * D = (by1 - by2) * (bx1 - ax1) + (bx2 - bx1) * (by1 - ay1)
// s * D = (ax2 - ax1) * (by1 - ay1) - (ay2 - ay1) * (bx1 - ax1)

fn line_intersection_horizontal(
    [[ax1, ay1], [ax2, ay2]]: [[i64; 2]; 2],
    [bx1, bx2]: [i64; 2],
    by: i64,
) -> bool {
    // D = - (bx1 - bx2) * (ay2 - ay1)
    //
    // t * D =                           + (bx2 - bx1) * (by1 - ay1)
    // s * D = (ax2 - ax1) * (by1 - ay1) - (ay2 - ay1) * (bx1 - ax1)

    let d = -(bx1 - bx2) * (ay2 - ay1);

    if d == 0 {
        assert_eq!(ay1, ay2);
        let axr = ax1.min(ax2)..=ax1.max(ax2);
        ay1 == by && (axr.contains(&bx1) || axr.contains(&bx2))
    } else {
        let td = (bx2 - bx1) * (by - ay1);
        let sd = (ax2 - ax1) * (by - ay1) - (ay2 - ay1) * (bx1 - ax1);

        let (td, sd, d) = if d < 0 { (-td, -sd, -d) } else { (td, sd, d) };

        0 <= td && td <= d && 0 <= sd && sd <= d
    }
}

fn line_intersection_vertical(
    [[ax1, ay1], [ax2, ay2]]: [[i64; 2]; 2],
    bx: i64,
    [by1, by2]: [i64; 2],
) -> bool {
    // D = (ax2 - ax1) * (by1 - by2)
    //
    // t * D = (by1 - by2) * (bx1 - ax1)
    // s * D = (ax2 - ax1) * (by1 - ay1) - (ay2 - ay1) * (bx1 - ax1)

    let d = (ax2 - ax1) * (by1 - by2);

    if d == 0 {
        assert_eq!(ax1, ax2);
        let ayr = ay1.min(ay2)..=ay1.max(ay2);
        ax1 == bx && (ayr.contains(&by1) || ayr.contains(&by2))
    } else {
        let td = (by1 - by2) * (bx - ax1);
        let sd = (ax2 - ax1) * (by1 - ay1) - (ay2 - ay1) * (bx - ax1);

        let (td, sd, d) = if d < 0 { (-td, -sd, -d) } else { (td, sd, d) };

        0 <= td && td <= d && 0 <= sd && sd <= d
    }
}

fn part3(input: String) -> QuestResult {
    let mut horizontal = BTreeMap::new();
    let mut vertical = BTreeMap::new();
    let mut corners = Vec::new();

    let mut dir = [0, 0];
    let mut pos = [0, 0];

    for section in input.split(',') {
        let dirchar = section.chars().next().unwrap();
        let n: usize = section[1..].parse().unwrap();

        dir = match dir {
            [0, 0] => [1, 0],
            [x, 0] => [0, x],
            [0, x] => [-x, 0],
            _ => panic!(),
        };

        if dirchar == 'L' {
            dir = [-dir[0], -dir[1]];
        }

        let new_pos = [pos[0] + dir[0] * n as i64, pos[1] + dir[1] * n as i64];

        corners.push(new_pos);

        if new_pos[0] == pos[0] {
            // vertical
            let xval = pos[0];
            let ystart = pos[1].min(new_pos[1]);
            let ystop = pos[1].max(new_pos[1]);

            let r = if let Some(r) = vertical.get_mut(&xval) {
                r
            } else {
                vertical.insert(xval, Ranges::new());
                vertical.get_mut(&xval).unwrap()
            };

            r.push([ystart, ystop]);
        } else {
            // horizontal
            let yval = pos[1];
            let xstart = pos[0].min(new_pos[0]);
            let xstop = pos[0].max(new_pos[0]);

            let r = if let Some(r) = horizontal.get_mut(&yval) {
                r
            } else {
                horizontal.insert(yval, Ranges::new());
                horizontal.get_mut(&yval).unwrap()
            };

            r.push([xstart, xstop]);
        }

        pos = new_pos;
    }

    let dest = pos;

    if let Some(r) = horizontal.get_mut(&0) {
        r.remove_point(0);
    }
    if let Some(r) = horizontal.get_mut(&dest[1]) {
        r.remove_point(dest[0]);
    }
    if let Some(r) = vertical.get_mut(&dest[0]) {
        r.remove_point(dest[1]);
    }

    let h =
        |[x, y]: &[i64; 2]| (x.abs_diff(dest[0]) + y.abs_diff(dest[1])) as i64;

    let mut nodes = vec![dest];

    const DIRS: [[i64; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    for c in corners {
        for dir in DIRS {
            let node = [c[0] + dir[0], c[1] + dir[1]];

            let horizontal_collision = horizontal
                .get(&node[1])
                .is_some_and(|r| r.contains(node[0]));

            let vertical_collision =
                vertical.get(&node[0]).is_some_and(|r| r.contains(node[1]));

            if !(horizontal_collision || vertical_collision) {
                nodes.push(node);
            }
        }
    }

    nodes.sort_by_key(h);

    let horizontal: Vec<_> = horizontal.into_iter().collect();
    let vertical: Vec<_> = vertical.into_iter().collect();

    let mut queue = PriorityQueue::new();
    let mut seen = HashSet::new();
    queue.push(([0, 0], 0), 0);
    seen.insert([0, 0]);

    while let Some(((pos, l), _)) = queue.pop() {
        if pos == dest {
            return QuestResult::Number(l);
        }

        seen.insert(pos);

        let tmp: Vec<_> = nodes
            .par_iter()
            .filter(|&newpos| !seen.contains(newpos))
            .filter_map(|newpos| {
                let horizontal_collision = horizontal
                    .iter()
                    .flat_map(|(y, r)| r.ranges.iter().map(move |r| (y, r)))
                    .any(|(y, r)| {
                        line_intersection_horizontal([pos, *newpos], *r, *y)
                    });

                let collision = horizontal_collision
                    || vertical
                        .iter()
                        .flat_map(|(x, r)| r.ranges.iter().map(move |r| (x, r)))
                        .any(|(x, r)| {
                            line_intersection_vertical([pos, *newpos], *x, *r)
                        });

                (!(collision || seen.contains(newpos))).then(|| {
                    let new_l = l
                        + (pos[0] - newpos[0]).abs()
                        + (pos[1] - newpos[1]).abs();

                    ((*newpos, new_l), -new_l - h(newpos))
                })
            })
            .collect();

        for (x, p) in tmp {
            queue.push_increase(x, p);
        }
    }

    panic!()
}
