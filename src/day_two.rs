#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

const RED_LIMIT: u64 = 12;
const GREEN_LIMIT: u64 = 13;
const BLUE_LIMIT: u64 = 14;

pub fn part_one() {
    sum_valid_games();
}

pub fn part_two() {
    get_games_power();
}

// Finds the minimum number of marbles required for each game, then multiplies them together.
// This is considered the marbles "power"
fn get_games_power() {
    let contents: String = fs::read_to_string("resources/inputs/day_2.txt").unwrap();

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut accum: u64 = 0;
    for line in lines {
        match get_minimum_power(line) {
            Some(t) => {
                accum = accum + t;
            }
            None => {
                // Not a valid game, or an error, do nothing.
            }
        }
    }

    println!("{accum}");
}

// sums the games which are valid according to the color within_limits
fn sum_valid_games() {
    let contents: String = fs::read_to_string("resources/inputs/day_2.txt").unwrap();

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut sum: u64 = 0;
    for line in lines {
        match game_valid(line) {
            Some(t) => {
                sum = sum + t;
            }
            None => {
                // Not a valid game, or an error, do nothing.
            }
        }
    }

    println!("{sum}");
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn within_limits(col: Color, count: u64) -> bool {
        match col {
            Color::Red => return count <= RED_LIMIT,
            Color::Green => return count <= GREEN_LIMIT,
            Color::Blue => return count <= BLUE_LIMIT,
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

struct MarbleCount {
    color: Color,
    count: u64,
}

struct Game {
    id: u64,
    pulls: Vec<MarbleCount>,
}

fn parse_game(input: &str) -> Option<Game> {
    let mut ret = Game {
        id: 0,
        pulls: vec![],
    };

    let info_pulls: Vec<&str> = input.split(':').collect();
    if info_pulls.len() != 2 {
        return None;
    }

    // get the ID
    let game_id: Vec<&str> = info_pulls[0].split(' ').collect();
    if game_id.len() != 2 {
        return None;
    }
    ret.id = match game_id[1].parse() {
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
            let mut marb_count = MarbleCount {
                color: Color::Red,
                count: 0,
            };

            let data: Vec<&str> = entry.trim().split(' ').collect();
            if data.len() != 2 {
                eprintln!("Invalid game format.");
                return None;
            }

            marb_count.count = match data[0].trim().parse() {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("{e}");
                    return None;
                }
            };

            marb_count.color = match Color::from_str(data[1].trim()) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("{e}");
                    return None;
                }
            };

            ret.pulls.push(marb_count);
        }
    }

    return Some(ret);
}

// returns the game id if valid
fn game_valid(game_string: &str) -> Option<u64> {
    let game = match parse_game(game_string) {
        Some(t) => t,
        None => return None,
    };

    for pull in game.pulls {
        if !Color::within_limits(pull.color, pull.count) {
            return None;
        }
    }

    // return none before here, thus assuming valid
    return Some(game.id);
}

// returns the minimum number of marbles needed for the game to be valid. Then multiplies them
// together and returns that.
fn get_minimum_power(game_string: &str) -> Option<u64> {
    let game = match parse_game(game_string) {
        Some(t) => t,
        None => return None,
    };

    let mut maximums: HashMap<Color, u64> = HashMap::new();

    for pull in game.pulls {
        match maximums.get(&pull.color) {
            Some(current) => {
                if pull.count > *current {
                    maximums.insert(pull.color, pull.count);
                }
            }
            None => {
                maximums.insert(pull.color, pull.count);
            }
        };
    }

    let mut accum: u64 = 1;
    for (key, value) in maximums {
        accum = accum * value;
    }

    return Some(accum);
}

#[test]
fn one_valid() {
    assert_eq!(game_valid("Game 1: 4 red, 5 blue, 4 green"), Some(1));
}

#[test]
fn one_invalid() {
    assert_eq!(game_valid("Game 1: 4 red, 5 blue, 4 green; 7 red, 8 blue, 2 green; 9 blue, 6 red; 1 green, 3 red, 7 blue; 3 green, 70 red"), None);
}

#[test]
fn mult_invalid() {
    assert_eq!(game_valid("Game 1: 4 red, 5 blue, 4 green; 7 red, 8 blue, 2 green; 9 blue, 6 red; 1 green, 3 red, 7 blue; 3 green, 70 red"), None);
}

#[test]
fn mult_valid() {
    assert_eq!(game_valid("Game 1234: 4 red, 5 blue, 4 green; 7 red, 8 blue, 2 green; 9 blue, 6 red; 1 green, 3 red, 7 blue; 3 green,0 red"), Some(1234));
}

#[test]
fn power() {
    assert_eq!(get_minimum_power("Game 1234: 4 red, 5 blue, 4 green; 7 red, 8 blue, 2 green; 9 blue, 6 red; 1 green, 3 red, 7 blue; 3 green,0 red"), Some(252));
}
