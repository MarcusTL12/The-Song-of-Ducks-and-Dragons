use ndarray::{ArrayView2, ArrayViewMut2, s};

pub fn input_to_grid(input: &[u8]) -> ArrayView2<'_, u8> {
    let w = input
        .iter()
        .enumerate()
        .find(|&(_, &x)| x == b'\n')
        .map(|(i, _)| i)
        .unwrap();

    let h = input.len() / (w + 1);

    ArrayView2::from_shape([h, w + 1], input)
        .unwrap()
        .slice_move(s![.., 0..w])
}

pub fn input_to_grid_mut(input: &mut [u8]) -> ArrayViewMut2<'_, u8> {
    let w = input
        .iter()
        .enumerate()
        .find(|&(_, &x)| x == b'\n')
        .map(|(i, _)| i)
        .unwrap();

    let h = input.len() / (w + 1);

    ArrayViewMut2::from_shape([h, w + 1], input)
        .unwrap()
        .slice_move(s![.., 0..w])
}

#[derive(Debug)]
pub struct Ranges {
    pub ranges: Vec<[i64; 2]>,
}

enum RangeSearchResult {
    In(usize),       // Completely contained in range at given index
    Void(usize),     // Completely disconnected from any ranges
    Left(usize),     // Not contained in any range but x - 1 is in range i - 1
    Right(usize),    // Not contained in any range but x + 1 is in range i
    Sandwich(usize), // Both Left and Right at the same time
}

impl Ranges {
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn find_point(&self, x: i64) -> Result<usize, usize> {
        use std::cmp::Ordering::*;
        self.ranges.binary_search_by(|&[a, b]| {
            if x < a {
                Greater
            } else if b < x {
                Less
            } else {
                Equal
            }
        })
    }

    fn analyze_point(&self, x: i64) -> RangeSearchResult {
        use RangeSearchResult::*;
        match self.find_point(x) {
            Ok(i) => In(i),
            Err(i) => {
                let left = i
                    .checked_sub(1)
                    .and_then(|j| self.ranges.get(j))
                    .is_some_and(|&[_, y]| x == y);

                let right =
                    self.ranges.get(i + 1).is_some_and(|&[y, _]| x == y);

                match [left, right] {
                    [false, false] => Void(i),
                    [true, false] => Left(i),
                    [false, true] => Right(i),
                    [true, true] => Sandwich(i),
                }
            }
        }
    }

    pub fn contains(&self, x: i64) -> bool {
        self.find_point(x).is_ok()
    }

    pub fn push(&mut self, [from, to]: [i64; 2]) {
        use RangeSearchResult::*;
        match [self.analyze_point(from), self.analyze_point(to)] {
            [In(i), In(j)] if i == j => {}
            [Void(i), Void(j)] if i == j => self.ranges.insert(i, [from, to]),
            [a, b] => {
                let (i, start) = match a {
                    In(i) => (i, self.ranges[i][0]),
                    Void(i) | Right(i) => (i, from),
                    Left(i) | Sandwich(i) => (i - 1, self.ranges[i - 1][0]),
                };

                let (j, stop) = match b {
                    In(j) | Right(j) | Sandwich(j) => (j, self.ranges[j][1]),
                    Void(j) | Left(j) => (j - 1, to),
                };

                self.ranges[i] = [start, stop];
                self.ranges.drain((i + 1)..=j);
            }
        }
    }

    pub fn remove_point(&mut self, x: i64) -> bool {
        if let Ok(i) = self.find_point(x) {
            if self.ranges[i][0] == self.ranges[i][1] {
                self.ranges.remove(i);
            } else if x == self.ranges[i][0] {
                self.ranges[i][0] += 1;
            } else if x == self.ranges[i][1] {
                self.ranges[i][1] -= 1;
            } else {
                let old_end = self.ranges[i][1];
                self.ranges[i][1] = x - 1;
                self.ranges.insert(i + 1, [x + 1, old_end]);
            }

            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = i64> {
        self.ranges.iter().flat_map(|&[a, b]| a..=b)
    }
}
