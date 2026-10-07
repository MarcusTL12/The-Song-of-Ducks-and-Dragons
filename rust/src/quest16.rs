use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let ans: u32 = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .map(|n: u32| 90 / n)
        .sum();

    QuestResult::Number(ans as i64)
}

fn part2(input: String) -> QuestResult {
    let mut wall: Vec<u32> =
        input.split(',').map(|x| x.parse().unwrap()).collect();

    let mut spellprod = 1;

    for n in 1.. {
        let &m = wall.iter().skip(n - 1).step_by(n).min().unwrap();

        for _ in 0..m {
            spellprod *= n;
        }

        for x in wall.iter_mut().skip(n - 1).step_by(n) {
            *x -= m;
        }

        if wall.iter().all(|&x| x == 0) {
            break;
        }
    }

    QuestResult::Number(spellprod as i64)
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
