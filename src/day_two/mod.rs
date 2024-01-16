#![allow(unused_variables, dead_code, unused_mut)]

use std::fs;
use std::str::FromStr;

pub fn run() {
    let contents: String =
        fs::read_to_string("resources/day_2_input.txt").expect("Could not find the file.");

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut sum: u64 = 0;
    for line in lines {
        match check_game(line) {
            Some(t) => {
                sum = sum + t;
            }
            None => { // Not a valid game, or an error, do nothing.
            }
        }
    }
    println!("Final sum {sum}");
}

enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn within_limits(col: Color, count: u32) -> bool {
        match col {
            Color::Red => return count <= 12,
            Color::Green => return count <= 13,
            Color::Blue => return count <= 14,
        }
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "red" => return Ok(Color::Red),
            "green" => return Ok(Color::Green),
            "blue" => return Ok(Color::Blue),
            _ => return Err("Unknown input".to_string()),
        }
    }
}

// returns the game id if valid
fn check_game(game: &str) -> Option<u64> {
    let info_pulls: Vec<&str> = game.split(':').collect();
    if info_pulls.len() != 2 {
        eprintln!("Invalid game format.");
        return None;
    }

    // get the ID
    let game_id: Vec<&str> = info_pulls[0].split(' ').collect();
    if game_id.len() != 2 {
        eprintln!("Invalid game format.");
        return None;
    }
    let id: u64 = match game_id[1].parse() {
        Ok(t) => t,
        Err(e) => {
            let st = game_id[1];
            eprintln!("Error parsing id: \"{st}\" {e}");
            return None;
        }
    };

    // get each pull
    let pulls: Vec<&str> = info_pulls[1].split(';').collect();
    for pull in pulls {
        // get list of number color
        let entries: Vec<&str> = pull.split(',').collect();

        let entry_num: usize = 0;
        for entry in entries {
            let data: Vec<&str> = entry.trim().split(' ').collect();
            if data.len() != 2 {
                eprintln!("Invalid game format.");
                return None;
            }

            let count: u32 = match data[0].trim().parse() {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("{e}");
                    return None;
                }
            };

            let col = match Color::from_str(data[1].trim()) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("{e}");
                    return None;
                }
            };

            if !Color::within_limits(col, count) {
                return None;
            }
        }
    }

    // return none before here, thus assuming valid
    return Some(id);
}

#[test]
fn one_valid() {
    assert_eq!(check_game("Game 1: 4 red, 5 blue, 4 green"), Some(1));
}

#[test]
fn one_invalid() {
    assert_eq!(check_game("Game 1: 4 red, 5 blue, 4 green; 7 red, 8 blue, 2 green; 9 blue, 6 red; 1 green, 3 red, 7 blue; 3 green, 70 red"), None);
}

#[test]
fn mult_invalid() {
    assert_eq!(check_game("Game 1: 4 red, 5 blue, 4 green; 7 red, 8 blue, 2 green; 9 blue, 6 red; 1 green, 3 red, 7 blue; 3 green, 70 red"), None);
}

#[test]
fn mult_valid() {
    assert_eq!(check_game("Game 1234: 4 red, 5 blue, 4 green; 7 red, 8 blue, 2 green; 9 blue, 6 red; 1 green, 3 red, 7 blue; 3 green,0 red"), Some(1234));
}
