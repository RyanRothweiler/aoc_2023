#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use crate::perma::line::Line;
use crate::perma::twod::TwoD;
use crate::perma::v2::V2;

pub fn run() {
    let v = get_area("resources/inputs/day_10.txt");
    println!("{v}");
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Cell {
    ct: CellType,
    is_edge: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum CellType {
    Start,
    Ground,

    Vert,
    Hor,

    NE,
    NW,
    SE,
    SW,
}

impl CellType {
    fn from_char(input: char) -> CellType {
        match input {
            'S' => return CellType::Start,

            '-' => return CellType::Hor,
            '|' => return CellType::Vert,

            'F' => return CellType::NW,
            '7' => return CellType::NE,
            'L' => return CellType::SW,
            'J' => return CellType::SE,
            _ => return CellType::Ground,
        }
    }

    fn is_pipe(input: char) -> bool {
        match input {
            '-' | '|' | 'F' | '7' | 'L' | 'J' => return true,
            _ => return false,
        }
    }
}

// returns map and starting position
fn build_map(input: &str) -> (TwoD<Cell>, V2) {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut map: TwoD<Cell> = TwoD::new(
        lines[0].len(),
        lines.len(),
        Cell {
            ct: CellType::Ground,
            is_edge: false,
        },
    );

    let mut start = V2::new(-1, -1);
    let mut found_start = false;

    for y in 0..lines.len() {
        let line = lines[y];
        let chars: Vec<char> = line.chars().collect();

        for x in 0..chars.len() {
            let c = Cell {
                ct: CellType::from_char(chars[x]),
                is_edge: false,
            };
            map.set(x, y, c.clone());

            if c.ct == CellType::Start {
                start = V2::new(x as i64, y as i64);
                found_start = true;
            }
        }
    }

    if !found_start {
        panic!("Didn't find start.");
    }

    return (map, start);
}

// first entry is the start
fn get_loop_positions(map: &mut TwoD<Cell>, start: V2) -> Vec<V2> {
    let mut ret: Vec<V2> = vec![];

    ret.push(start);

    let mut current = V2::new(0, 0);
    let mut direction = V2::new(0, 0);

    // mark all edges
    // Find any adjacent cell to start on
    if map.get_i(start.x + 1, start.y).unwrap().ct != CellType::Ground {
        current = V2::new(start.x + 1, start.y);
        direction = V2::new(1, 0);
    } else if start.y != 0 && map.get_i(start.x, start.y - 1).unwrap().ct != CellType::Ground {
        current = V2::new(start.x, start.y - 1);
        direction = V2::new(0, -1);
    } else if map.get_i(start.x, start.y + 1).unwrap().ct != CellType::Ground {
        current = V2::new(start.x, start.y + 1);
        direction = V2::new(0, 1);
    } else if start.x != 0 && map.get_i(start.x - 1, start.y).unwrap().ct != CellType::Ground {
        current = V2::new(start.x - 1, start.y);
        direction = V2::new(-1, 0);
    }

    loop {
        ret.push(current);
        let curr_cell: &mut Cell = map.get_v2(current).unwrap();

        //print!("{:?}", current);

        match curr_cell.ct {
            CellType::Start => {
                return ret;
            }

            CellType::Ground => {
                panic!("Pipe led to ground. This should never happen.");
            }

            CellType::Hor | CellType::Vert => {
                // Do nothing. Keep the same direction
            }

            // 7
            CellType::NE => {
                //print!("7 ");
                if direction.x == 1 && direction.y == 0 {
                    direction = V2::new(0, 1);
                } else if direction.x == 0 && direction.y == -1 {
                    direction = V2::new(-1, 0);
                } else {
                    panic!("Invalid input direction. {direction:?}");
                }
            }

            // F
            CellType::NW => {
                //print!("F ");
                if direction.x == -1 && direction.y == 0 {
                    direction = V2::new(0, 1);
                } else if direction.x == 0 && direction.y == -1 {
                    direction = V2::new(1, 0);
                } else {
                    panic!("Invalid input direction. {direction:?}");
                }
            }

            // J
            CellType::SE => {
                //print!("J ");
                if direction.x == 1 && direction.y == 0 {
                    direction = V2::new(0, -1);
                } else if direction.x == 0 && direction.y == 1 {
                    direction = V2::new(-1, 0);
                } else {
                    panic!("Invalid input direction. {direction:?}");
                }
            }

            // L
            CellType::SW => {
                //print!("L ");
                if direction.x == -1 && direction.y == 0 {
                    direction = V2::new(0, -1);
                } else if direction.x == 0 && direction.y == 1 {
                    direction = V2::new(1, 0);
                } else {
                    panic!("Invalid input direction. {direction:?}");
                }
            }
        }

        //println!("");
        current = current + direction;
    }
}

fn get_segments(map: &mut TwoD<Cell>, start: V2) -> Vec<Line> {
    let mut ret: Vec<Line> = vec![];

    let positions = get_loop_positions(map, start);

    let mut next_line = Line::new_empty();
    let start = true;
    for p in &positions {
        let cell = map.get_v2(*p).unwrap();

        // is a corner
        if cell.ct == CellType::NE
            || cell.ct == CellType::NW
            || cell.ct == CellType::SW
            || cell.ct == CellType::SE
        {
            let v = cell.ct;

            let mut prev_point = V2::new(0, 0);
            if ret.len() == 0 {
                prev_point = *positions.first().unwrap();
            } else {
                prev_point = ret.last().unwrap().second;
            }

            ret.push(Line::new(prev_point, *p));
        }
    }

    // finish loop
    ret.push(Line::new(
        ret.last().unwrap().second,
        *positions.first().unwrap(),
    ));

    return ret;
}

fn mark_edges(map: &mut TwoD<Cell>, start: V2) {
    let lp = get_loop_positions(map, start);

    for pos in lp {
        map.get_v2(pos).unwrap().is_edge = true;
    }
}

// uses ray-intersection algorithm
// returns true for points on the edge
fn within_shape(point: V2, segments: &Vec<Line>) -> bool {

    // x offset needs to be at least width of map. Problematic magic number here.
    // Would not do this in production.
    // Aren't handling literal corners correctly. So pick these to avoid intersecting with corners.
    let line = Line::new(point, V2::new(point.x + 100000, point.y - 1));

    let mut count = 0;
    for s in segments {
        if line.intersects(*s) {
            count += 1;
        }
    }

    let v = count as f64 % 2.0;
    return v != 0.0;
}

fn get_area(input_file: &str) -> i64 {
    let contents = std::fs::read_to_string(input_file).unwrap();
    let mut map_info = build_map(&contents);

    let positions = get_loop_positions(&mut map_info.0, map_info.1);

    // This will get the loop positions again. Potential optimization to re-use the positions we
    // already have.
    let mut segments = get_segments(&mut map_info.0, map_info.1);

    let mut count = 0;
    for x in 0..map_info.0.width() {
        for y in 0..map_info.0.height() {
            let p = V2::new(x as i64, y as i64);
            if !positions.contains(&p) && within_shape(p, &mut segments) {
                count += 1;
            }
        }
    }

    return count;
}

fn get_half_len(input_file: &str) -> i64 {
    let contents = std::fs::read_to_string(input_file).unwrap();
    let mut mp = build_map(&contents);

    let lop = get_loop_positions(&mut mp.0, mp.1);

    return (lop.len() as f32 * 0.5) as i64;
}

#[test]
fn simple_loop() {
    assert_eq!(get_half_len("resources/day_10/day_10_easy.txt"), 4);
}

#[test]
fn complicated_loop() {
    assert_eq!(get_half_len("resources/day_10/day_10_hard.txt"), 8);
}

#[test]
fn within_shape_simple() {
    let contents = std::fs::read_to_string("resources/day_10/day_10_easy.txt").unwrap();
    let mut map_info = build_map(&contents);
    let mut segments = get_segments(&mut map_info.0, map_info.1);

    for i in 0..4 {
        assert_eq!(within_shape(V2::new(0, i), &segments), false);
    }
    for i in 0..4 {
        assert_eq!(within_shape(V2::new(4, i), &segments), false);
    }

    assert_eq!(within_shape(V2::new(1, 1), &segments), false);
    assert_eq!(within_shape(V2::new(2, 2), &segments), true);
    assert_eq!(within_shape(V2::new(2, 3), &segments), true);
    assert_eq!(within_shape(V2::new(3, 3), &segments), true);
    assert_eq!(within_shape(V2::new(4, 4), &segments), false);
    assert_eq!(within_shape(V2::new(1, 4), &segments), false);
}

#[test]
fn area_easy() {
    assert_eq!(get_area("resources/day_10/day_10_easy.txt"), 1);
}

#[test]
fn area_hards() {
    assert_eq!(get_area("resources/day_10/day_10_hard.txt"), 1);
    assert_eq!(get_area("resources/day_10/day_10_loop_back.txt"), 4);
    assert_eq!(get_area("resources/day_10/day_10_fancy.txt"), 10);
}

#[test]
fn segments() {
    let contents = std::fs::read_to_string("resources/day_10/day_10_easy.txt").unwrap();
    let mut map_info = build_map(&contents);
    let segs = get_segments(&mut map_info.0, map_info.1);

    assert_eq!(segs.len(), 4);

    assert_eq!(segs[0].first.x, 1);
    assert_eq!(segs[0].first.y, 1);
    assert_eq!(segs[0].second.x, 3);
    assert_eq!(segs[0].second.y, 1);

    assert_eq!(segs[2].first.x, 3);
    assert_eq!(segs[2].first.y, 3);
    assert_eq!(segs[2].second.x, 1);
    assert_eq!(segs[2].second.y, 3);

    assert_eq!(segs[3].first.x, 1);
    assert_eq!(segs[3].first.y, 3);
    assert_eq!(segs[3].second.x, 1);
    assert_eq!(segs[3].second.y, 1);
}
