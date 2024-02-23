#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use crate::perma::line::Line;
use crate::perma::math::*;
use crate::perma::v2::V2;

pub fn part_one() {
    let v = run("resources/inputs/day_18.txt", false);
    println!("{v}");
}

pub fn part_two() {
    let v = run("resources/inputs/day_18.txt", true);
    println!("{v}");
}

fn run(file_dir: &str, flip: bool) -> f64 {
    let map = build_points(file_dir, flip);
    let p = map.perimiter as f64 * 0.5;
    let a = shoelace_area(&map.points) as f64 + p + 1.0;
    return a;
}

struct Map {
    points: Vec<V2>,
    perimiter: i64,
}

fn build_points(file_dir: &str, flip: bool) -> Map {
    let mut ret = Map {
        points: vec![],
        perimiter: 0,
    };

    let contents = std::fs::read_to_string(file_dir).unwrap();
    let mut lines = contents.lines();

    let mut bot_pos = V2::new(0, 0);
    ret.points.push(bot_pos);

    for step_string in lines {
        // part_one
        let mut miner = MineStep::from_string(step_string).unwrap();
        if flip {
            miner = MineStep::from_string_flipped(step_string).unwrap();
        }

        bot_pos = miner.step(bot_pos);

        ret.points.push(bot_pos);
        ret.perimiter += miner.len;
    }

    return ret;
}

#[derive(Eq, PartialEq, Debug)]
struct MineStep {
    dir: V2,
    len: i64,
    color_hex: String,
}

impl MineStep {
    fn new(dir: V2, len: i64) {}

    // For part one
    // Input format is "U 3 (#a77fa3)"
    fn from_string(input: &str) -> Result<MineStep, &str> {
        let parts: Vec<&str> = input.split(' ').collect();
        if parts.len() != 3 {
            return Err("Invalid Format");
        }

        let mut ms = MineStep {
            dir: V2::new(0, 0),
            len: 0,
            color_hex: "".to_string(),
        };

        // parse dir
        let dir_chars: Vec<char> = parts[0].chars().collect();
        if dir_chars.len() < 1 {
            return Err("Invalid Format");
        }
        match dir_chars[0] {
            'R' => ms.dir = V2::new(1, 0),
            'L' => ms.dir = V2::new(-1, 0),
            'D' => ms.dir = V2::new(0, 1),
            'U' => ms.dir = V2::new(0, -1),
            _ => return Err("Error parsing direction"),
        }

        // parse length
        match parts[1].parse() {
            Ok(i) => ms.len = i,
            Err(t) => return Err("Error parsing length"),
        }

        // parse color
        let mut col = parts[2].to_string();
        //'('
        col.remove(0);
        //'#'
        col.remove(0);
        //')'
        col.remove(col.len() - 1);
        ms.color_hex = col;

        return Ok(ms);
    }

    // for part two
    fn from_string_flipped(input: &str) -> Result<MineStep, &str> {
        let parts: Vec<&str> = input.split(' ').collect();
        if parts.len() != 3 {
            return Err("Invalid Format");
        }

        let mut ms = MineStep {
            dir: V2::new(0, 0),
            len: 0,
            color_hex: "".to_string(),
        };

        // The full instruction is encoded in the color.
        let mut col = parts[2].to_string();

        // remove the beginning and end bits
        //'('
        col.remove(0);
        //'#'
        col.remove(0);
        //')'
        col.remove(col.len() - 1);

        // get length. first 5 digits in hex
        let len_string = &col.clone()[..5];
        ms.len = i64::from_str_radix(len_string, 16).unwrap();

        // direction
        let chars: Vec<char> = col.chars().collect();
        let dir_c: char = chars[chars.len() - 1];
        match dir_c {
            '0' => ms.dir = V2::new(1, 0),
            '1' => ms.dir = V2::new(0, 1),
            '2' => ms.dir = V2::new(-1, 0),
            '3' => ms.dir = V2::new(0, -1),
            _ => {
                return Err("Invalid direction character");
            }
        }

        return Ok(ms);
    }

    fn step(&self, curr: V2) -> V2 {
        return curr + (self.dir * self.len);
    }
}

#[test]
fn parse_step() {
    let mut ms = MineStep::from_string("R 1 (#70c710)").unwrap();
    assert_eq!(ms.dir, V2::new(1, 0));
    assert_eq!(ms.len, 1);
    assert_eq!(ms.color_hex, "70c710");

    let mut ms = MineStep::from_string("U 10 (#70c710)").unwrap();
    assert_eq!(ms.dir, V2::new(0, -1));
    assert_eq!(ms.len, 10);
    assert_eq!(ms.color_hex, "70c710");

    let mut ms = MineStep::from_string("D 1110 (#70c710)").unwrap();
    assert_eq!(ms.dir, V2::new(0, 1));
    assert_eq!(ms.len, 1110);
    assert_eq!(ms.color_hex, "70c710");

    let mut ms = MineStep::from_string("L 5 (#70c710)").unwrap();
    assert_eq!(ms.dir, V2::new(-1, 0));
    assert_eq!(ms.len, 5);
    assert_eq!(ms.color_hex, "70c710");
}

#[test]
fn parse_step_hex() {
    let mut ms = MineStep::from_string_flipped("R 1 (#70c710)").unwrap();
    assert_eq!(ms.len, 461937);
    assert_eq!(ms.dir, V2::new(1, 0));

    let mut ms = MineStep::from_string_flipped("D 5 (#0dc571)").unwrap();
    assert_eq!(ms.len, 56407);
    assert_eq!(ms.dir, V2::new(0, 1));
}
