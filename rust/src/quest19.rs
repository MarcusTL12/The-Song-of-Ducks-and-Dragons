use std::collections::BTreeMap;

use priority_queue::PriorityQueue;

use crate::{
    Quest,
    QuestResult::{self, Number},
    util::Ranges,
};

pub const PARTS: Quest = [part1, part2, part2];

fn part1(input: String) -> QuestResult {
    let walls: BTreeMap<_, _> = input
        .split('\n')
        .map(|l| {
            let mut it = l.split(',').map(|c| c.parse::<i32>().unwrap());

            let x = it.next().unwrap();
            let ystart = it.next().unwrap();
            let len = it.next().unwrap();

            (x, ystart..(ystart + len))
        })
        .collect();

    let &max_x = walls.keys().last().unwrap();

    let mut queue = PriorityQueue::new();
    queue.push([0, 0], 0);

    while let Some(([x, y], flaps)) = queue.pop() {
        if x >= max_x {
            return Number(-flaps);
        }

        for flap in [false, true] {
            let newx = x + 1;
            let newy = if flap { y + 1 } else { y - 1 };
            let newflaps = if flap { flaps - 1 } else { flaps };

            if let Some(hole) = walls.get(&newx)
                && !hole.contains(&newy)
            {
                continue;
            }

            queue.push_increase([newx, newy], newflaps);
        }
    }

    panic!()
}

fn propagate_flaps_to_column(
    from: &BTreeMap<i64, i64>,
    x: i64,
    y: i64,
) -> Option<i64> {
    from.iter()
        .filter_map(|(&y_init, &flaps_init)| {
            ((y_init - y).abs() <= x
                && (y.abs_diff(y_init) & 1) == (x as u64 & 1))
                .then_some(flaps_init + (x + y - y_init) / 2)
        })
        .min()
}

fn part2(input: String) -> QuestResult {
    let mut walls = BTreeMap::new();

    for l in input.split('\n') {
        let mut it = l.split(',').map(|c| c.parse().unwrap());

        let x = it.next().unwrap();
        let ystart = it.next().unwrap();
        let len = it.next().unwrap();

        let r = if let Some(r) = walls.get_mut(&x) {
            r
        } else {
            walls.insert(x, Ranges::new());
            walls.get_mut(&x).unwrap()
        };

        r.push([ystart, ystart + len - 1]);
    }

    let walls: Vec<_> = walls.into_iter().collect();

    let mut state = BTreeMap::new();
    state.insert(0, 0);
    let mut x = 0;

    for (newx, r) in walls {
        state = r
            .iter()
            .filter_map(|y| {
                propagate_flaps_to_column(&state, newx - x, y)
                    .map(|flaps| (y, flaps))
            })
            .collect();

        x = newx;
    }

    Number(state.into_values().min().unwrap())
}
