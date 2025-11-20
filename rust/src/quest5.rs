use crate::{Quest, QuestResult};
use std::io::Write;

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut numbers = input
        .split_once(':')
        .unwrap()
        .1
        .split(',')
        .map(|x| x.parse::<u8>().unwrap());

    let mut fishbone = vec![(numbers.next().unwrap(), None, None)];
    let mut buf = Vec::new();
    write!(&mut buf, "{}", fishbone[0].0).unwrap();

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
            write!(&mut buf, "{x}").unwrap();
        }
    }

    QuestResult::Text(String::from_utf8(buf).unwrap())
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
