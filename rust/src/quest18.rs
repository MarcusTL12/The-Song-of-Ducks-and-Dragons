use crate::{
    Quest,
    QuestResult::{self, Number},
};

pub const PARTS: Quest = [part1, part2, part3];

fn get_energy(
    plants: &[(i64, Vec<(usize, i64)>)],
    energies: &mut [Option<i64>],
    i: usize,
) -> i64 {
    if let Some(energy) = energies[i - 1] {
        return energy;
    }

    let incoming: i64 = plants[i - 1]
        .1
        .iter()
        .map(|&(j, t)| t * get_energy(plants, energies, j))
        .sum();

    let energy = if incoming >= plants[i - 1].0 {
        incoming
    } else {
        0
    };

    energies[i - 1] = Some(energy);

    energy
}

fn parse_network(network_str: &str) -> Vec<(i64, Vec<(usize, i64)>)> {
    network_str
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

            let plant_thickness: i64 =
                plant_thickness_str.as_str().parse().unwrap();

            let branches: Vec<_> = lines
                .map(|l| {
                    if l == "- free branch with thickness 1" {
                        (0, 1)
                    } else {
                        let mut parts = l.split_ascii_whitespace();
                        let target: usize =
                            parts.nth(4).unwrap().parse().unwrap();
                        let thickness: i64 =
                            parts.nth(2).unwrap().parse().unwrap();

                        (target, thickness)
                    }
                })
                .collect();

            (plant_thickness, branches)
        })
        .collect()
}

fn part1(input: String) -> QuestResult {
    let plants = parse_network(&input);

    let mut energies = vec![None; plants.len()];

    for (plant, energy) in plants.iter().zip(&mut energies) {
        if plant.0 == 1 && plant.1.len() == 1 && plant.1[0] == (0, 1) {
            *energy = Some(1);
        }
    }

    let ans = get_energy(&plants, &mut energies, plants.len());

    Number(ans as i64)
}

fn get_energy_for_test<I: Iterator<Item = i64>>(
    test: I,
    plants: &[(i64, Vec<(usize, i64)>)],
    energies: &mut [Option<i64>],
) -> i64 {
    energies.fill(None);

    for (x, e) in test.zip(energies.iter_mut()) {
        *e = Some(x);
    }

    get_energy(plants, energies, plants.len())
}

fn part2(input: String) -> QuestResult {
    let (network_str, test_str) = input.split_once("\n\n\n").unwrap();
    let plants = parse_network(network_str);

    let mut energies = vec![None; plants.len()];

    let ans = test_str
        .split('\n')
        .map(|l| {
            get_energy_for_test(
                l.split_ascii_whitespace().map(|x| x.parse().unwrap()),
                &plants,
                &mut energies,
            )
        })
        .sum();

    Number(ans)
}

fn part3(input: String) -> QuestResult {
    todo!("\n{input}")
}
