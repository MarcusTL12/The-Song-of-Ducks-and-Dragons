use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut n_knights = 0;
    let mut n_pairs = 0;

    for x in input.chars() {
        match x {
            'A' => n_knights += 1,
            'a' => n_pairs += n_knights,
            _ => {}
        }
    }

    QuestResult::Number(n_pairs)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
