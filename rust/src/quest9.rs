use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut lines = input.lines();

    let parent1 = lines.next().unwrap().split_once(':').unwrap().1.as_bytes();
    let parent2 = lines.next().unwrap().split_once(':').unwrap().1.as_bytes();
    let child = lines.next().unwrap().split_once(':').unwrap().1.as_bytes();

    let sim1 = parent1
        .iter()
        .zip(child.iter())
        .filter(|(a, b)| a == b)
        .count();

    let sim2 = parent2
        .iter()
        .zip(child.iter())
        .filter(|(a, b)| a == b)
        .count();

    QuestResult::Number((sim1 * sim2) as i64)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
