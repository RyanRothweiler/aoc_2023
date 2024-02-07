#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::perma::twod::TwoD;
use std::convert::From;

pub fn run() {
    let contents = std::fs::read_to_string("resources/inputs/day_14.txt").unwrap();
    let mut map = build_map(&contents);
    let v = calc(&mut map, 1_000_000_000);
    println!("{v}");
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

fn build_map(file_data: &str) -> TwoD<CellType> {
    let lines: Vec<&str> = file_data.lines().collect();
    let first_chars: Vec<char> = lines[0].trim().chars().collect();
    let mut map: TwoD<CellType> = TwoD::new(first_chars.len(), lines.len(), CellType::Ground);

    for y in 0..map.height() {
        let line: &str = lines[y].trim();
        let chars: Vec<char> = line.trim().chars().collect();

        for x in 0..map.width() {
            map.set(x, y, CellType::from(chars[x]));
        }
    }

    return map;
}

// -1 for down 1 for up
fn slide_vert(map: &mut TwoD<CellType>, dir: i64) {
    for x in 0..map.width() {
        let mut next_rock_slot: i64;
        if dir < 0 {
            next_rock_slot = (map.height() - 1) as i64;
        } else {
            next_rock_slot = 0;
        }

        let mut lp: Vec<usize> = (0..map.height()).collect();
        if dir < 0 {
            lp.reverse();
        }

        for y in lp {
            let cell = *map.get(x, y).unwrap();
            match cell {
                CellType::Boulder => {
                    map.set(x, next_rock_slot as usize, CellType::Boulder);

                    if y != next_rock_slot as usize {
                        map.set(x, y, CellType::Ground);
                    }

                    next_rock_slot += dir;
                }
                CellType::Ground => {}
                CellType::Rock => {
                    next_rock_slot = y as i64 + dir;
                }
            }
        }
    }
}

// -1 for right 1 for left
fn slide_hor(map: &mut TwoD<CellType>, dir: i64) {
    for y in 0..map.height() {
        let mut next_rock_slot: i64;
        if dir < 0 {
            next_rock_slot = (map.width() - 1) as i64;
        } else {
            next_rock_slot = 0;
        }

        let mut lp: Vec<usize> = (0..map.width()).collect();
        if dir < 0 {
            lp.reverse();
        }

        for x in lp {
            let cell = *map.get(x, y).unwrap();
            match cell {
                CellType::Boulder => {
                    map.set(next_rock_slot as usize, y, CellType::Boulder);

                    if x != next_rock_slot as usize {
                        map.set(x, y, CellType::Ground);
                    }

                    next_rock_slot += dir;
                }
                CellType::Ground => {}
                CellType::Rock => {
                    next_rock_slot = x as i64 + dir;
                }
            }
        }
    }
}

fn calc_weight(map: &mut TwoD<CellType>) -> i64 {
    let mut boulders_count: Vec<i64> = vec![0; map.width()];
    for y in 0..map.height() {
        for x in 0..map.width() {
            if *map.get(x, y).unwrap() == CellType::Boulder {
                boulders_count[y] += 1;
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

fn cycle(map: &mut TwoD<CellType>) {
    slide_vert(map, 1);
    slide_hor(map, 1);
    slide_vert(map, -1);
    slide_hor(map, -1);
}

fn calc(map: &mut TwoD<CellType>, cycles: i64) -> i64 {
    let mut cycle_start: i64 = 0;
    let mut cycle_len: i64 = 0;

    // loop until we find a cycle
    let mut past_maps: Vec<TwoD<CellType>> = vec![];
    let mut i = 0;
    //for c in 0..cycles {
    loop {
        cycle(map);
        i += 1;

        if past_maps.contains(map) {
            cycle_start = past_maps.iter().position(|x| x == map).unwrap() as i64;
            // -1 so that we don't count this current cycle.
            // One less steps to get back to this map.
            cycle_len = i - cycle_start - 1;
            break;
        }

        past_maps.push(map.clone());
    }

    let cycles_remain = cycles - i;
    let remain_after_loop = cycles_remain % cycle_len;

    for c in 0..(remain_after_loop) {
        cycle(map);
    }

    return calc_weight(map);
}

#[test]
fn sample_up() {
    let contents = std::fs::read_to_string("resources/day_14/day_14_sample.txt").unwrap();
    let mut map = build_map(&contents);
    slide_vert(&mut map, 1);

    assert_eq!(calc_weight(&mut map), 136);
}

#[test]
fn sample_cycle() {
    let contents = std::fs::read_to_string("resources/day_14/day_14_sample.txt").unwrap();
    let mut map = build_map(&contents);

    assert_eq!(calc(&mut map, 1_000_000_000), 64);
}
