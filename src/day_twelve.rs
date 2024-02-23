#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use std::collections::HashMap;

pub fn part_one() {
    let v = sum_permutations("resources/inputs/day_12.txt", 1);
    println!("{v}");
}

pub fn part_two() {
    let v = sum_permutations("resources/inputs/day_12.txt", 5);
    println!("{v}");
}

fn sum_permutations(file_path: &str, copies_count: i64) -> i64 {
    let content = std::fs::read_to_string(file_path).unwrap();

    let mut cache: HashMap<State, i64> = HashMap::new();

    let mut count = 0;
    let lines: Vec<&str> = content.split('\n').collect();
    for l in lines {
        let line = l.trim();
        if line.len() > 1 {
            let perms = count_permutations(State::from_string(line, copies_count), &mut cache);
            count += perms;
        }
    }

    return count;
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Spring {
    // .
    Working,
    // #
    Broken,
    // ?
    Unknown,
}

impl Spring {
    fn from_char(input: char) -> Spring {
        match input {
            '#' => return Spring::Broken,
            '.' => return Spring::Working,
            '?' => return Spring::Unknown,
            _ => panic!("Invalid spring type of {input}"),
        }
    }

    fn can_be_working(input: Spring) -> bool {
        return input == Spring::Working || input == Spring::Unknown;
    }

    fn can_be_broken(input: Spring) -> bool {
        return input == Spring::Broken || input == Spring::Unknown;
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    springs: Vec<Spring>,
    groups: Vec<i64>,
}

impl State {
    // number of copies to return. so copies_count of 1 means just return the file.
    fn from_string(input: &str, copies_count: i64) -> State {
        let mut ret = State {
            springs: vec![],
            groups: vec![],
        };

        let springs_nums: Vec<&str> = input.split(' ').collect();
        if springs_nums.len() != 2 {
            panic!("Invalid spring input format");
        }

        // parse springs
        let springs: Vec<char> = springs_nums[0].trim().chars().collect();
        for c in springs {
            ret.springs.push(Spring::from_char(c));
        }

        //parse groups
        let groups: Vec<&str> = springs_nums[1].trim().split(',').collect();
        for g in groups {
            // Could do better error checking here.
            ret.groups.push(g.parse().unwrap());
        }

        // "unfold" make copies
        let mut springs_orig = ret.springs.clone();
        let mut groups_orig = ret.groups.clone();
        for c in 0..(copies_count - 1) {
            ret.springs.push(Spring::Unknown);
            ret.springs.append(&mut springs_orig.clone());
            ret.groups.append(&mut groups_orig.clone());
        }

        return ret;
    }

    // can we fit a spring
    // spring_start is the start of the spring. not the required preceeding working
    // this also does verify that working if not on the edge
    fn fit_spring(&self, spring_start: usize, spring_len: usize) -> bool {
        if spring_len + spring_start > self.springs.len() {
            return false;
        }

        // middle needs to be all broken
        for i in 0..spring_len {
            if !Spring::can_be_broken(self.springs[spring_start + i]) {
                return false;
            }
        }

        // if beginning not on edge the must have working before spring_start
        if spring_start > 0 {
            if !Spring::can_be_working(self.springs[spring_start - 1]) {
                return false;
            }
        }

        // if end not on edge then must have working after spring_start + spring_len
        let end_i = spring_start + spring_len;
        if end_i < self.springs.len() {
            if !Spring::can_be_working(self.springs[end_i]) {
                return false;
            }
        }

        return true;
    }
}

fn count_permutations(input: State, cache: &mut HashMap<State, i64>) -> i64 {
    match cache.get(&input) {
        Some(v) => {
            return *v;
        }
        None => {}
    }

    let mut count = 0;

    // got through all the numbers
    if input.groups.len() == 0 {
        if input.springs.contains(&Spring::Broken) {
            return 0;
        } else {
            return 1;
        }
    }

    // No more room for springs. So not a valid configuration.
    if input.springs.len() == 0 {
        return 0;
    }

    // get the first group in the list
    let group_size = usize::try_from(*input.groups.get(0).unwrap()).unwrap();

    for i in 0..input.springs.len() {
        if input.fit_spring(i, group_size) {
            // if we have moved past a working spring that is separated from this spring then we
            // have created another group, which makes this placement invalid
            let start = i as i64 - 1;
            if start > 0 {
                for r in 0..start {
                    let ur = usize::try_from(r).unwrap();
                    if input.springs[ur] == Spring::Broken {
                        cache.insert(input, count);
                        return count;
                    }
                }
            }

            let split_point = i + group_size + 1;

            // that was the last group to place
            if input.groups.len() == 1 {
                let mut found = false;
                for s in split_point..input.springs.len() {
                    if input.springs[s] == Spring::Broken {
                        found = true;
                        break;
                    }
                }
                if !found {
                    count += 1;
                }
                continue;
            }

            // If we would need to split longer than the available springs then not a valid
            // configuration because no more room for springs.
            if split_point >= input.springs.len() {
                cache.insert(input, count);
                return count;
            }

            // now check the sub spring without the current number
            let mut sub_state = input.clone();
            sub_state.groups.remove(0);
            sub_state.springs = sub_state.springs.split_off(split_point);

            // continue onto the next group
            count += count_permutations(sub_state, cache);
        }
    }

    cache.insert(input, count);
    return count;
}

#[test]
fn state_parse() {
    let state = State::from_string("##..?? 1,2,30", 1);
    assert_eq!(state.groups.len(), 3);
    assert_eq!(state.groups[0], 1);
    assert_eq!(state.groups[1], 2);
    assert_eq!(state.groups[2], 30);

    assert_eq!(state.springs.len(), 6);
    assert_eq!(state.springs[0], Spring::Broken);
    assert_eq!(state.springs[1], Spring::Broken);
    assert_eq!(state.springs[2], Spring::Working);
    assert_eq!(state.springs[3], Spring::Working);
    assert_eq!(state.springs[4], Spring::Unknown);
    assert_eq!(state.springs[5], Spring::Unknown);
}

#[test]
fn can_be_working_can_be_broken() {
    assert_eq!(Spring::can_be_working(Spring::Broken), false);
    assert_eq!(Spring::can_be_working(Spring::Unknown), true);
    assert_eq!(Spring::can_be_working(Spring::Working), true);

    assert_eq!(Spring::can_be_broken(Spring::Broken), true);
    assert_eq!(Spring::can_be_broken(Spring::Unknown), true);
    assert_eq!(Spring::can_be_broken(Spring::Working), false);
}

#[test]
fn fit_spring() {
    let state = State::from_string("??? 1", 1);

    // length of 1 should have three options
    assert_eq!(state.fit_spring(0, 1), true);
    assert_eq!(state.fit_spring(1, 1), true);
    assert_eq!(state.fit_spring(2, 1), true);

    // length of 2 should have two otions
    assert_eq!(state.fit_spring(0, 2), true);
    assert_eq!(state.fit_spring(1, 2), true);
    assert_eq!(state.fit_spring(2, 2), false);

    // legnth of 3 should have one option
    assert_eq!(state.fit_spring(0, 3), true);
    assert_eq!(state.fit_spring(1, 3), false);
    assert_eq!(state.fit_spring(2, 3), false);

    // any higher lengths have no options
    assert_eq!(state.fit_spring(0, 4), false);
    assert_eq!(state.fit_spring(1, 4), false);
    assert_eq!(state.fit_spring(0, 5), false);

    let state = State::from_string("?#? 1", 1);

    // length of 1 has only 1 options
    assert_eq!(state.fit_spring(0, 1), false);
    assert_eq!(state.fit_spring(1, 1), true);
    assert_eq!(state.fit_spring(2, 1), false);

    // length of two only has two options
    assert_eq!(state.fit_spring(0, 2), true);
    assert_eq!(state.fit_spring(1, 2), true);
    assert_eq!(state.fit_spring(2, 2), false);

    // length of three has only one option
    assert_eq!(state.fit_spring(0, 3), true);
    assert_eq!(state.fit_spring(1, 3), false);

    let state = State::from_string("?#?#? 1", 1);

    // length of one
    assert_eq!(state.fit_spring(0, 1), false);
    assert_eq!(state.fit_spring(1, 1), true);
    assert_eq!(state.fit_spring(2, 1), false);
    assert_eq!(state.fit_spring(3, 1), true);
    assert_eq!(state.fit_spring(4, 1), false);

    // length of two
    assert_eq!(state.fit_spring(0, 2), true);
    assert_eq!(state.fit_spring(1, 2), false);
    assert_eq!(state.fit_spring(2, 2), false);
    assert_eq!(state.fit_spring(3, 2), true);
}

#[test]
fn permutations() {
    let mut cache: HashMap<State, i64> = HashMap::new();

    assert_eq!(
        count_permutations(State::from_string("??? 1", 1), &mut cache),
        3
    );
    assert_eq!(
        count_permutations(State::from_string("??? 2", 1), &mut cache),
        2
    );
    assert_eq!(
        count_permutations(State::from_string("??? 3", 1), &mut cache),
        1
    );
    assert_eq!(
        count_permutations(State::from_string("??? 4", 1), &mut cache),
        0
    );

    assert_eq!(
        count_permutations(State::from_string("?#?? 1", 1), &mut cache),
        1
    );

    assert_eq!(
        count_permutations(
            State::from_string("#....######..#####. 1,6,5", 1),
            &mut cache
        ),
        1
    );
    assert_eq!(
        count_permutations(State::from_string("#.#?. 1,1", 1), &mut cache),
        1
    );
    assert_eq!(
        count_permutations(State::from_string("??? 1,1", 1), &mut cache),
        1
    );
    assert_eq!(
        count_permutations(State::from_string("??? 1,1,1", 1), &mut cache),
        0
    );

    assert_eq!(
        count_permutations(State::from_string("???.### 1,1,3", 1), &mut cache),
        1
    );

    assert_eq!(
        count_permutations(State::from_string(".??..??...?##. 1,1,3", 1), &mut cache),
        4
    );
    assert_eq!(
        count_permutations(State::from_string("?#?#?#?#?#?#?#? 1,3,1,6", 1), &mut cache),
        1
    );
    assert_eq!(
        count_permutations(State::from_string("????.#...#... 4,1,1", 1), &mut cache),
        1
    );
    assert_eq!(
        count_permutations(
            State::from_string("????.######..#####. 1,6,5", 1),
            &mut cache
        ),
        4
    );
    assert_eq!(
        count_permutations(State::from_string("?###???????? 3,2,1", 1), &mut cache),
        10
    );
    assert_eq!(
        count_permutations(
            State::from_string("?#.??????#??#?#?#?#? 1,1,15", 1),
            &mut cache
        ),
        1
    );

    assert_eq!(
        count_permutations(State::from_string(".##.?#??.#.?# 2,1,1,1", 1), &mut cache),
        1
    );
}

#[test]
fn sample() {
    assert_eq!(sum_permutations("resources/day_12/sample.txt", 1), 31);
}

#[test]
fn unfolding() {
    let mut cache: HashMap<State, i64> = HashMap::new();

    let state = State::from_string("???.### 1,1,3", 5);
    assert_eq!(count_permutations(state, &mut cache), 1);

    let state = State::from_string("????.#...#... 4,1,1", 5);
    assert_eq!(count_permutations(state, &mut cache), 16);

    let state = State::from_string(".??..??...?##. 1,1,3", 5);
    assert_eq!(count_permutations(state, &mut cache), 16384);
}
