/* Improvements
 * - Map is being rebuilt every run. Can just reset the data.
 * - SHOULD be able to do the counting in one pass, this is doing two passes.
 */

#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use crate::perma::twod::TwoD;
use crate::perma::v2::V2;

pub fn part_one() {
    let contents = std::fs::read_to_string("resources/inputs/day_16.txt").unwrap();
    part_one_solve(&contents);
}

pub fn part_two() {
    let contents = std::fs::read_to_string("resources/inputs/day_16.txt").unwrap();
    part_two_solve(&contents);
}

fn part_two_solve(contents: &str) {
    let mut map = build_map(&contents);

    let mut max: i64 = 0;

    let mut starts: Vec<Ray> = vec![];
    for x in 0..map.width() {
        starts.push(Ray {
            pos: V2::new(x as i64, -1),
            dir: V2::new(0, 1),
        });

        starts.push(Ray {
            pos: V2::new(x as i64, map.height() as i64),
            dir: V2::new(0, -1),
        });
    }
    for y in 0..map.height() {
        starts.push(Ray {
            pos: V2::new(-1, y as i64),
            dir: V2::new(1, 0),
        });

        starts.push(Ray {
            pos: V2::new(map.width() as i64, y as i64),
            dir: V2::new(-1, 0),
        });
    }

    for r in starts {
        // reset the map. This is very inefficient as this rebuilds the entire map from the string.
        map = build_map(&contents);

        let v = light_map(r, &mut map);
        if v > max {
            max = v;
        }
    }
    println!("{max}");
}

fn part_one_solve(contents: &str) {
    let mut map = build_map(&contents);

    let v = light_map(
        Ray {
            pos: V2::new(-1, 0),
            dir: V2::new(1, 0),
        },
        &mut map,
    );
    println!("{v}");
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Cell {
    ty: char,

    // use for couting light amount
    visited_count: bool,

    // Has this cell been visisted by a horizontal or vertical ray
    // Used so that we don't run the ray forever.
    visited_hor: bool,
    visited_ver: bool,
}

impl Cell {
    fn new(ty: char) -> Self {
        Self {
            ty,
            visited_hor: false,
            visited_ver: false,
            visited_count: false,
        }
    }

    fn light(&mut self, dir: V2) {
        if dir.x > 0 || dir.x < 0 {
            self.visited_hor = true;
        }
        if dir.y > 0 || dir.y < 0 {
            self.visited_ver = true;
        }
    }
}

fn build_map(file_data: &str) -> TwoD<Cell> {
    let lines: Vec<&str> = file_data.lines().collect();
    let first_chars: Vec<char> = lines[0].trim().chars().collect();
    let mut map: TwoD<Cell> = TwoD::new(first_chars.len(), lines.len(), Cell::new('.'));

    for y in 0..map.height() {
        let line: &str = lines[y].trim();
        let chars: Vec<char> = line.trim().chars().collect();

        for x in 0..map.width() {
            map.set(x, y, Cell::new(chars[x]));
        }
    }

    return map;
}

#[derive(Debug)]
struct Ray {
    pos: V2,
    dir: V2,
}

// ray origin is current cell, new_cell is the next cell the ray will be entering
// returns a list of new rays
fn direct_ray(mut ray: Ray, new_cell: &mut Cell) -> Vec<Ray> {
    new_cell.visited_count = true;

    let next_pos = ray.pos + ray.dir;

    // This cell has already been visited then end the ray
    if ray.dir.y != 0 && new_cell.visited_ver {
        return vec![];
    }
    if ray.dir.x != 0 && new_cell.visited_hor {
        return vec![];
    }

    match new_cell.ty {
        '.' => {
            // ray continues in same direction
            ray.pos = next_pos;
            new_cell.light(ray.dir);
            return vec![ray];
        }
        '|' => {
            new_cell.visited_hor = true;
            new_cell.visited_ver = true;

            // split the ray if moving horizontally
            if ray.dir.x != 0 {
                let up = Ray {
                    pos: next_pos,
                    dir: V2::new(0, -1),
                };
                let down = Ray {
                    pos: next_pos,
                    dir: V2::new(0, 1),
                };

                return vec![up, down];
            } else {
                // ray continues in same direction
                ray.pos = next_pos;
                new_cell.light(ray.dir);
                return vec![ray];
            }
        }
        '-' => {
            new_cell.visited_hor = true;
            new_cell.visited_ver = true;

            // split the ray if moving vertically
            if ray.dir.y != 0 {
                let left = Ray {
                    pos: next_pos,
                    dir: V2::new(-1, 0),
                };
                let right = Ray {
                    pos: next_pos,
                    dir: V2::new(1, 0),
                };

                new_cell.visited_hor = true;
                new_cell.visited_ver = true;

                return vec![left, right];
            } else {
                // ray continues in same direction
                ray.pos = next_pos;
                new_cell.light(ray.dir);
                return vec![ray];
            }
        }

        '\\' => {
            // don't set visited hor or ver here because direction matters

            let mut next_ray = Ray {
                pos: next_pos,
                dir: V2::new(0, 0),
            };

            if ray.dir.x > 0 {
                next_ray.dir = V2::new(0, 1);
            } else if ray.dir.x < 0 {
                next_ray.dir = V2::new(0, -1);
            } else if ray.dir.y < 0 {
                next_ray.dir = V2::new(-1, 0);
            } else if ray.dir.y > 0 {
                next_ray.dir = V2::new(1, 0);
            } else {
                panic!("A ray with no direction. This shoudn't ever happen.");
            }

            return vec![next_ray];
        }
        '/' => {
            // don't set visited hor or ver here because direction matters

            let mut next_ray = Ray {
                pos: next_pos,
                dir: V2::new(0, 0),
            };

            if ray.dir.x > 0 {
                next_ray.dir = V2::new(0, -1);
            } else if ray.dir.x < 0 {
                next_ray.dir = V2::new(0, 1);
            } else if ray.dir.y < 0 {
                next_ray.dir = V2::new(1, 0);
            } else if ray.dir.y > 0 {
                next_ray.dir = V2::new(-1, 0);
            } else {
                panic!("A ray with no direction. This shoudn't ever happen.");
            }

            return vec![next_ray];
        }
        _ => {
            panic!("Map is invalid. Unknown character");
        }
    };
}

fn light_map(start: Ray, map: &mut TwoD<Cell>) -> i64 {
    let mut lights: Vec<Ray> = vec![];
    lights.push(start);

    let mut count: i64 = 0;
    loop {
        if lights.len() == 0 {
            break;
        }

        let mut ray = lights.pop().unwrap();
        let next_pos = ray.pos + ray.dir;

        match map.get_v2(next_pos) {
            Some(cell) => {
                let new_rays = direct_ray(ray, cell);
                for r in new_rays {
                    lights.push(r);
                }
            }
            None => {
                // if next position is out of bounds then continue, this ray is done.
                continue;
            }
        }
    }

    for x in 0..map.width() {
        for y in 0..map.height() {
            let cell = map.get(x, y).unwrap();
            if cell.visited_count {
                count += 1;
            }
        }
    }

    return count;
}

#[test]
fn sample() {
    let contents = std::fs::read_to_string("resources/day_16/day_16_sample.txt").unwrap();
    let mut map = build_map(&contents);
    let start = Ray {
        pos: V2::new(-1, 0),
        dir: V2::new(1, 0),
    };
    let v = light_map(start, &mut map);
    assert_eq!(v, 46);
}

#[test]
fn ray_mirror_ground() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(1, 0),
    };

    let mut ground = Cell::new('.');
    let new_rays = direct_ray(ray, &mut ground);

    assert_eq!(ground.visited_hor, true);
    assert_eq!(ground.visited_ver, false);
    assert_eq!(new_rays.len(), 1);
    assert_eq!(new_rays[0].pos.x, 11);
    assert_eq!(new_rays[0].pos.y, 10);
}

