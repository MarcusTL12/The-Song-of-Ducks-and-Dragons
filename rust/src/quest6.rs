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
    let mut knights = [0; 3];
    let mut pairs = [0; 3];

    for &x in input.as_bytes() {
        let i = ((x & !0x20) - b'A') as usize;

        if (x & 0x20) == 0 {
            knights[i] += 1;
        } else {
            pairs[i] += knights[i];
        }
    }

    QuestResult::Number(pairs.into_iter().sum::<i64>())
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
