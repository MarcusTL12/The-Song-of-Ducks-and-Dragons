use rayon::prelude::*;
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

fn parse_input(input: &str) -> [Int; 2] {
    let reg = Regex::new(r"A=\[(-?\d+),(-?\d+)\]").unwrap();

    let c = reg.captures(input).unwrap();

    let re: i64 = c[1].parse().unwrap();
    let im: i64 = c[2].parse().unwrap();

    [re, im]
}

fn part1(input: String) -> QuestResult {
    let a = parse_input(&input);

    let mut x = [0, 0];

    for _ in 0..3 {
        x = cmul(x, x);
        x = cdiv(x, [10, 10]);
        x = cadd(x, a);
    }

    QuestResult::Text(format!("[{},{}]", x[0], x[1]))
}

fn should_engrave(&a: &[Int; 2]) -> bool {
    let mut x = [0, 0];

    for _ in 0..100 {
        x = cmul(x, x);
        x = cdiv(x, [100_000, 100_000]);
        x = cadd(x, a);

        if !x.iter().all(|q| (-1_000_000..=1_000_000).contains(q)) {
            return false;
        }
    }

    true
}

fn part2(input: String) -> QuestResult {
    let [x0, y0] = parse_input(&input);

    let ans = (x0..)
        .step_by(10)
        .take(101)
        .flat_map(|x| (y0..).step_by(10).take(101).map(move |y| [x, y]))
        .filter(should_engrave)
        .count();

    QuestResult::Number(ans as i64)
}

fn part3(input: String) -> QuestResult {
    let [x0, y0] = parse_input(&input);

    let ans = (x0..=x0 + 1000)
        .into_par_iter()
        .flat_map(|x| (y0..=y0 + 1000).into_par_iter().map(move |y| [x, y]))
        .filter(should_engrave)
        .count();

    QuestResult::Number(ans as i64)
}
