use std::collections::HashMap;

use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn check_rules([from, to]: [u8; 2], rules: &[&[u8]; 256]) -> bool {
    rules[from as usize].contains(&to)
}

fn fits_rules(name: &[u8], rules: &[&[u8]; 256]) -> bool {
    name.array_windows().all(|&x| check_rules(x, rules))
}

fn parse_input(input: &str) -> (&str, [&[u8]; 256]) {
    let (names, rules) = input.split_once("\n\n").unwrap();

    let mut rule_lookup: [&[u8]; 256] = [&[]; 256];

    for l in rules.lines() {
        let (from, to) = l.split_once(" > ").unwrap();

        rule_lookup[from.as_bytes().iter().cloned().next().unwrap() as usize] =
            to.as_bytes();
    }

    (names, rule_lookup)
}

fn part1(input: String) -> QuestResult {
    let (names, rule_lookup) = parse_input(&input);

    let ans = names
        .split(',')
        .find(|name| fits_rules(name.as_bytes(), &rule_lookup))
        .unwrap();

    QuestResult::Text(ans.to_owned())
}

fn part2(input: String) -> QuestResult {
    let (names, rule_lookup) = parse_input(&input);

    let ans = names
        .split(',')
        .enumerate()
        .filter(|(_, name)| fits_rules(name.as_bytes(), &rule_lookup))
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    QuestResult::Number(ans as i64)
}

fn recurse_possibilities(
    memo: &mut HashMap<(usize, u8), usize>,
    curlen: usize,
    curlast: u8,
    rules: &[&[u8]; 256],
) -> usize {
    if curlen == 11 {
        return 1;
    }

    let k = (curlen, curlast);

    if let Some(&x) = memo.get(&k) {
        return x;
    }

    let ans = (if curlen >= 7 { 1 } else { 0 })
        + rules[curlast as usize]
            .iter()
            .cloned()
            .filter(|&x| x != b',')
            .map(|next| recurse_possibilities(memo, curlen + 1, next, rules))
            .sum::<usize>();

    memo.insert(k, ans);

    ans
}

fn part3(input: String) -> QuestResult {
    let (prefixes, rule_lookup) = parse_input(&input);

    let mut memo = HashMap::new();

    let ans = prefixes
        .split(',')
        .filter(|prefix| fits_rules(prefix.as_bytes(), &rule_lookup))
        .filter(|&prefix| {
            !prefixes
                .split(',')
                .any(|other| prefix != other && prefix.starts_with(other))
        })
        .map(|prefix| {
            recurse_possibilities(
                &mut memo,
                prefix.len(),
                *prefix.as_bytes().last().unwrap(),
                &rule_lookup,
            )
        })
        .sum::<usize>();

    QuestResult::Number(ans as i64)
}
