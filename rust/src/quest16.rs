use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let ans: u64 = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .map(|n: u64| 90 / n)
        .sum();

    QuestResult::Number(ans as i64)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
