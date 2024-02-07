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
    //let contents = std::fs::read_to_string("resources/inputs/day_14.txt").unwrap();
    let contents = std::fs::read_to_string("resources/day_14/day_14_sample.txt").unwrap();

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

    let mut v = 0;
    for i in 0..1 {
        v = calc(&mut map);
    }
    //println!("{v}");
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum CellType {
    // boulders roll
    Boulder,
    // Rocks are square and don't roll
    Rock,
    // Empty space
    Ground,
}

impl std::fmt::Debug for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CellType::Boulder => {
                write!(f, "O")
            }
            CellType::Rock => {
                write!(f, "#")
            }
            CellType::Ground => {
                write!(f, ".")
            }
        }
    }
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

// for debugging
fn print_map(map: &mut TwoD<CellType>) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            let c = map.get(x, y).unwrap();
            print!("{c:?}");
        }
        println!("");
    }
}

fn slide(map: &mut TwoD<CellType>) {
    for x in 0..map.width() {
        let mut next_rock_slot = 0;

        for y in 0..map.height() {
            let cell = *map.get(x, y).unwrap();
            match cell {
                CellType::Boulder => {
                    map.set(x, next_rock_slot, CellType::Boulder);

                    if y != next_rock_slot {
                        map.set(x, y, CellType::Ground);
                    }

                    next_rock_slot += 1;
                }
                CellType::Ground => {}
                CellType::Rock => {
                    next_rock_slot = y + 1;
                }
            }
        }
    }
}

fn calc(map: &mut TwoD<CellType>) -> i64 {
    // count of boulders in each row
    let mut boulders_count: Vec<i64> = vec![0; map.width()];

    slide(map);

    print_map(map);

    let mut sum: i64 = 0;
    /*
    for i in 0..boulders_count.len() {
        let row: i64 = boulders_count.len() as i64 - i as i64;
        let curr = boulders_count[i] * row;
        sum += curr;
    }
    */

    return sum;
}

/*
#[test]
fn sample() {
    assert_eq!(calc("resources/day_14/day_14_sample.txt"), 136);
}
*/
