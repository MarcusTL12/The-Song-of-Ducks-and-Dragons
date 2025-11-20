use itertools::{Itertools, MinMaxResult};
use rayon::str::ParallelString;

use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

type Fishbone = Vec<(u8, Option<u8>, Option<u8>)>;

fn construct_fishbone(line: &str) -> (usize, Fishbone) {
    let (id, numbers) = line.split_once(':').unwrap();

    let id = id.parse().unwrap();

    let mut numbers = numbers.split(',').map(|x| x.parse::<u8>().unwrap());

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

    (id, fishbone)
}

fn digits_to_num<I: Iterator<Item = u8>>(it: I) -> u64 {
    it.fold(0, |x, d| 10 * x + d as u64)
}

fn compute_quality(line: &str) -> u64 {
    let (_, fishbone) = construct_fishbone(line);

    digits_to_num(fishbone.into_iter().map(|(d, _, _)| d))
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

fn compute_order_parameters(id: usize, fishbone: Fishbone) -> Vec<u64> {
    let mut params = vec![digits_to_num(fishbone.iter().map(|&(d, _, _)| d))];

    params.extend(fishbone.into_iter().map(|(spine, left, right)| {
        digits_to_num([left, Some(spine), right].into_iter().flatten())
    }));

    params.push(id as u64);

    params
}

fn part3(input: String) -> QuestResult {
    let mut params: Vec<_> = input
        .lines()
        .map(construct_fishbone)
        .map(|(id, fishbone)| compute_order_parameters(id, fishbone))
        .collect();

    params.sort();

    let ans: u64 = params
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, params)| (i + 1) as u64 * params.last().unwrap())
        .sum();

    QuestResult::Number(ans as i64)
}
