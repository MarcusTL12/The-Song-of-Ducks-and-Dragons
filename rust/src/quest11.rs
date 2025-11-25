use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut columns: Vec<u32> =
        input.lines().map(|l| l.parse().unwrap()).collect();

    let mut dir = false;

    let mut r = 0;

    loop {
        let mut did_round = false;

        for i in 1..columns.len() {
            let [a, b] = columns.get_disjoint_mut([i - 1, i]).unwrap();

            if !dir && *a > *b {
                *a -= 1;
                *b += 1;
                did_round = true;
            } else if dir && *a < *b {
                *a += 1;
                *b -= 1;
                did_round = true;
            }
        }

        if did_round {
            r += 1;
        } else if !dir {
            dir = true;
        }

        if r == 10 {
            break;
        }
    }

    let ans = columns
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i as u32 + 1) * x)
        .sum::<u32>();

    QuestResult::Number(ans as i64)
}

fn part2(input: String) -> QuestResult {
    let mut columns: Vec<u32> =
        input.lines().map(|l| l.parse().unwrap()).collect();

    let mut dir = false;

    let mut r = 0;

    loop {
        let mut did_round = false;

        for i in 1..columns.len() {
            let [a, b] = columns.get_disjoint_mut([i - 1, i]).unwrap();

            if !dir && *a > *b {
                *a -= 1;
                *b += 1;
                did_round = true;
            } else if dir && *a < *b {
                *a += 1;
                *b -= 1;
                did_round = true;
            }
        }

        if did_round {
            r += 1;
        } else if !dir {
            dir = true;
        } else {
            break;
        }
    }

    QuestResult::Number(r as i64)
}

fn part3(input: String) -> QuestResult {
    let columns: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mean = columns.iter().sum::<u64>() / columns.len() as u64;

    let ans = columns
        .into_iter()
        .take_while(|&x| x < mean)
        .map(|x| mean - x)
        .sum::<u64>();

    QuestResult::Number(ans as i64)
}
