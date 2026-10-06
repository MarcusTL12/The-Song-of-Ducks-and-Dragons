use std::collections::{HashSet, VecDeque};

use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part1, part3];

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

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
