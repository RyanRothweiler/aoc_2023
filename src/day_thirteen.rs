/* Improvements
 * - Some bad type casting to and from usize.
 * - Code duplicated for row / col. Could definitely generaize this to work on any number of dimensions.
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

pub fn run() {
    let content = std::fs::read_to_string("resources/inputs/day_13.txt").unwrap();
    let v = calc(&content);
    println!("{v}");
}

fn calc(file_data: &str) -> i64 {
    let mut maps: Vec<String> = vec![];
    let mut accum: String = String::new();
    for l in file_data.lines() {
        if l.trim().len() == 0 {
            maps.push(accum.clone());
            accum.clear();
            continue;
        }

        accum.push_str(&l.trim());
        accum.push('\n');
    }

    maps.push(accum.clone());

    let mut row_count = 0;
    let mut col_count = 0;
    for m in maps {
        let c = check_cols(&m);
        let r = check_rows(&m);

        if c == None && r == None {
            println!("{m}");
            panic!("Didn't find a row or a col!");
        }

        match c {
            Some(v) => col_count += v,
            None => {}
        }

        match r {
            Some(v) => row_count += v,
            None => {}
        }
    }

    return (100 * row_count) + col_count;
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

fn build_map(map_data: &str) -> TwoD<char> {
    let lines: Vec<&str> = map_data.lines().collect();
    let first_chars: Vec<char> = lines[0].trim().chars().collect();

    let mut map: TwoD<char> = TwoD::new(first_chars.len(), lines.len(), '.');
    for y in 0..map.height() {
        let line: &str = lines[y].trim();
        let chars: Vec<char> = line.trim().chars().collect();

        for x in 0..map.width() {
            map.set(x, y, chars[x]);
        }
    }

    return map;
}

fn check_cols(map_data: &str) -> Option<i64> {
    let mut map = build_map(map_data);

    let mut col_checking: i64 = 1;
    'top: loop {
        if col_checking == map.width() as i64 {
            break;
        }

        let right: char = map.get(col_checking as usize, 0).unwrap().clone();
        let left: char = map.get(col_checking as usize - 1, 0).unwrap().clone();

        // requireds one incorrect 'smudge"
        let mut smudge_count = 0;

        let mut col_right = col_checking;
        let mut col_left = col_checking - 1;

        // verify col_right and col_left are equal
        'verify: loop {
            for y in 0..map.height() {
                let right: char = map.get(col_right as usize, y).unwrap().clone();
                let left: char = map.get(col_left as usize, y).unwrap().clone();

                // not symetrical. advance to the next col
                if right != left {
                    smudge_count += 1;

                    // two smudges is too many
                    if smudge_count == 2 {
                        col_checking += 1;
                        continue 'top;
                    }
                }
            }

            col_right += 1;
            col_left -= 1;

            // if we're off the map, then we're done and they're symetrical
            if col_right == map.width() as i64 || col_left < 0 {
                break 'verify;
            }
        }

        // they are symetrical
        // require one smudge
        if smudge_count == 1 {
            return Some(col_checking);
        } else {
            col_checking += 1;
        }
    }

    return None;
}

fn check_rows(map_data: &str) -> Option<i64> {
    let mut map = build_map(map_data);

    let mut row_checking: i64 = 1;
    'top: loop {
        if row_checking == map.height() as i64 {
            break;
        }

        let right: char = map.get(0, row_checking as usize).unwrap().clone();
        let left: char = map.get(0, row_checking as usize - 1).unwrap().clone();

        // requireds one incorrect 'smudge"
        let mut smudge_count = 0;

        let mut row_right = row_checking;
        let mut row_left = row_checking - 1;

        // verify col_right and col_left are equal
        'verify: loop {
            for x in 0..map.width() {
                let right: char = map.get(x, row_right as usize).unwrap().clone();
                let left: char = map.get(x, row_left as usize).unwrap().clone();

                // not symetrical. advance to the next col
                if right != left {
                    smudge_count += 1;

                    // two smudges is too many
                    if smudge_count == 2 {
                        row_checking += 1;
                        continue 'top;
                    }
                }
            }

            row_right += 1;
            row_left -= 1;

            // if we're off the map, then we're done and they're symetrical
            if row_right == map.height() as i64 || row_left < 0 {
                break 'verify;
            }
        }

        // they are symetrical
        // require one smudge
        if smudge_count == 1 {
            return Some(row_checking);
        } else {
            row_checking += 1;
        }
    }

    return None;
}

#[test]
fn colums_ptwo() {
    let contents_one = std::fs::read_to_string("resources/day_13/sample_one.txt").unwrap();
    let contents_two = std::fs::read_to_string("resources/day_13/sample_two.txt").unwrap();

    assert_eq!(check_cols(&contents_one), None);
    assert_eq!(check_cols(&contents_two), None);
}

#[test]
fn rows_ptwo() {
    let contents_one = std::fs::read_to_string("resources/day_13/sample_one.txt").unwrap();
    let contents_two = std::fs::read_to_string("resources/day_13/sample_two.txt").unwrap();

    assert_eq!(check_rows(&contents_one), Some(3));
    assert_eq!(check_rows(&contents_two), Some(1));
}

#[test]
fn start_smudge() {
    let contents_two = std::fs::read_to_string("resources/day_13/sample_start_smudge.txt").unwrap();
    assert_eq!(check_cols(&contents_two), Some(16));
}

#[test]
fn combined_ptwo() {
    let contents = std::fs::read_to_string("resources/day_13/sample_both.txt").unwrap();
    assert_eq!(calc(&contents), 400);
}

// these test part one
/*
#[test]
fn colums() {
    let contents_one = std::fs::read_to_string("resources/day_13/sample_one.txt").unwrap();
    let contents_two = std::fs::read_to_string("resources/day_13/sample_two.txt").unwrap();

    assert_eq!(check_cols(&contents_one), Some(5));
    assert_eq!(check_cols(&contents_two), None);
}

#[test]
fn rows() {
    let contents_one = std::fs::read_to_string("resources/day_13/sample_one.txt").unwrap();
    let contents_two = std::fs::read_to_string("resources/day_13/sample_two.txt").unwrap();

    assert_eq!(check_rows(&contents_one), None);
    assert_eq!(check_rows(&contents_two), Some(4));
}

#[test]
fn combined() {
    let contents = std::fs::read_to_string("resources/day_13/sample_both.txt").unwrap();
    assert_eq!(calc(&contents), 405);
}
*/
