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
    let mut dial = VecDeque::new();

    dial.push_back([1u32, 1]);

    let mut top = 0;
    let mut totlen = 1;

    for ([a, b], front) in input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            [a.parse().unwrap(), b.parse().unwrap()]
        })
        .zip([false, true].into_iter().cycle())
    {
        let rlen = (b - a + 1) as u64;
        totlen += rlen;

        if front {
            top += rlen;
            dial.push_front([b, a]);
        } else {
            dial.push_back([a, b]);
        }
    }

    let mut i = (top + 202520252025) % totlen;

    for [a, b] in dial {
        let rlen = (a.abs_diff(b) + 1) as u64;

        if i >= rlen {
            i -= rlen;
        } else if a < b {
            return QuestResult::Number((a as u64 + i) as i64);
        } else {
            return QuestResult::Number((a as u64 - i) as i64);
        }
    }

    todo!()
}
