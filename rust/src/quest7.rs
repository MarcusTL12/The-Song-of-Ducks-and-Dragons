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

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
