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

fn similarity_score(p1: &[u8], p2: &[u8], c: &[u8]) -> Option<u64> {
    let mut n1 = 0;
    let mut n2 = 0;

    for ((&a, &b), &c) in p1.iter().zip(p2.iter()).zip(c.iter()) {
        if c == a {
            n1 += 1;
        }

        if c == b {
            n2 += 1;
        }

        if c != a && c != b {
            return None;
        }
    }

    Some(n1 * n2)
}

fn part2(input: String) -> QuestResult {
    let scales: Vec<_> = input
        .lines()
        .map(|l| l.split_once(':').unwrap().1.as_bytes())
        .collect();

    let scales = scales.as_slice();

    let ans: u64 = scales
        .iter()
        .enumerate()
        .flat_map(|(i, &c)| {
            scales
                .iter()
                .enumerate()
                .filter(move |&(j, _)| j != i)
                .flat_map(move |(j, a)| {
                    scales
                        .iter()
                        .enumerate()
                        .filter(move |&(k, _)| k != i)
                        .take(j)
                        .map(move |(_, b)| [a, b, c])
                })
        })
        .filter_map(|[a, b, c]| similarity_score(a, b, c))
        .sum();

    QuestResult::Number(ans as i64)
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
