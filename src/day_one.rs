#![allow(unused_variables, dead_code)]

use core::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs;

const NUM_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_one() {
    process("resources/day_1_input.txt", vec![]);
}

pub fn part_two() {
    process("resources/day_1_input.txt", NUM_WORDS.to_vec());
}

fn process(file_dir: &str, num_words: Vec<&str>) {
    let contents: String = fs::read_to_string(file_dir).unwrap();

    let lines: Vec<&str> = contents.split('\n').collect();

    let mut sum: u32 = 0;
    for l in lines {
        sum += handle_line(l, &num_words);
    }

    println!("{sum}");
}

struct Entry {
    num: u32,
    index: u32,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.index).cmp(&(other.index))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        (self.index) == (other.index)
    }
}

impl Eq for Entry {}

fn handle_line(line: &str, num_words: &Vec<&str>) -> u32 {
    let mut tree: BTreeSet<Entry> = BTreeSet::new();

    let line_lower = line.to_lowercase();

    // Find the words
    for i in 0..num_words.len() {
        match line_lower.find(num_words[i]) {
            Some(ind) => {
                tree.insert(Entry {
                    num: (i + 1) as u32,
                    index: ind as u32,
                });
            }
            None => {}
        }

        match line_lower.rfind(NUM_WORDS[i]) {
            Some(ind) => {
                tree.insert(Entry {
                    num: (i + 1) as u32,
                    index: ind as u32,
                });
            }
            None => {}
        }
    }

    // Find the character numbers
    {
        let mut i = 0;
        for c in line_lower.chars() {
            let v = c.to_digit(10);
            match v {
                Some(val) => {
                    tree.insert(Entry { num: val, index: i });
                }
                None => {}
            }

            i = i + 1;
        }
    }

    if tree.len() == 0 {
        return 0;
    }

    if tree.len() == 1 {
        return (tree.first().unwrap().num * 10) + tree.first().unwrap().num;
    } else {
        return (tree.first().unwrap().num * 10) + tree.last().unwrap().num;
    }
}

#[test]
fn numbers_ends() {
    assert_eq!(handle_line("1asfadf1", &NUM_WORDS.to_vec()), 11);
}

#[test]
fn numbers_hidden() {
    assert_eq!(handle_line("aa1sfa1d2fff3", &NUM_WORDS.to_vec()), 13);
}

#[test]
fn numbers_single() {
    assert_eq!(handle_line("2asfadf", &NUM_WORDS.to_vec()), 22);
}

#[test]
fn words_ends() {
    assert_eq!(handle_line("oneasfadtwo", &NUM_WORDS.to_vec()), 12);
}

#[test]
fn words_extra() {
    assert_eq!(handle_line("oneasonefadtwothree", &NUM_WORDS.to_vec()), 13);
}

#[test]
fn mixed_numbers_ends() {
    assert_eq!(handle_line("1asonefadtwothree9", &NUM_WORDS.to_vec()), 19);
}

#[test]
fn mixed_ends() {
    assert_eq!(handle_line("asfivefadtwothree9", &NUM_WORDS.to_vec()), 59);
}

#[test]
fn mixed_hidden() {
    assert_eq!(
        handle_line("asfivefadtwothree9theeeeee", &NUM_WORDS.to_vec()),
        59
    );
}
