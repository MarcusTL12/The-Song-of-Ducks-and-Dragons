use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut crates: Vec<u32> =
        input.split(',').map(|x| x.parse().unwrap()).collect();

    crates.sort();

    let mut cursize = 0;
    let mut cursum = 0;

    for x in crates {
        if x > cursize {
            cursize = x;
            cursum += x;
        }
    }

    QuestResult::Number(cursum as i64)
}

fn part2(input: String) -> QuestResult {
    let mut crates: Vec<u32> =
        input.split(',').map(|x| x.parse().unwrap()).collect();

    crates.sort();

    let mut numcrates = 0;
    let mut cursize = 0;
    let mut cursum = 0;

    for x in crates {
        if x > cursize {
            numcrates += 1;
            cursize = x;
            cursum += x;
        }

        if numcrates == 20 {
            break;
        }
    }

    QuestResult::Number(cursum as i64)
}

fn part3(input: String) -> QuestResult {
    let mut crates: Vec<u32> =
        input.split(',').map(|x| x.parse().unwrap()).collect();

    crates.sort();

    let mut numsets = 0;
    let mut numleft = crates.len();

    while numleft > 0 {
        numsets += 1;
        let mut cursize = 0;

        for x in crates.iter_mut() {
            if *x > 0 && *x > cursize {
                cursize = *x;
                *x = 0;
                numleft -= 1;
            }
        }
    }

    QuestResult::Number(numsets as i64)
}
