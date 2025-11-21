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
    let ans = input.as_bytes().iter().fold(
        [[0; 3]; 2],
        |[mut knights, mut pairs], x| {
            let i = ((x & !0x20) - b'A') as usize;

            if (x & 0x20) == 0 {
                knights[i] += 1;
            } else {
                pairs[i] += knights[i];
            }

            [knights, pairs]
        },
    )[1]
    .iter()
    .sum();

    QuestResult::Number(ans)
}

fn part3(input: String) -> QuestResult {
    const REPEATS: usize = 1000;
    const MAXDIST: usize = 1000;

    let mut knights = [0; 3];
    let mut novices = [0; 3];

    let mut pairs = 0;

    let input = input.as_bytes();

    let totlen = input.len() * REPEATS;

    for head in 0..totlen {
        if head > MAXDIST {
            let tail = head - MAXDIST - 1;
            let x_tail = input[tail % input.len()];
            let i = ((x_tail & !0x20) - b'A') as usize;

            if (x_tail & 0x20) == 0 {
                knights[i] -= 1;
            } else {
                novices[i] -= 1;
            }
        }

        let x_head = input[head % input.len()];
        let i = ((x_head & !0x20) - b'A') as usize;

        if (x_head & 0x20) == 0 {
            pairs += novices[i];
            knights[i] += 1;
        } else {
            pairs += knights[i];
            novices[i] += 1;
        }
    }

    QuestResult::Number(pairs)
}
