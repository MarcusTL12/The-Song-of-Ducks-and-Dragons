use ndarray::ArrayViewMut2;

use crate::{Quest, QuestResult, util::input_to_grid_mut};

pub const PARTS: Quest = [part1, part2, part3];

fn recurse(
    grid: &mut ArrayViewMut2<u8>,
    n: usize,
    [i, j]: [usize; 2],
) -> usize {
    if n == 0 {
        return 0;
    }

    let mut sheep = 0;

    for [nx, ny] in [[1, 2], [2, 1]] {
        for sx in [-1, 1] {
            let dx = nx * sx;
            let x = (i as isize + dx) as usize;

            for sy in [-1, 1] {
                let dy = ny * sy;
                let y = (j as isize + dy) as usize;

                if let Some(tile) = grid.get_mut([x, y]) {
                    if *tile == b'S' {
                        *tile = b'X';
                        sheep += 1;
                    }
                    sheep += recurse(grid, n - 1, [x, y]);
                }
            }
        }
    }

    sheep
}

fn part1(input: String) -> QuestResult {
    let mut input = input.into_bytes();
    input.push(b'\n');
    let mut grid = input_to_grid_mut(&mut input);

    let ((i, j), _) = grid.indexed_iter().find(|&(_, &x)| x == b'D').unwrap();

    let sheep = recurse(&mut grid, 4, [i, j]);

    QuestResult::Number(sheep as i64)
}

fn propagate_dragon(grid: &mut ArrayViewMut2<u8>) -> usize {
    let (h, w) = grid.dim();

    for i in 0..h {
        for j in 0..w {
            if (grid[[i, j]] & 0b0000_0001) == 0 {
                continue;
            }

            grid[[i, j]] &= 0b1111_1110;

            for [nx, ny] in [[1, 2], [2, 1]] {
                for sx in [-1, 1] {
                    let dx = nx * sx;
                    let x = (j as isize + dx) as usize;

                    for sy in [-1, 1] {
                        let dy = ny * sy;
                        let y = (i as isize + dy) as usize;

                        if let Some(tile) = grid.get_mut([y, x]) {
                            *tile |= 0b0000_1000;
                        }
                    }
                }
            }
        }
    }

    let mut eaten = 0;

    for x in grid.iter_mut() {
        if (*x & 0b0000_1000) != 0 {
            if (*x & 0b0000_0110) == 0b0000_0010 {
                eaten += 1;
                *x = (*x & 0b1111_0101) | 0b0000_0001;
            } else {
                *x = (*x & 0b1111_0111) | 0b0000_0001;
            }
        }
    }

    eaten
}

fn propagate_sheep(grid: &mut ArrayViewMut2<u8>) -> usize {
    let (h, w) = grid.dim();

    let mut eaten = 0;

    for i in (1..h).rev() {
        for j in 0..w {
            let mut tile = grid[[i, j]];
            if (grid[[i - 1, j]] & 0b0000_0010) != 0 {
                tile |= 0b0000_0010;
                if (tile & 0b0000_0101) == 0b0000_0001 {
                    eaten += 1;
                    tile &= 0b1111_1101;
                }
            } else {
                tile &= 0b1111_1101;
            }

            grid[[i, j]] = tile;
        }
    }

    for j in 0..w {
        grid[[0, j]] &= 0b1111_1101;
    }

    eaten
}

fn part2(input: String) -> QuestResult {
    let mut input = input.into_bytes();
    input.push(b'\n');

    for x in &mut input {
        *x = match *x {
            b'.' => 0b0000_0000,
            b'D' => 0b0000_0001,
            b'S' => 0b0000_0010,
            b'#' => 0b0000_0100,
            _ => *x,
        }
    }

    let mut grid = input_to_grid_mut(&mut input);

    let mut eaten = 0;

    for _ in 0..20 {
        eaten += propagate_dragon(&mut grid);
        eaten += propagate_sheep(&mut grid);
    }

    QuestResult::Number(eaten as i64)
}

const W: usize = 3;
const H: usize = 4;

fn count_solutions_sheep(
    sheep: [u8; W],
    dragon: [u8; 2],
    hideouts: u64,
) -> usize {
    let mut sol = 0;

    let mut legal_moves = false;

    for (i, &s) in sheep.iter().enumerate() {
        if s != 255
            && (dragon[0] != i as u8
                || dragon[1] != s + 1
                || hideouts & (1 << (s * 8 + i as u8)) != 0)
        {
            legal_moves = true;

            let mut new_sheep = sheep;

            new_sheep[i] += 1;

            if new_sheep[i] < H as u8 {
                sol += count_solutions_dragon(new_sheep, dragon, hideouts);
            }
        }
    }

    if !legal_moves {
        sol = count_solutions_dragon(sheep, dragon, hideouts);
    }

    sol
}

fn count_solutions_dragon(
    sheep: [u8; W],
    [x, y]: [u8; 2],
    hideouts: u64,
) -> usize {
    let mut sol = 0;

    for [nx, ny] in [[1, 2], [2, 1]] {
        for sx in [false, true] {
            let new_x = match sx {
                false => {
                    if nx > x {
                        continue;
                    } else {
                        x - nx
                    }
                }
                true => {
                    if x + nx >= W as u8 {
                        continue;
                    } else {
                        x + nx
                    }
                }
            };

            for sy in [false, true] {
                let new_y = match sy {
                    false => {
                        if ny > y {
                            continue;
                        } else {
                            y - ny
                        }
                    }
                    true => {
                        if y + ny >= H as u8 {
                            continue;
                        } else {
                            y + ny
                        }
                    }
                };

                let mut new_sheep = sheep;

                if sheep[new_x as usize] == new_y
                    && hideouts & (1 << (new_x + new_y * 8)) == 0
                {
                    new_sheep[new_x as usize] = 255;
                }

                if new_sheep.iter().all(|&x| x == 255) {
                    sol += 1;
                } else {
                    sol += count_solutions_sheep(
                        new_sheep,
                        [new_x, new_y],
                        hideouts,
                    );
                }
            }
        }
    }

    sol
}

fn part3(input: String) -> QuestResult {
    let mut sheep = [255; W];
    let mut dragon = [255; 2];
    let mut hideouts: u64 = 0;

    let mut lines = input.lines();

    for (i, c) in lines.next().unwrap().chars().enumerate() {
        if c == 'S' {
            sheep[i] = 0;
        }
    }

    let mut base_bit = 1 << 8;

    for l in lines {
        let mut bit = base_bit;

        for c in l.chars() {
            if c == '#' {
                hideouts |= bit;
            } else if c == 'D' {
                let i = bit.trailing_zeros() as u8;
                dragon = [i % 8, i / 8];
            }
            bit <<= 1;
        }

        base_bit <<= 8;
    }

    println!("dragon: {dragon:?}");
    println!("sheep: {sheep:?}");
    for x in &u64::to_ne_bytes(hideouts)[0..H] {
        println!("{x:03b}");
    }

    let ans = count_solutions_sheep(sheep, dragon, hideouts);

    QuestResult::Number(ans as i64)
}
