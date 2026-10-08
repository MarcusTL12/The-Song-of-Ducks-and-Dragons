use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn get_energy(plants: &[(u64, Vec<(usize, u64)>)], i: usize) -> u64 {
    todo!()
}

fn part1(input: String) -> QuestResult {
    let plants: Vec<_> = input
        .split("\n\n")
        .map(|plant_spec| {
            let mut lines = plant_spec.split('\n');

            let mut plant_thickness_str = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .chars();

            plant_thickness_str.next_back();

            let plant_thickness: u64 =
                plant_thickness_str.as_str().parse().unwrap();

            let branches: Vec<_> = lines
                .map(|l| {
                    if l == "- free branch with thickness 1" {
                        (0, 1)
                    } else {
                        let mut parts = l.split_ascii_whitespace();
                        let target: usize =
                            parts.nth(4).unwrap().parse().unwrap();
                        let thickness: u64 =
                            parts.nth(2).unwrap().parse().unwrap();

                        (target, thickness)
                    }
                })
                .collect();

            (plant_thickness, branches)
        })
        .collect();

    println!("{plants:?}");

    todo!()
}

fn part2(input: String) -> QuestResult {
    todo!("\n{input}")
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