#[test]
fn ray_mirror_vsplit() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(1, 0),
    };

    let mut cell = Cell::new('|');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, true);
    assert_eq!(cell.visited_ver, true);
    assert_eq!(new_rays.len(), 2);

    assert_eq!(new_rays[0].pos.x, 11);
    assert_eq!(new_rays[0].pos.y, 10);
    assert_eq!(new_rays[0].dir.x, 0);
    assert_eq!(new_rays[0].dir.y, -1);

    assert_eq!(new_rays[1].pos.x, 11);
    assert_eq!(new_rays[1].pos.y, 10);
    assert_eq!(new_rays[1].dir.x, 0);
    assert_eq!(new_rays[1].dir.y, 1);
}

#[test]
fn ray_mirror_vsplit_v() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(0, 1),
    };

    let mut cell = Cell::new('|');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, true);
    assert_eq!(cell.visited_ver, true);
    assert_eq!(new_rays.len(), 1);

    assert_eq!(new_rays[0].pos.x, 10);
    assert_eq!(new_rays[0].pos.y, 11);
    assert_eq!(new_rays[0].dir.x, 0);
    assert_eq!(new_rays[0].dir.y, 1);
}

#[test]
fn ray_mirror_hsplit() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(0, 1),
    };

    let mut cell = Cell::new('-');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, true);
    assert_eq!(cell.visited_ver, true);
    assert_eq!(new_rays.len(), 2);

    assert_eq!(new_rays[0].pos.x, 10);
    assert_eq!(new_rays[0].pos.y, 11);
    assert_eq!(new_rays[0].dir.x, -1);
    assert_eq!(new_rays[0].dir.y, 0);

    assert_eq!(new_rays[1].pos.x, 10);
    assert_eq!(new_rays[1].pos.y, 11);
    assert_eq!(new_rays[1].dir.x, 1);
    assert_eq!(new_rays[1].dir.y, 0);
}

