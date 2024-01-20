#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

use std::fs;

pub fn run() {
    let contents: String =
        fs::read_to_string("resources/day_4/day_4_input.txt").expect("Unable to read file.");

    let mut sum: u64 = 0;
    for l in contents.split('\n') {
        // handle empty lines
        if l.len() > 1 {
            match calc_point_values(l.trim()) {
                Some(v) => {
                    sum = sum + v;
                }
                None => {
                    eprintln!("Error parsing cards.");
                }
            }
        }
    }

    println!("{sum}");
}

fn string_to_list(input: Vec<&str>) -> Option<Vec<u64>> {
    let mut nums: Vec<u64> = vec![];
    for w in input {
        if w.trim().len() != 0 {
            let val: u64 = match w.parse() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error parsing winning number {w}");
                    return None;
                }
            };

            nums.push(val);
        }
    }

    return Some(nums);
}

/* format  is
 * Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
 */
fn calc_point_values(input: &str) -> Option<u64> {
    let card_data: Vec<&str> = input.split(':').collect();

    // numbers that we have and winning numbers
    let have_winning: Vec<&str> = match card_data.get(1) {
        Some(t) => t.split('|').collect(),
        None => {
            eprintln!("Invalid card format. Error on |");
            return None;
        }
    };

    // get list of winning numbers
    let winning_string: Vec<&str> = match have_winning.get(0) {
        Some(t) => t.split(' ').collect(),
        None => {
            eprintln!("Invalid card format. Error on :");
            return None;
        }
    };

    let mut winning_numbers: Vec<u64> = match string_to_list(winning_string) {
        Some(t) => t,
        None => {
            return None;
        }
    };

    // get list of have numbers
    let have_string: Vec<&str> = match have_winning.get(1) {
        Some(t) => t.split(' ').collect(),
        None => {
            eprintln!("Invalid card format. Error on :");
            return None;
        }
    };

    let mut have_numbers: Vec<u64> = match string_to_list(have_string) {
        Some(t) => t,
        None => {
            return None;
        }
    };

    // check the numbers
    let mut res: u64 = 0;
    for w in winning_numbers {
        if have_numbers.contains(&w) {
            if res == 0 {
                res = 1;
            } else {
                res = res * 2;
            }
        }
    }

    Some(res)
}

#[test]
fn card_one() {
    assert_eq!(
        calc_point_values("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
        Some(8)
    );
}

#[test]
fn card_two() {
    assert_eq!(
        calc_point_values("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
        Some(2)
    );
}

#[test]
fn card_five() {
    assert_eq!(
        calc_point_values("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
        Some(0)
    );
}
