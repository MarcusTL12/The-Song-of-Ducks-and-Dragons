use crate::{
    Quest,
    QuestResult::{self, Number},
};

use z3::{Optimize, ast::Int};

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

fn get_energy_z3(
    plants: &[(i64, Vec<(usize, i64)>)],
    energies: &mut [Option<Int>],
    i: usize,
) -> Int {
    if let Some(energy) = &energies[i - 1] {
        return energy.clone();
    }

    let incoming: Int = plants[i - 1]
        .1
        .iter()
        .map(|&(j, t)| t * get_energy_z3(plants, energies, j))
        .sum();

    let energy = incoming
        .ge(plants[i - 1].0)
        .ite(&incoming, &Int::from_i64(0));

    energies[i - 1] = Some(energy.clone());

    energy
}

fn find_max_value_z3(plants: &[(i64, Vec<(usize, i64)>)]) -> i64 {
    let num_inputs = plants
        .iter()
        .take_while(|plant| {
            plant.0 == 1 && plant.1.len() == 1 && plant.1[0] == (0, 1)
        })
        .count();

    let mut plant_energies: Vec<_> = vec![None; plants.len()];

    let o = Optimize::new();

    for (i, e) in plant_energies.iter_mut().take(num_inputs).enumerate() {
        let new_inp = Int::new_const(format!("inp_{}", i + 1));
        o.assert(new_inp.eq(0) | new_inp.eq(1));
        *e = Some(new_inp);
    }

    let output = get_energy_z3(plants, &mut plant_energies, plants.len());

    o.maximize(&output);
    o.check(&[]);
    // dbg!(o.get_model().unwrap()); // To check which inputs solved

    o.get_upper(0).unwrap().as_int().unwrap().as_i64().unwrap()
}

fn part3(input: String) -> QuestResult {
    let (network_str, test_str) = input.split_once("\n\n\n").unwrap();
    let plants = parse_network(network_str);
    let mut energies = vec![None; plants.len()];

    let max_energy = find_max_value_z3(&plants);

    let ans = test_str
        .split('\n')
        .map(|l| {
            get_energy_for_test(
                l.split_ascii_whitespace().map(|x| x.parse().unwrap()),
                &plants,
                &mut energies,
            )
        })
        .filter(|&x| x != 0)
        .map(|e| max_energy - e)
        .sum();

    Number(ans)
}
