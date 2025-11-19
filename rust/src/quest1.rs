use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let (names, moves) = input.split_once("\n\n").unwrap();

    let names: Vec<_> = names.split(',').collect();

    let mut i: usize = 0;

    for m in moves.split(',') {
        let (dir, n) = m.split_at(1);

        let n: usize = n.parse().unwrap();

        match dir {
            "R" => i = (i + n).min(names.len() - 1),
            "L" => i = i.saturating_sub(n),
            _ => panic!(),
        }
    }

    QuestResult::Text(names[i].to_owned())
}

fn part2(input: String) -> QuestResult {
    let (names, moves) = input.split_once("\n\n").unwrap();

    let names: Vec<_> = names.split(',').collect();

    let mut i: usize = 0;

    for m in moves.split(',') {
        let (dir, n) = m.split_at(1);

        let n: usize = n.parse().unwrap();

        i = match dir {
            "R" => i + n,
            "L" => i + names.len() - n,
            _ => panic!(),
        } % names.len();
    }

    QuestResult::Text(names[i].to_owned())
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
