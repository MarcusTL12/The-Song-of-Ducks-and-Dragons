use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn find_block_count<I: Iterator<Item = u64>>(spell: I, wall_len: u64) -> u64 {
    spell.map(|n| wall_len / n).sum()
}

fn part1(input: String) -> QuestResult {
    let ans =
        find_block_count(input.split(',').map(|x| x.parse().unwrap()), 90);

    QuestResult::Number(ans as i64)
}

fn find_spell(input: String) -> Vec<u64> {
    let mut wall: Vec<u32> =
        input.split(',').map(|x| x.parse().unwrap()).collect();

    let mut spell = Vec::new();

    for n in 1.. {
        while wall.iter().skip(n - 1).step_by(n).all(|&x| x > 0) {
            spell.push(n as u64);
            for x in wall.iter_mut().skip(n - 1).step_by(n) {
                *x -= 1;
            }
        }

        if wall.iter().all(|&x| x == 0) {
            break;
        }
    }

    spell
}

fn part2(input: String) -> QuestResult {
    QuestResult::Number(find_spell(input).into_iter().product::<u64>() as i64)
}

fn part3(input: String) -> QuestResult {
    let spell = find_spell(input);

    let mut l = 1;
    let mut h = 2;

    const TARGET: u64 = 202520252025000;

    while find_block_count(spell.iter().cloned(), h) < TARGET {
        h *= 2;
    }

    while l + 1 < h {
        let m = l + (h - l) / 2;

        let x = find_block_count(spell.iter().cloned(), m);

        if x < TARGET {
            l = m;
        } else if x > TARGET {
            h = m;
        } else if x == TARGET {
            l = m;
            h = m;
        }
    }

    QuestResult::Number(l as i64)
}
