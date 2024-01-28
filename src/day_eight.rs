#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use std::collections::HashMap;

pub fn run() {
    let c = steps_count("resources/day_8/day_8_input.txt").unwrap();
    println!("{c}");
}

struct Node {
    left: String,
    right: String,
    key: String,
}

const NODE_START: &str = "AAA";
const NODE_END: &str = "ZZZ";

fn path_length(starting_id: &str, instructions: &str, map: &HashMap<String, Node>) -> i32 {
    let mut node_current: &Node = map.get(starting_id).unwrap();
    let mut count: i32 = 0;

    loop {
        let mut instruction_chars = instructions.trim().chars();

        if node_current.key.ends_with('Z') {
            break;
        }

        loop {
            match instruction_chars.next() {
                Some(c) => {
                    let mut next_id: &str = "";
                    if c == 'L' {
                        next_id = &node_current.left;
                    } else if c == 'R' {
                        next_id = &node_current.right;
                    } else {
                        return 0;
                        //return Err("Unknown step".to_string());
                    }

                    count += 1;
                    node_current = map.get(next_id).expect("No node for key {next_id}");
                }
                None => break,
            }
        }
    }

    return count;
}

fn steps_count(file: &str) -> Result<i32, String> {
    let contents = std::fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = contents.split('\n').collect();
    let instructions = lines[0];

    let mut map: HashMap<String, Node> = HashMap::new();

    for l in 2..lines.len() {
        let line = lines[l];
        if line.len() == 0 {
            continue;
        }

        let eq: Vec<&str> = line.split('=').collect();

        let key: String = eq[0].trim().to_string();
        let steps: Vec<&str> = eq[1].split(',').collect();

        let mut chars = steps[0].trim().chars();
        chars.next();
        let left = chars.as_str();

        let mut chars = steps[1].trim().chars();
        chars.next_back();
        let right = chars.as_str();

        let n = Node {
            key: key.clone(),
            left: left.to_string(),
            right: right.to_string(),
        };
        map.entry(key).or_insert(n);
    }

    // walk map
    let mut node_current: &Node = map.get(NODE_START).expect("Missing starting node AAA");
    let mut count: i32 = 0;

    // find all starting nodes
    let mut currents: Vec<&Node> = vec![];
    for (key, value) in &map {
        if key.ends_with('A') {
            //currents.push(value);
            let v = path_length(key, instructions, &map);

            // calc lcm of the paths
            println!("{v}");
        }
    }

    Ok(0)
}

#[test]
fn sample() {
    assert_eq!(steps_count("resources/day_8/day_8_sample.txt").unwrap(), 2);
}

#[test]
fn sample_cycle() {
    assert_eq!(
        steps_count("resources/day_8/day_8_sample_cycle.txt").unwrap(),
        6
    );
}

/*
#[test]
fn part_one_answer() {
    assert_eq!(
        steps_count("resources/day_8/day_8_input.txt").unwrap(),
        19199
    );
}
*/
