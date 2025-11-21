use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let [_, ans] =
        input
            .chars()
            .fold([0, 0], |[n_knights, n_pairs], x| match x {
                'A' => [n_knights + 1, n_pairs],
                'a' => [n_knights, n_pairs + n_knights],
                _ => [n_knights, n_pairs],
            });

    QuestResult::Number(ans)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
