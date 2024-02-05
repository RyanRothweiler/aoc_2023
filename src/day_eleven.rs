/*
 * Some improvements
 * - use a struct instead of the tuples.
 * - can probably expand the rows / colums to work in with any dimensions.
 * - double counting the pairs.
 */

#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use crate::perma::twod::TwoD;
use crate::perma::v2::V2;
 
pub fn run() {
    let v = calculate_distance("resources/inputs/day_11.txt", 1_000_000);
    println!("{v}");
}

// for debugging
fn print_map(map: &mut TwoD<char>) {
    for y in 0..map.height() {
        for x in 0..map.width() {
            let c = map.get(x, y).unwrap();
            print!("{c}");
        }
        println!("");
    }
}

struct Star {
    orig: V2,
    expanded: V2,
}

fn get_empty_cols_rows(map: &mut TwoD<char>) -> (Vec<i64>, Vec<i64>) {
    // find empty cols
    let mut empty_cols: Vec<i64> = vec![];
    for x in 0..map.width() {
        let mut empty = true;

        for y in 0..map.height() {
            if *map.get(x, y).unwrap() == '#' {
                empty = false;
                break;
            }
        }

        if empty {
            empty_cols.push(x as i64);
        }
    }

    // find empty rows
    let mut empty_rows: Vec<i64> = vec![];
    for y in 0..map.height() {
        let mut empty = true;

        for x in 0..map.width() {
            if *map.get(x, y).unwrap() == '#' {
                empty = false;
                break;
            }
        }

        if empty {
            empty_rows.push(y as i64);
        }
    }

    return (empty_cols, empty_rows);
}

fn build_map(input: &str) -> (TwoD<char>, Vec<Star>) {
    let lines: Vec<&str> = input.split('\r').collect();

    let chars_trimmed: Vec<char> = lines[0].trim().chars().collect();
    let width = chars_trimmed.len();
    let height = lines.len();

    let mut stars: Vec<Star> = vec![];

    // -1 because of the extra end line
    let mut map_start: TwoD<char> = TwoD::new(width, height, 'X');

    for y in 0..height {
        let line = lines[y];

        let chars: Vec<char> = line.trim().chars().collect();
        if chars.len() <= 1 {
            continue;
        }

        for x in 0..width {
            let c = chars[x];

            map_start.set(x, y, chars[x]);

            if chars[x] == '#' {
                stars.push(Star {
                    orig: V2::new(x as i64, y as i64),
                    expanded: V2::new(x as i64, y as i64),
                });
            }
        }
    }

    return (map_start, stars);
}

// replace 1 empty row / col with the expansion_rate val
fn expand_stars(empties: (Vec<i64>, Vec<i64>), stars: &mut Vec<Star>, expansion_rate: i64) {
    // expand out star positions
    for r in empties.0 {
        for s in stars.iter_mut() {
            if s.orig.x > r {
                s.expanded.x += expansion_rate - 1;
            }
        }
    }
    for r in empties.1 {
        for s in stars.iter_mut() {
            if s.orig.y > r {
                s.expanded.y += expansion_rate - 1;
            }
        }
    }
}

// this does double the amount of work needed. It counts each pair twice.
fn sum_lengths(stars: &Vec<Star>) -> i64 {
    let mut sum: i64 = 0;
    let mut c = 0;
    for first in stars.into_iter() {
        for second in stars.into_iter() {
            if first.expanded != second.expanded {
                c += 1;
                sum += V2::manhattan_dist(&first.expanded, &second.expanded);
            }
        }
    }

    return sum / 2;
}

fn calculate_distance(input: &str, expansion_rate: i64) -> i64 {
    let contents = std::fs::read_to_string(input).unwrap();

    let map = build_map(&contents);
    let mut map_start = map.0;
    let mut stars = map.1;

    let empties = get_empty_cols_rows(&mut map_start);

    expand_stars(empties, &mut stars, expansion_rate);

    return sum_lengths(&stars);
}

#[test]
fn sample_part_one() {
    let contents = std::fs::read_to_string("resources/day_11/day_11_sample.txt").unwrap();

    let map = build_map(&contents);
    let mut map_start = map.0;
    let mut stars = map.1;

    assert_eq!(stars.len(), 9);

    let empties = get_empty_cols_rows(&mut map_start);

    assert_eq!(empties.0.len(), 3);
    assert_eq!(empties.0[0], 2);
    assert_eq!(empties.0[1], 5);
    assert_eq!(empties.0[2], 8);

    expand_stars(empties, &mut stars, 2);

    assert_eq!(stars[0].expanded, V2::new(4, 0));
    assert_eq!(stars[1].expanded, V2::new(9, 1));
    assert_eq!(stars[2].expanded, V2::new(0, 2));
    assert_eq!(stars[3].expanded, V2::new(8, 5));
    assert_eq!(stars[4].expanded, V2::new(1, 6));
    assert_eq!(stars[5].expanded, V2::new(12, 7));
    assert_eq!(stars[6].expanded, V2::new(9, 10));
    assert_eq!(stars[7].expanded, V2::new(0, 11));
    assert_eq!(stars[8].expanded, V2::new(5, 11));

    let v = sum_lengths(&stars);
    assert_eq!(v, 374);

    let v = calculate_distance("resources/day_11/day_11_sample.txt", 2);
    assert_eq!(v, 374);
}

#[test]
fn sample_part_two(){ 
    let v = calculate_distance("resources/day_11/day_11_sample.txt", 100);
    assert_eq!(v, 8410);
}