#[test]
fn ray_mirror_hsplit_h() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(1, 0),
    };

    let mut cell = Cell::new('-');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, true);
    assert_eq!(cell.visited_ver, true);
    assert_eq!(new_rays.len(), 1);

    assert_eq!(new_rays[0].pos.x, 11);
    assert_eq!(new_rays[0].pos.y, 10);
    assert_eq!(new_rays[0].dir.x, 1);
    assert_eq!(new_rays[0].dir.y, 0);
}

#[test]
fn ray_mirror_visited_horizontal() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(1, 0),
    };

    let mut cell = Cell::new('|');
    cell.visited_hor = true;
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(new_rays.len(), 0);
}

#[test]
fn ray_mirror_visited_vertical() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(0, 1),
    };

    let mut cell = Cell::new('.');
    cell.visited_ver = true;
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(new_rays.len(), 0);
}

#[test]
fn ray_mirror_fslash_north() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(0, -1),
    };

    let mut cell = Cell::new('/');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, false);
    assert_eq!(cell.visited_ver, false);
    assert_eq!(new_rays.len(), 1);

    assert_eq!(new_rays[0].pos.x, 10);
    assert_eq!(new_rays[0].pos.y, 9);
    assert_eq!(new_rays[0].dir.x, 1);
    assert_eq!(new_rays[0].dir.y, 0);
}

#[test]
fn ray_mirror_fslash_east() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(1, 0),
    };

    let mut cell = Cell::new('/');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, false);
    assert_eq!(cell.visited_ver, false);
    assert_eq!(new_rays.len(), 1);

    assert_eq!(new_rays[0].pos.x, 11);
    assert_eq!(new_rays[0].pos.y, 10);
    assert_eq!(new_rays[0].dir.x, 0);
    assert_eq!(new_rays[0].dir.y, -1);
}

#[test]
fn ray_mirror_fslash_south() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(0, 1),
    };

    let mut cell = Cell::new('/');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, false);
    assert_eq!(cell.visited_ver, false);
    assert_eq!(new_rays.len(), 1);

    assert_eq!(new_rays[0].pos.x, 10);
    assert_eq!(new_rays[0].pos.y, 11);
    assert_eq!(new_rays[0].dir.x, -1);
    assert_eq!(new_rays[0].dir.y, 0);
}

#[test]
fn ray_mirror_fslash_west() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(-1, 0),
    };

    let mut cell = Cell::new('/');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, false);
    assert_eq!(cell.visited_ver, false);
    assert_eq!(new_rays.len(), 1);

    assert_eq!(new_rays[0].pos.x, 9);
    assert_eq!(new_rays[0].pos.y, 10);
    assert_eq!(new_rays[0].dir.x, 0);
    assert_eq!(new_rays[0].dir.y, 1);
}

#[test]
fn ray_mirror_bslash_north() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(0, -1),
    };

    let mut cell = Cell::new('\\');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, false);
    assert_eq!(cell.visited_ver, false);
    assert_eq!(new_rays.len(), 1);

    assert_eq!(new_rays[0].pos.x, 10);
    assert_eq!(new_rays[0].pos.y, 9);
    assert_eq!(new_rays[0].dir.x, -1);
    assert_eq!(new_rays[0].dir.y, 0);
}

#[test]
fn ray_mirror_bslash_east() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(1, 0),
    };

    let mut cell = Cell::new('\\');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, false);
    assert_eq!(cell.visited_ver, false);
    assert_eq!(new_rays.len(), 1);

    assert_eq!(new_rays[0].pos.x, 11);
    assert_eq!(new_rays[0].pos.y, 10);
    assert_eq!(new_rays[0].dir.x, 0);
    assert_eq!(new_rays[0].dir.y, 1);
}

#[test]
fn ray_mirror_bslash_south() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(0, 1),
    };

    let mut cell = Cell::new('\\');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, false);
    assert_eq!(cell.visited_ver, false);
    assert_eq!(new_rays.len(), 1);

    assert_eq!(new_rays[0].pos.x, 10);
    assert_eq!(new_rays[0].pos.y, 11);
    assert_eq!(new_rays[0].dir.x, 1);
    assert_eq!(new_rays[0].dir.y, 0);
}

#[test]
fn ray_mirror_bslash_west() {
    let mut ray = Ray {
        pos: V2::new(10, 10),
        dir: V2::new(-1, 0),
    };

    let mut cell = Cell::new('\\');
    let new_rays = direct_ray(ray, &mut cell);

    assert_eq!(cell.visited_hor, false);
    assert_eq!(cell.visited_ver, false);
    assert_eq!(new_rays.len(), 1);

    assert_eq!(new_rays[0].pos.x, 9);
    assert_eq!(new_rays[0].pos.y, 10);
    assert_eq!(new_rays[0].dir.x, 0);
    assert_eq!(new_rays[0].dir.y, -1);
}
