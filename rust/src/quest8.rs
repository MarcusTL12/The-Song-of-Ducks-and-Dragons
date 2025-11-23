use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    const MODULUS: u8 = 32;

    let ans = input
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .map_windows(|[a, b]| (a % (MODULUS / 2)) == (b % (MODULUS / 2)))
        .filter(|&x| x)
        .count();

    QuestResult::Number(ans as i64)
}

fn part2(input: String) -> QuestResult {
    let lines: Vec<[u32; 2]> = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .map_windows(|&[a, b]| if a < b { [a, b] } else { [b, a] })
        .collect();

    let ans = lines
        .iter()
        .enumerate()
        .flat_map(|(i, [a1, b1])| {
            lines.iter().take(i).map(move |[a2, b2]| {
                a1 < a2 && a2 < b1 && b1 < b2 || a2 < a1 && a1 < b2 && b2 < b1
            })
        })
        .filter(|&x| x)
        .count();

    QuestResult::Number(ans as i64)
}

fn part3(input: String) -> QuestResult {
    const NAILS: u32 = 256;

    let lines: Vec<[u32; 2]> = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .map_windows(|&[a, b]| if a < b { [a, b] } else { [b, a] })
        .collect();

    let ans = (1..NAILS)
        .flat_map(|a1| (a1 + 1..=NAILS).map(move |b1| [a1, b1]))
        .map(|[a1, b1]| {
            lines
                .iter()
                .filter(move |&&[a2, b2]| {
                    a1 < a2 && a2 < b1 && b1 < b2
                        || a2 < a1 && a1 < b2 && b2 < b1
                        || a1 == a2 && b1 == b2
                })
                .count()
        })
        .max()
        .unwrap();

    QuestResult::Number(ans as i64)
}
