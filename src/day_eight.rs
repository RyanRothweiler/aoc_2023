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

        //println!("{line}");

        let eq: Vec<&str> = line.split('=').collect();

        let key: String = eq[0].trim().to_string();
        let steps: Vec<&str> = eq[1].split(',').collect();

        //let left = steps[0].trim().chars().next().unwrap().next_back().unwrap().as_str();
        let mut chars = steps[0].trim().chars();
        chars.next();
        let left = chars.as_str();

        let mut chars = steps[1].trim().chars();
        chars.next_back();
        let right = chars.as_str();

        //println!("{key} {left} {right}");

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

    loop {
        let mut instruction_chars = instructions.trim().chars();

        if node_current.key == NODE_END {
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
                        return Err("Unknown step".to_string());
                    }

                    count += 1;
                    //println!("step {c} moving to {next_id}");
                    node_current = map.get(next_id).expect("No node for key {next_id}");
                }
                None => break,
            }
        }
    }

    Ok(count)
}

#[test]
fn sample(){ 
    assert_eq!(steps_count("resources/day_8/day_8_sample.txt").unwrap(), 2);
}

#[test]
fn sample_cycle(){ 
    assert_eq!(steps_count("resources/day_8/day_8_sample_cycle.txt").unwrap(), 6);
}

#[test]
fn part_one_answer(){ 
    assert_eq!(steps_count("resources/day_8/day_8_input.txt").unwrap(), 19199);
}
