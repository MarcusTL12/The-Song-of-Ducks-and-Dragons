use rayon::prelude::*;

use crate::{Quest, QuestResult};

pub const PARTS: Quest = [part1, part2, part3];

fn part1(input: String) -> QuestResult {
    let mut lines = input.lines();

    let parent1 = lines.next().unwrap().split_once(':').unwrap().1.as_bytes();
    let parent2 = lines.next().unwrap().split_once(':').unwrap().1.as_bytes();
    let child = lines.next().unwrap().split_once(':').unwrap().1.as_bytes();

    let sim1 = parent1
        .iter()
        .zip(child.iter())
        .filter(|(a, b)| a == b)
        .count();

    let sim2 = parent2
        .iter()
        .zip(child.iter())
        .filter(|(a, b)| a == b)
        .count();

    QuestResult::Number((sim1 * sim2) as i64)
}

fn similarity_score(p1: &[u8], p2: &[u8], c: &[u8]) -> Option<u64> {
    let mut n1 = 0;
    let mut n2 = 0;

    for ((&a, &b), &c) in p1.iter().zip(p2.iter()).zip(c.iter()) {
        if c == a {
            n1 += 1;
        }

        if c == b {
            n2 += 1;
        }

        if c != a && c != b {
            return None;
        }
    }

    Some(n1 * n2)
}

fn part2(input: String) -> QuestResult {
    let scales: Vec<_> = input
        .lines()
        .map(|l| l.split_once(':').unwrap().1.as_bytes())
        .collect();

    let scales = scales.as_slice();

    let ans: u64 = scales
        .par_iter()
        .enumerate()
        .flat_map_iter(|(i, &c)| {
            scales
                .iter()
                .enumerate()
                .filter(move |&(j, _)| j != i)
                .flat_map(move |(j, a)| {
                    scales
                        .iter()
                        .enumerate()
                        .filter(move |&(k, _)| k != i)
                        .take(j)
                        .map(move |(_, b)| [a, b, c])
                })
        })
        .filter_map(|[a, b, c]| similarity_score(a, b, c))
        .sum();

    QuestResult::Number(ans as i64)
}

#[derive(Debug, Clone, Copy)]
enum Node {
    Root((usize, usize)),
    Pointer(usize),
}

fn get_root(tree: &mut [Option<Node>], i: usize) -> usize {
    match tree[i] {
        None => {
            tree[i] = Some(Node::Root((1, i + 1)));
            i
        }
        Some(Node::Root(_)) => i,
        Some(Node::Pointer(j)) => {
            let k = get_root(tree, j);

            tree[i] = Some(Node::Pointer(k));

            k
        }
    }
}

fn combine_nodes(tree: &mut [Option<Node>], src: usize, dest: usize) {
    let src = get_root(tree, src);
    let dest = get_root(tree, dest);

    if src == dest {
        return;
    }

    let Some(Node::Root((src_n, src_s))) = tree[src] else {
        unreachable!()
    };

    let Some(Node::Root((n, s))) = tree.get_mut(dest).unwrap() else {
        unreachable!()
    };

    *n += src_n;
    *s += src_s;

    tree[src] = Some(Node::Pointer(dest));
}

fn recurse_tree(tree: &mut [Option<Node>], scales: &[&[u8]], i: usize) {
    if tree[i].is_some() {
        return;
    }

    for (j, &a) in scales.iter().enumerate() {
        if j == i {
            continue;
        }

        for (k, &b) in scales.iter().enumerate().take(j) {
            if k == i {
                continue;
            }

            if similarity_score(a, b, scales[i]).is_some() {
                recurse_tree(tree, scales, j);
                recurse_tree(tree, scales, k);

                combine_nodes(tree, j, i);
                combine_nodes(tree, k, i);

                return;
            }
        }
    }
}

fn part3(input: String) -> QuestResult {
    let scales: Vec<_> = input
        .lines()
        .map(|l| l.split_once(':').unwrap().1.as_bytes())
        .collect();

    let mut tree = vec![None; scales.len()];

    for i in 0..scales.len() {
        recurse_tree(&mut tree, &scales, i);
    }

    let ans = tree
        .into_iter()
        .flatten()
        .filter_map(|x| if let Node::Root(x) = x { Some(x) } else { None })
        .max_by_key(|&(n, _)| n)
        .unwrap()
        .1;

    QuestResult::Number(ans as i64)
}
