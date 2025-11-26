use std::collections::VecDeque;

use itertools::Itertools;

use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut dial = VecDeque::new();

    dial.push_back(1u32);

    for (x, front) in input
        .lines()
        .map(|l| l.parse().unwrap())
        .zip([false, true].into_iter().cycle())
    {
        if front {
            dial.push_front(x);
        } else {
            dial.push_back(x);
        }
    }

    let top = dial.len().div_ceil(2) - 1;

    let i = (top + 2025) % dial.len();

    QuestResult::Number(dial[i] as i64)
}

fn build_dial(input: &str) -> (usize, VecDeque<u32>) {
    let mut dial = VecDeque::new();

    dial.push_back(1);

    let mut top = 0;

    for (r, front) in input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            a.parse().unwrap()..=b.parse().unwrap()
        })
        .zip([false, true].into_iter().cycle())
    {
        if front {
            top += r.try_len().unwrap();
            dial.extend_front(r);
        } else {
            dial.extend(r);
        }
    }

    (top, dial)
}

fn part2(input: String) -> QuestResult {
    let (top, dial) = build_dial(&input);

    let i = (top + 20252025) % dial.len();

    QuestResult::Number(dial[i] as i64)
}

fn part3(input: String) -> QuestResult {
    let (top, dial) = build_dial(&input);

    let i = ((top as u64 + 202520252025) % dial.len() as u64) as usize;

    QuestResult::Number(dial[i] as i64)
}
