use std::collections::{BTreeMap, HashSet, VecDeque};

use priority_queue::PriorityQueue;

use crate::{Quest, QuestResult};

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

#[derive(Debug)]
struct Ranges {
    ranges: Vec<[i64; 2]>,
}

enum RangeSearchResult {
    In(usize),       // Completely contained in range at given index
    Void(usize),     // Completely disconnected from any ranges
    Left(usize),     // Not contained in any range but x - 1 is in range i - 1
    Right(usize),    // Not contained in any range but x + 1 is in range i
    Sandwich(usize), // Both Left and Right at the same time
}

impl Ranges {
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn find_point(&self, x: i64) -> Result<usize, usize> {
        use std::cmp::Ordering::*;
        self.ranges.binary_search_by(|&[a, b]| {
            if x < a {
                Greater
            } else if b < x {
                Less
            } else {
                Equal
            }
        })
    }

    fn analyze_point(&self, x: i64) -> RangeSearchResult {
        use RangeSearchResult::*;
        match self.find_point(x) {
            Ok(i) => In(i),
            Err(i) => {
                let left = i
                    .checked_sub(1)
                    .and_then(|j| self.ranges.get(j))
                    .is_some_and(|&[_, y]| x == y);

                let right =
                    self.ranges.get(i + 1).is_some_and(|&[y, _]| x == y);

                match [left, right] {
                    [false, false] => Void(i),
                    [true, false] => Left(i),
                    [false, true] => Right(i),
                    [true, true] => Sandwich(i),
                }
            }
        }
    }

    pub fn contains(&self, x: i64) -> bool {
        self.find_point(x).is_ok()
    }

    pub fn push(&mut self, [from, to]: [i64; 2]) {
        use RangeSearchResult::*;
        match [self.analyze_point(from), self.analyze_point(to)] {
            [In(i), In(j)] if i == j => {}
            [Void(i), Void(j)] if i == j => self.ranges.insert(i, [from, to]),
            [a, b] => {
                let (i, start) = match a {
                    In(i) => (i, self.ranges[i][0]),
                    Void(i) | Right(i) => (i, from),
                    Left(i) | Sandwich(i) => (i - 1, self.ranges[i - 1][0]),
                };

                let (j, stop) = match b {
                    In(j) | Right(j) | Sandwich(j) => (j, self.ranges[j][1]),
                    Void(j) | Left(j) => (j - 1, to),
                };

                self.ranges[i] = [start, stop];
                self.ranges.drain((i + 1)..=j);
            }
        }
    }
}

fn part3(input: String) -> QuestResult {
    let mut horizontal = BTreeMap::new();
    let mut vertical = BTreeMap::new();

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

    let h =
        |[x, y]: [i64; 2]| (x.abs_diff(dest[0]) + y.abs_diff(dest[1])) as i64;

    let mut queue = PriorityQueue::new();
    let mut seen = HashSet::new();
    queue.push(([0, 0], 0), -h([0, 0]));
    seen.insert([0, 0]);

    const DIRS: [[i64; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    while let Some(((pos, l), _)) = queue.pop() {
        if pos == dest {
            return QuestResult::Number(l);
        }

        seen.insert(pos);

        for dir in DIRS {
            let newpos = [pos[0] + dir[0], pos[1] + dir[1]];

            let horizontal_collision = horizontal
                .get(&newpos[1])
                .is_some_and(|r| r.contains(newpos[0]));

            let vertical_collision = vertical
                .get(&newpos[0])
                .is_some_and(|r| r.contains(newpos[1]));

            if newpos == dest
                || !(horizontal_collision
                    || vertical_collision
                    || seen.contains(&newpos))
            {
                queue.push((newpos, l + 1), -l - h(newpos));
            }
        }
    }

    panic!()
}
