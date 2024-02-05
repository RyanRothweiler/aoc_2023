#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

pub fn run() {
    //permutations(5);
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
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

struct State {
    springs: Vec<Spring>,
    groups: Vec<i64>,
}

impl State {
    fn from_string(input: &str) -> State {
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

        /*
        let mut curr_index = 0;

        // if not on edge then must start with .
        if spring_start > 0 {
            // start must be working
            if !Spring::can_be_working(self.springs[spring_start]) {
                return false;
            }

            curr_index += 1;
        }

        // middle needs to be all broken
        for i in 0..spring_len {
            if !Spring::can_be_broken(self.springs[curr_index]) {
                return false;
            }

            curr_index += 1;
        }

        // if we're on the edge then we're good
        if curr_index == self.springs.len() {
            return true;
        }

        if !Spring::can_be_working(self.springs[curr_index]) {
            return false;
        }
        */

        return true;
    }
}

/*
// returns the number of ways you can fit the first group
fn fit_first_group_count(input_state: State) -> i64 {
    // no groups or no springs!
    if input_state.groups.len() == 0 || input_state.springs.len() == 0 {
        return 0;
    }

    let group_size = input_state.groups[0];

    // no space for a group of that size
    if state.springs.len() < group_size {
        return 0;
    }

    let i = 0;
    let space_count = 0;
    loop {
        // ran out of room
        if i > state.springs.len() {
            break;
        }

        // found space!
        if space_count == group_size {

        }
    }

    return 0;
}
*/

// returns true if damaged, false if operational
// 1 == # (damaged) 0 == . (operational)
fn get_broken(num: i32, index: i32) -> bool {
    let i: i32 = (num >> index) & 1;
    return i == 1;
}

/*
fn valid(num: i64, bit_len: i64, spec: Vec<i64>) -> bool {

    let curr: i64 = 0;

    // move to first group
    loop {
        if curr > bit_len {
            // no groups, automatically not valid
            return false;
        }

        if curr
    }

    let group_size: i64 = 0;
    loop {
        if curr > bit_len  {
            break;
        }

        curr += 1;
        group_size += 1;
    }


    return true;
}

fn permutations(bits: u32) {
    let count = 2_i64.pow(bits);
    for num in 0..count {
        println!("{:b}", num);
    }
}
*/

/*
#[test]
fn is_broken() {
    assert_eq!(get_broken(0, 0), false);
    assert_eq!(get_broken(1, 0), true);

    // 0x10
    assert_eq!(get_broken(2, 0), false);
    assert_eq!(get_broken(2, 1), true);
    assert_eq!(get_broken(2, 3), false);
    assert_eq!(get_broken(2, 4), false);

    // 0x11
    assert_eq!(get_broken(3, 0), true);
    assert_eq!(get_broken(3, 1), true);
    assert_eq!(get_broken(3, 2), false);
}
*/

#[test]
fn state_parse() {
    let state = State::from_string("##..?? 1,2,3");
    assert_eq!(state.groups.len(), 3);
    assert_eq!(state.groups[0], 1);
    assert_eq!(state.groups[1], 2);
    assert_eq!(state.groups[2], 3);

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
    let state = State::from_string("??? 1");

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

    let state = State::from_string("?#? 1");

    // length of 1 has only 1 options
    assert_eq!(state.fit_spring(0, 1), false);
    assert_eq!(state.fit_spring(1, 1), true);
    assert_eq!(state.fit_spring(2, 1), false);

    // length of two only has two options
    assert_eq!(state.fit_spring(0, 2), true);
    assert_eq!(state.fit_spring(1, 2), true);
    assert_eq!(state.fit_spring(2, 2), false);

    /*
    assert_eq!(state.fit_spring(0, 2), true);
    assert_eq!(state.fit_spring(0, 3), true);
    assert_eq!(state.fit_spring(0, 4), false);

    assert_eq!(state.fit_spring(1, 1), false);
    */

    /*
    //assert_eq!(state.fit_spring(0, 2), true);

    assert_eq!(state.fit_spring(0, 3), true);
    assert_eq!(state.fit_spring(0, 4), false);
    assert_eq!(state.fit_spring(0, 5), false);

    assert_eq!(state.fit_spring(1, 1), true);
    assert_eq!(state.fit_spring(1, 2), true);
    assert_eq!(state.fit_spring(1, 3), false);
    assert_eq!(state.fit_spring(1, 4), false);

    assert_eq!(state.fit_spring(2, 1), true);
    assert_eq!(state.fit_spring(2, 2), false);
    assert_eq!(state.fit_spring(2, 3), false);
    */
}
