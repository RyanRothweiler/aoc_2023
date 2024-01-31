#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use crate::twod::TwoD;
use crate::v2::V2;

pub fn run() {
    let v = get_half_len("resources/inputs/day_10.txt");
    println!("{v}");
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Start,
    Ground,

    Vert,
    Hor,

    NE,
    NW,
    SE,
    SW,
}

impl Cell {
    fn from_char(input: char) -> Cell {
        match input {
            'S' => return Cell::Start,

            '-' => return Cell::Hor,
            '|' => return Cell::Vert,

            'F' => return Cell::NW,
            '7' => return Cell::NE,
            'L' => return Cell::SW,
            'J' => return Cell::SE,
            _ => return Cell::Ground,
        }
    }

    fn is_pipe(input: char) -> bool {
        match input {
            '-' | '|' | 'F' | '7' | 'L' | 'J' => return true,
            _ => return false,
        }
    }
}

fn get_half_len(input: &str) -> i64 {
    //let contents = std::fs::read_to_string("resources/day_10/day_10_simple.txt").unwrap();
    let contents = std::fs::read_to_string(input).unwrap();

    let lines: Vec<&str> = contents.split('\n').collect();

    // build map
    let mut map: TwoD<Cell> = TwoD::new(lines[0].len(), lines.len(), Cell::Ground);
    let mut start = V2::new(-1, -1);
    for y in 0..lines.len() {
        let line = lines[y];
        let chars: Vec<char> = line.chars().collect();

        for x in 0..chars.len() {
            let c = Cell::from_char(chars[x]);
            map.set(x, y, c.clone());

            if c == Cell::Start {
                start = V2::new(x as i64, y as i64);
            }
        }
    }

    // count steps in loop
    let mut steps: i64 = 0;

    let mut current = V2::new(0, 0);
    let mut direction = V2::new(0, 0);

    // Find any adjacent cell to start on
    // NOTE this will error if the start is on the edge.
    if *map.get_i(start.x + 1, start.y).unwrap() != Cell::Ground {
        current = V2::new(start.x + 1, start.y);
        direction = V2::new(1, 0);
    } else if *map.get_i(start.x, start.y - 1).unwrap() != Cell::Ground {
        current = V2::new(start.x, start.y - 1);
        direction = V2::new(0, -1);
    } else if *map.get_i(start.x, start.y + 1).unwrap() != Cell::Ground {
        current = V2::new(start.x, start.y + 1);
        direction = V2::new(0, 1);
    } else if *map.get_i(start.x - 1, start.y).unwrap() != Cell::Ground {
        current = V2::new(start.x - 1, start.y);
        direction = V2::new(-1, 0);
    }

    loop {
        let curr_cell: Cell = *map.get_v2(current).unwrap();

        //print!("{:?}", current);

        match curr_cell {
            Cell::Start => {
                break;
            }

            Cell::Ground => {
                panic!("Pipe led to ground. This should never happen.");
            }

            Cell::Hor | Cell::Vert => {
                // Do nothing. Keep the same direction
            }

            // 7
            Cell::NE => {
                //print!("7 ");
                if direction.x == 1 && direction.y == 0 {
                    direction = V2::new(0, 1);
                } else if direction.x == 0 && direction.y == -1 {
                    direction = V2::new(-1, 0);
                } else {
                    panic!("Invalid input direction.");
                }
            }

            // F
            Cell::NW => {
                //print!("F ");
                if direction.x == -1 && direction.y == 0 {
                    direction = V2::new(0, 1);
                } else if direction.x == 0 && direction.y == -1 {
                    direction = V2::new(1, 0);
                } else {
                    panic!("Invalid input direction.");
                }
            }

            // J
            Cell::SE => {
                //print!("J ");
                if direction.x == 1 && direction.y == 0 {
                    direction = V2::new(0, -1);
                } else if direction.x == 0 && direction.y == 1 {
                    direction = V2::new(-1, 0);
                } else {
                    panic!("Invalid input direction.");
                }
            }

            // L
            Cell::SW => {
                //print!("L ");
                if direction.x == -1 && direction.y == 0 {
                    direction = V2::new(0, -1);
                } else if direction.x == 0 && direction.y == 1 {
                    direction = V2::new(1, 0);
                } else {
                    panic!("Invalid input direction.");
                }
            }
        }

        //println!("");
        steps += 1;
        current = current + direction;
    }

    // include the start
    steps += 1;

    return (steps as f32 * 0.5) as i64;
}

#[test]
fn simple_loop() {
    assert_eq!(get_half_len("resources/day_10/day_10_easy.txt"), 4);
}

#[test]
fn complicated_loop() {
    assert_eq!(get_half_len("resources/day_10/day_10_hard.txt"), 8);
}
