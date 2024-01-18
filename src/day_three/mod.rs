#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

use std::fs;

/*

- build 2d array
- iterate through array until find a symbol
- iterate through every cell adjacent to the symbol
    - if cell isn't visited yet
        - mark as visited
        - expand that cell horizontally to get the full number
        - parse and add number to sum

 */
pub fn run() {
    let contents: String = fs::read_to_string("resources/day_3_testing_single_number.txt")
        .expect("Couldn't find file.");

    process(&contents);
}

#[derive(Clone)]
struct Cell {
    data: char,
    visited: bool,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            data: '0',
            visited: false,
        }
    }
}

struct TwoDimArray<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: std::clone::Clone> TwoDimArray<T> {
    fn new(w: usize, h: usize, def: T) -> TwoDimArray<T> {
        TwoDimArray {
            cells: vec![def; w * h],
            width: w,
            height: h,
        }
    }

    fn get(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let i = (y * self.width) + x;
        match self.cells.get_mut(i) {
            Some(t) => return Some(t),
            None => return None,
        }
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        let i = (y * self.width) + x;
        self.cells[i] = val;
    }
}

struct V2I {
    x: i64,
    y: i64,
}

fn process(input: &str) -> u64 {
    let directions: Vec<V2I> = vec![
        V2I { x: 1, y: 0 },
        V2I { x: -1, y: 0 },
        V2I { x: 0, y: 1 },
        V2I { x: 0, y: -1 },
        V2I { x: 1, y: -1 },
        V2I { x: -1, y: 1 },
        V2I { x: -1, y: -1 },
        V2I { x: 1, y: 1 },
    ];

    let mut sum = 0;

    let lines: Vec<&str> = input.split('\n').collect();

    // -1 to remove the endline characters
    let width = lines[0].len() - 1;
    let height = lines.len();

    let mut cells = TwoDimArray::new(width, height, Cell::new());

    // copy data into cells
    for x in 0..cells.width {
        for y in 0..cells.height {
            let row: Vec<char> = lines[y].chars().collect();
            let c = row[x];
            //println!("{c}");

            cells.set(
                x,
                y,
                Cell {
                    data: row[x],
                    visited: false,
                },
            );
        }
    }

    // Go through cells, find a symbol, then check all adjacent cells to find the numbers
    for y in 0..cells.height {
        for x in 0..cells.width {
            match cells.get(x, y) {
                Some(data) => {
                    let c = data.data;
                    if c != '.' && !c.is_alphanumeric() && !data.visited {
                        //println!("symbol {c}");

                        data.visited = true;

                        for dir in &directions {
                            let x_fin: i64 = (x as i64) + dir.x;
                            let y_fin: i64 = (y as i64) + dir.y;

                            if x_fin >= 0 && y_fin >= 0 {
                                //println!("checking {x_fin} {y_fin}");
                                match check_direction(&mut cells, x_fin as usize, y_fin as usize) {
                                    Some(t) => {
                                        //println!("number {t}");
                                        sum = sum + t;
                                    }
                                    None => {}
                                };
                            }
                        }
                    }
                }
                None => {
                    //println!("Invalid index {x},{y}");
                }
            }
        }
    }

    //println!("width {width}x{height}");
    sum
}

fn check_direction(cells: &mut TwoDimArray<Cell>, x: usize, y: usize) -> Option<u64> {
    match cells.get(x, y) {
        Some(d) => {
            // Is that a number
            let c = d.data;
            if !d.visited && c.is_alphanumeric() {
                // Expand to find the full number
                let mut start_index: usize = x;
                let mut end_index: usize = x;

                // get start of number
                loop {
                    match cells.get(start_index, y) {
                        Some(e) => {
                            if !e.visited && !e.data.is_alphanumeric() {
                                break;
                            } else {
                                let zz = e.data;
                                //println!("NO {zz}");
                                start_index = start_index - 1;
                                e.visited = true;
                            }
                        }
                        None => break,
                    }
                }

                // get end of number
                loop {
                    match cells.get(end_index, y) {
                        Some(e) => {
                            if !e.visited && !e.data.is_alphanumeric() {
                                break;
                            } else {
                                let zz = e.data;
                                //println!("NO {zz}");
                                end_index = end_index + 1;
                                e.visited = true;
                            }
                        }
                        None => break,
                    }
                }

                // Pull number into string
                let mut val_string = String::new();
                for i in (start_index + 1)..end_index {
                    match cells.get(i, y) {
                        Some(e) => {
                            val_string.push(e.data);
                        }
                        None => {
                            eprintln!("Error extracting number.");
                            return None;
                        }
                    }
                }

                //println!("{x},{y} start->{start_index} end->{end_index}, str->{val_string}");

                let val: u64 = match val_string.parse() {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("Error parsing string to number {e}");
                        return None;
                    }
                };

                //println!("val->{val}");
                return Some(val);
            }
        }
        None => return None,
    }

    None
}

#[test]
fn single_number() {
    let contents: String = fs::read_to_string("resources/day_3_testing_single_number.txt")
        .expect("Couldn't find file.");
    assert_eq!(process(&contents), 18);
}

#[test]
fn single_number_edge() {
    let contents: String = fs::read_to_string("resources/day_3_testing_single_number_edge.txt")
        .expect("Couldn't find file.");
    assert_eq!(process(&contents), 18);
}

#[test]
fn two() {
    let contents: String =
        fs::read_to_string("resources/day_3_testing_two.txt").expect("Couldn't find file.");
    assert_eq!(process(&contents), 118);
}
