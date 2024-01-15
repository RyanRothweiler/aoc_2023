use core::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs;

#[allow(unused_variables, unused_assignments, unused_mut)]
fn main() {
    let contents: String =
        fs::read_to_string("resources/day1_input.txt").expect("Could not find the file.");

    let mut sum: u32 = 0;

    let lines: Vec<&str> = contents.split('\n').collect();

    for l in lines {
        sum += handle_line(l);
    }

    println!("{sum}");
}

const NUM_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

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

fn handle_line(line: &str) -> u32 {
    let mut tree: BTreeSet<Entry> = BTreeSet::new();

    let line_lower = line.to_lowercase();

    // Find the words
    for i in 0..NUM_WORDS.len() {
        match line_lower.find(NUM_WORDS[i]) {
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
        eprintln!("Emptry, tree. No numbers on this line?");
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
    assert_eq!(handle_line("1asfadf1"), 11);
}

#[test]
fn numbers_hidden() {
    assert_eq!(handle_line("aa1sfa1d2fff3"), 13);
}

#[test]
fn numbers_single() {
    assert_eq!(handle_line("2asfadf"), 22);
}

#[test]
fn words_ends() {
    assert_eq!(handle_line("oneasfadtwo"), 12);
}

#[test]
fn words_extra() {
    assert_eq!(handle_line("oneasonefadtwothree"), 13);
}

#[test]
fn mixed_numbers_ends() {
    assert_eq!(handle_line("1asonefadtwothree9"), 19);
}

#[test]
fn mixed_ends() {
    assert_eq!(handle_line("asfivefadtwothree9"), 59);
}

#[test]
fn mixed_hidden() {
    assert_eq!(handle_line("asfivefadtwothree9theeeeee"), 59);
}
