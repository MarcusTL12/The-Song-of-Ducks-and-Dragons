use crate::{
    Quest,
    QuestResult::{self, Number},
    util::input_to_grid,
};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(mut input: String) -> QuestResult {
    input.push('\n');
    let grid = input_to_grid(input.as_bytes());

    let ans: u64 = grid
        .indexed_iter()
        .filter(|&(_, &x)| x == b'T')
        .map(|((y, x), _)| {
            grid.get([y, x + 1]).is_some_and(|&v| v == b'T') as u64
                + ((x + y + 1).is_multiple_of(2)
                    && grid.get([y + 1, x]).is_some_and(|&v| v == b'T'))
                    as u64
        })
        .sum();

    Number(ans as i64)
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
