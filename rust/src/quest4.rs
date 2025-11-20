use num_rational::Rational64;

use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut numbers = input.lines();

    let first: u64 = numbers.next().unwrap().parse().unwrap();
    let last: u64 = numbers.next_back().unwrap().parse().unwrap();

    QuestResult::Number(((2025 * first) / last) as i64)
}

fn part2(input: String) -> QuestResult {
    let mut numbers = input.lines();

    let first: u64 = numbers.next().unwrap().parse().unwrap();
    let last: u64 = numbers.next_back().unwrap().parse().unwrap();

    QuestResult::Number((10_000_000_000_000 * last).div_ceil(first) as i64)
}

fn part3(input: String) -> QuestResult {
    let mut numbers = input.lines();

    let first = numbers.next().unwrap().parse().unwrap();
    let last = numbers.next_back().unwrap().parse().unwrap();

    let ratio = numbers
        .map(|l| l.split_once('|').unwrap())
        .map(|(a, b)| Rational64::new(a.parse().unwrap(), b.parse().unwrap()))
        .fold(Rational64::new(1, first), |cur, new| cur * new)
        * Rational64::new(last, 1);

    QuestResult::Number((ratio.denom() * 100) / ratio.numer())
}
