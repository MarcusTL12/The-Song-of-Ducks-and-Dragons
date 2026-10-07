use crate::{Quest, QuestResult, util::input_to_grid};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(mut input: String) -> QuestResult {
    input.push('\n');
    let grid = input_to_grid(input.as_bytes());

    let [vy, vx] = grid
        .indexed_iter()
        .find_map(|((i, j), &x)| (x == b'@').then_some([i, j]))
        .unwrap();

    const R: i64 = 10;

    let ans: u64 = grid
        .indexed_iter()
        .filter_map(|((y, x), &n)| {
            let dx = x as i64 - vx as i64;
            let dy = y as i64 - vy as i64;

            (n != b'@' && dx * dx + dy * dy <= R * R)
                .then_some((n - b'0') as u64)
        })
        .sum();

    QuestResult::Number(ans as i64)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
