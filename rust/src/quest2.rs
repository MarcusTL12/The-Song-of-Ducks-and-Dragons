use regex::Regex;

use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

type Int = i64;

fn cadd([x1, y1]: [Int; 2], [x2, y2]: [Int; 2]) -> [Int; 2] {
    [x1 + x2, y1 + y2]
}

fn cmul([x1, y1]: [Int; 2], [x2, y2]: [Int; 2]) -> [Int; 2] {
    [x1 * x2 - y1 * y2, x1 * y2 + y1 * x2]
}

fn cdiv([x1, y1]: [Int; 2], [x2, y2]: [Int; 2]) -> [Int; 2] {
    [x1 / x2, y1 / y2]
}

fn part1(input: String) -> QuestResult {
    let reg = Regex::new(r"A=\[(\d+),(\d+)\]").unwrap();

    let c = reg.captures(&input).unwrap();

    let re: i64 = c[1].parse().unwrap();
    let im: i64 = c[2].parse().unwrap();

    let a = [re, im];

    let mut x = [0, 0];

    for _ in 0..3 {
        x = cmul(x, x);
        x = cdiv(x, [10, 10]);
        x = cadd(x, a);
    }

    QuestResult::Text(format!("[{},{}]", x[0], x[1]))
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
