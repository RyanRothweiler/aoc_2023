#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use crate::perma::twod::TwoD;
use std::convert::From;

pub fn run() {
    let v = calc("resources/inputs/day_14.txt");
    println!("{v}");
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum CellType {
    // boulders roll
    Boulder,
    // Rocks are square and don't roll
    Rock,
    // Empty space
    Ground,
}

impl From<char> for CellType {
    fn from(input: char) -> Self {
        match input {
            'O' => return CellType::Boulder,
            '.' => return CellType::Ground,
            '#' => return CellType::Rock,
            _ => panic!("Unknown cell type"),
        }
    }
}

fn calc(file_dir: &str) -> i64 {
    let contents = std::fs::read_to_string(file_dir).unwrap();

    let lines: Vec<&str> = contents.lines().collect();
    let first_chars: Vec<char> = lines[0].trim().chars().collect();
    let mut map: TwoD<CellType> = TwoD::new(first_chars.len(), lines.len(), CellType::Ground);

    for y in 0..map.height() {
        let line: &str = lines[y].trim();
        let chars: Vec<char> = line.trim().chars().collect();

        for x in 0..map.width() {
            map.set(x, y, CellType::from(chars[x]));
        }
    }

    // count of boulders in each row
    let mut boulders_count: Vec<i64> = vec![0; map.width()];

    for x in 0..map.width() {
        let mut next_rock_slot = 0;

        for y in 0..map.height() {
            let cell = *map.get(x, y).unwrap();
            match cell {
                CellType::Boulder => {
                    boulders_count[next_rock_slot] += 1;
                    next_rock_slot += 1;
                }
                CellType::Ground => {}
                CellType::Rock => {
                    next_rock_slot = y + 1;
                }
            }
        }
    }

    let mut sum: i64 = 0;
    for i in 0..boulders_count.len() {
        let row: i64 = boulders_count.len() as i64 - i as i64;
        let curr = boulders_count[i] * row;
        sum += curr;
    }

    return sum;
}

#[test]
fn sample() {
    assert_eq!(calc("resources/day_14/day_14_sample.txt"), 136);
}
