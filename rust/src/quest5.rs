use itertools::{Itertools, MinMaxResult};

use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn compute_quality(line: &str) -> u64 {
    let mut numbers = line
        .split_once(':')
        .unwrap()
        .1
        .split(',')
        .map(|x| x.parse::<u8>().unwrap());

    let mut fishbone = vec![(numbers.next().unwrap(), None, None)];

    for x in numbers {
        let mut found_place = false;
        for (spine, left, right) in fishbone.iter_mut() {
            if left.is_none() && x < *spine {
                *left = Some(x);
                found_place = true;
                break;
            } else if right.is_none() && x > *spine {
                *right = Some(x);
                found_place = true;
                break;
            }
        }

        if !found_place {
            fishbone.push((x, None, None));
        }
    }

    fishbone
        .into_iter()
        .fold(0, |x, (d, _, _)| 10 * x + d as u64)
}

fn part1(input: String) -> QuestResult {
    QuestResult::Number(compute_quality(&input) as i64)
}

fn part2(input: String) -> QuestResult {
    let ans = match input.lines().map(compute_quality).minmax() {
        MinMaxResult::NoElements => panic!(),
        MinMaxResult::OneElement(_) => 0,
        MinMaxResult::MinMax(lo, hi) => hi - lo,
    };

    QuestResult::Number(ans as i64)
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
