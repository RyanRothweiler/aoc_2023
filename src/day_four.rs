#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

pub fn run() {
    let contents: String = std::fs::read_to_string("resources/inputs/day_4.txt").unwrap();

    let sum: u64 = cards_count(&contents).expect("Error parsing input.");

    println!("{sum}");
}

// part 1
fn sum_points(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    for l in input.split('\n') {
        // handle empty lines
        if l.len() > 1 {
            match calc_point_values(l.trim()) {
                Some(v) => {
                    sum = sum + v;
                }
                None => {
                    eprintln!("Error parsing cards.");
                    return None;
                }
            }
        }
    }

    Some(sum)
}

// part 2
fn cards_count(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    let lines: Vec<&str> = input.split('\n').collect();

    // how many copies of each card we have.
    // We always start with one copy so init to 1.
    let mut cards_count: Vec<u64> = vec![1; lines.len()];

    //for l in lines {
    for i in 0..lines.len() {
        let line = lines[i];

        let current_card_count = match cards_count.get(i) {
            Some(v) => *v,
            None => {
                eprintln!("Invalid card format.");
                return None;
            }
        };

        // handle empty lines
        if line.len() > 1 {
            match matching_count(line.trim()) {
                Some(winning_count) => {
                    // iterate down the next winning cards, adding one to the winning count
                    for c in 0..winning_count {
                        let cu: usize = c as usize + i + 1;

                        match &cards_count.get(cu) {
                            Some(current_accum) => {
                                cards_count[cu] = *current_accum + current_card_count;
                            }
                            None => {
                                eprintln!("Card format is invalid.");
                                return None;
                            }
                        }
                    }
                }
                None => {
                    eprintln!("Error parsing cards.");
                    return None;
                }
            }
        }
    }

    for s in cards_count {
        sum = sum + s;
    }

    Some(sum)
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

/* Input format  is
 * Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
 *
 * Returns the count of matching numbers.
 */
fn matching_count(input: &str) -> Option<u64> {
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
    let mut count: u64 = 0;
    for w in winning_numbers {
        if have_numbers.contains(&w) {
            count = count + 1;
        }
    }

    Some(count)
}

// Returns the cards "points"
fn calc_point_values(input: &str) -> Option<u64> {
    match matching_count(input) {
        Some(count) => {
            let mut res: u64 = 0;
            if count == 0 {
                return Some(0);
            } else if count == 1 {
                return Some(1);
            } else {
                return Some(u64::pow(2, (count as u32) - 1));
            }
        }
        None => return None,
    };
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

#[test]
fn part_two_sample() {
    let contents: String = std::fs::read_to_string("resources/day_4/day_4_sample.txt").unwrap();
    assert_eq!(cards_count(&contents), Some(30));
}

#[test]
fn part_two_sample_two() {
    let contents: String = std::fs::read_to_string("resources/day_4/day_4_sample_two.txt").unwrap();
    assert_eq!(cards_count(&contents), Some(15));
}
