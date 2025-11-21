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

    let mut init_pairs = 0;

    let input = input.as_bytes();

    let n_passes_to_fill_window = MAXDIST.div_ceil(input.len());
    assert!(n_passes_to_fill_window <= REPEATS);

    let mut head = 0;

    for _ in 0..n_passes_to_fill_window {
        for x_head in input {
            if head > MAXDIST {
                let tail = head - MAXDIST - 1;
                let x_tail = input[tail];
                let i = ((x_tail & !0x20) - b'A') as usize;

                if (x_tail & 0x20) == 0 {
                    knights[i] -= 1;
                } else {
                    novices[i] -= 1;
                }
            }

            let i = ((x_head & !0x20) - b'A') as usize;

            if (x_head & 0x20) == 0 {
                init_pairs += novices[i];
                knights[i] += 1;
            } else {
                init_pairs += knights[i];
                novices[i] += 1;
            }

            head += 1;
        }
    }

    let mut tail = head - MAXDIST - 1;
    assert!(tail < input.len());

    let mut bulk_pairs = 0;

    for x_head in input {
        let x_tail = input[tail];
        let i = ((x_tail & !0x20) - b'A') as usize;

        if (x_tail & 0x20) == 0 {
            knights[i] -= 1;
        } else {
            novices[i] -= 1;
        }

        let i = ((x_head & !0x20) - b'A') as usize;

        if (x_head & 0x20) == 0 {
            bulk_pairs += novices[i];
            knights[i] += 1;
        } else {
            bulk_pairs += knights[i];
            novices[i] += 1;
        }

        tail += 1;
        if tail == input.len() {
            tail = 0;
        }
    }

    let pairs = init_pairs + bulk_pairs * (REPEATS - n_passes_to_fill_window);

    QuestResult::Number(pairs as i64)
}
