use std::{env, fmt::Display, fs::read_to_string, time::Instant};

use home::home_dir;

// mod util;

mod quest1;
mod quest2;

#[derive(Debug)]
enum QuestResult {
    Text(String),
    Number(i64),
}

impl Display for QuestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(s) => s.fmt(f),
            Self::Number(n) => n.fmt(f),
        }
    }
}

type Quest = [fn(String) -> QuestResult; 3];

const QUESTS: &[Quest] = &[
    quest1::PARTS,
    quest2::PARTS,
];

fn load_input(quest: usize, part: usize, example: usize) -> String {
    let path = home_dir().unwrap().join("ec-input").join(if example == 0 {
        format!("everybody_codes_e2025_q{quest:02}_p{part}.txt")
    } else {
        format!("everybody_codes_e2025_q{quest:02}_ex{example}.txt")
    });

    read_to_string(path).unwrap()
}

fn main() {
    let mut args = env::args();

    args.next();

    let quest: usize = args
        .next()
        .expect("Give quest number as first cli argument")
        .parse()
        .expect("Quest number not numeric");

    let part: usize = args
        .next()
        .expect("Give quest part as second cli argument")
        .parse()
        .expect("Quest part not numeric");

    let example: usize = args.next().map(|s| s.parse().unwrap()).unwrap_or(0);

    let t = Instant::now();

    let input = load_input(quest, part, example);

    let t_load = t.elapsed();

    println!("Loading input took: {t_load:?}");

    let t = Instant::now();

    let result = QUESTS[quest - 1][part - 1](input);

    let t_solve = t.elapsed();

    println!("{result}");

    println!("Solving took: {t_solve:?}");
}
