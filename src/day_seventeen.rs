/* Improvemnets
 * - Use enum for the direcions.
 *
 */

#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::perma::twod::TwoD;
use crate::perma::v2::V2;

pub fn run() {
    let v = part_one("resources/inputs/day_17.txt");
    println!("{v}");
}

#[derive(Hash, Eq, PartialEq)]
struct Arrival {
    pos: V2,
    forward: V2,
    count_forward: i64,
}

// returns cost of min path (heat loss)
pub fn part_one(file_dir: &str) -> i64 {
    let contents = std::fs::read_to_string(file_dir).unwrap();
    let mut map = build_map(&contents);

    // goals is bottom right corner
    let goal = V2::new(map.width() as i64 - 1, map.height() as i64 - 1);

    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    let mut previous_paths: HashMap<Arrival, i64> = HashMap::new();

    // add starting pos
    frontier.push(State {
        cost: 0,
        pos: V2::new(0, 0),
        forward: V2::new(1, 0),
        count_forward: 1,
    });

    loop {
        let next_best = frontier.pop().unwrap();

        if next_best.pos == goal {
            return next_best.cost;
        }

        let adjacents = valid_adjacents(next_best, &mut map);
        for s in adjacents {
            let arr = Arrival {
                pos: s.pos,
                forward: s.forward,
                count_forward: s.count_forward,
            };

            if previous_paths.contains_key(&arr) {
                let mut prev_cost = *previous_paths.get(&arr).unwrap();
                if s.cost < prev_cost {
                    previous_paths.insert(arr, s.cost);
                    frontier.push(s);
                }
            } else {
                previous_paths.insert(arr, s.cost);
                frontier.push(s);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: i64,
    pos: V2,
    forward: V2,
    count_forward: i64,
}

// Following example on the binary_heap docs.
// Explicit implementation to make the binary_heap a 'min heap'
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.x.cmp(&other.pos.x))
            .then_with(|| self.pos.y.cmp(&other.pos.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cell {
    map_cost: i64,
}

impl Cell {
    fn new(map_cost: i64) -> Cell {
        Cell { map_cost }
    }

    fn dir_to_index(input: V2) -> usize {
        if input.x < 0 {
            return 0;
        } else if input.x > 0 {
            return 1;
        } else if input.y < 0 {
            return 2;
        } else if input.y > 0 {
            return 3;
        } else {
            panic!("Invalid direction");
        }
    }
}

// Returns only the possible adjacents given the crucible rules
fn valid_adjacents(state: State, map: &mut TwoD<Cell>) -> Vec<State> {
    let mut ret: Vec<State> = vec![];

    let mut dirs: Vec<V2> = vec![];
    dirs.push(V2::new(1, 0));
    dirs.push(V2::new(-1, 0));
    dirs.push(V2::new(0, 1));
    dirs.push(V2::new(0, -1));

    for d in dirs {
        // can't move in the same direction more than 3 times
        if d == state.forward && state.count_forward >= 3 {
            continue;
        }

        // can't move backwards
        if d + state.forward == V2::new(0, 0) {
            continue;
        }

        let next_pos = state.pos + d;

        match map.get_v2(next_pos) {
            Some(c) => {
                let mut new_state = State {
                    cost: state.cost + c.map_cost,
                    pos: next_pos,
                    forward: d,
                    count_forward: 1,
                };

                // update forward
                if d == state.forward {
                    new_state.count_forward = state.count_forward + 1;
                }

                ret.push(new_state);
            }
            None => {}
        }
    }

    return ret;
}

fn build_map(file_data: &str) -> TwoD<Cell> {
    let lines: Vec<&str> = file_data.lines().collect();
    let first_chars: Vec<char> = lines[0].trim().chars().collect();
    let mut map: TwoD<Cell> = TwoD::new(first_chars.len(), lines.len(), Cell::new(0));

    for y in 0..map.height() {
        let line: &str = lines[y].trim();
        let chars: Vec<char> = line.trim().chars().collect();

        for x in 0..map.width() {
            map.set(x, y, Cell::new(chars[x].to_digit(10).unwrap() as i64));
        }
    }

    return map;
}

#[test]
fn sample() {
    let v = part_one("resources/day_17/day_17_sample.txt");
    assert_eq!(v, 102);
}

#[test]
fn adjacents() {
    let contents = std::fs::read_to_string("resources/day_17/day_17_sample.txt").unwrap();
    let mut map = build_map(&contents);

    let mut state = State {
        cost: 0,
        pos: V2::new(2, 2),
        forward: V2::new(1, 0),
        count_forward: 1,
    };

    let adj = valid_adjacents(state, &mut map);

    assert_eq!(adj.len(), 3);

    assert_eq!(adj[0].forward, V2::new(1, 0));
    assert_eq!(adj[0].pos, V2::new(3, 2));
    assert_eq!(adj[0].count_forward, 2);
    assert_eq!(adj[0].cost, 5);

    assert_eq!(adj[1].forward, V2::new(0, 1));
    assert_eq!(adj[1].pos, V2::new(2, 3));
    assert_eq!(adj[1].count_forward, 1);
    assert_eq!(adj[1].cost, 4);

    assert_eq!(adj[2].forward, V2::new(0, -1));
    assert_eq!(adj[2].pos, V2::new(2, 1));
    assert_eq!(adj[2].count_forward, 1);
    assert_eq!(adj[2].cost, 1);

    let adj = valid_adjacents(adj[1], &mut map);

    assert_eq!(adj.len(), 3);

    assert_eq!(adj[0].forward, V2::new(1, 0));
    assert_eq!(adj[0].pos, V2::new(3, 3));
    assert_eq!(adj[0].count_forward, 1);
    assert_eq!(adj[0].cost, 10);
}
