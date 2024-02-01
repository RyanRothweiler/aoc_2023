#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use super::ray::Ray;
use super::v2::V2;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Line {
    pub first: V2,
    pub second: V2,
}

impl Line {
    pub fn new(first: V2, second: V2) -> Line {
        Line { first, second }
    }

    pub fn new_empty() -> Line {
        Line {
            first: V2::new(0, 0),
            second: V2::new(0, 0),
        }
    }

    //https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect/565282#565282
    pub fn intersects(&self, input: Line) -> bool {
        let p0_x = self.first.x as f64;
        let p0_y = self.first.y as f64;
        let p1_x = self.second.x as f64;
        let p1_y = self.second.y as f64;

        let p2_x = input.first.x as f64;
        let p2_y = input.first.y as f64;
        let p3_x = input.second.x as f64;
        let p3_y = input.second.y as f64;

        let s1_x = p1_x - p0_x;
        let s1_y = p1_y - p0_y;

        let s2_x = p3_x - p2_x;
        let s2_y = p3_y - p2_y;

        let s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / (-s2_x * s1_y + s1_x * s2_y);
        let t = (s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / (-s2_x * s1_y + s1_x * s2_y);

        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
            return true;
        }

        return false;
    }
}

#[test]
fn line_line_intersection_true() {
    let first = Line::new(V2::new(0, 0), V2::new(3, 3));
    let second = Line::new(V2::new(0, 1), V2::new(1, 0));
    assert_eq!(first.intersects(second), true);
}

#[test]
fn line_line_intersection_false() {
    let first = Line::new(V2::new(0, 0), V2::new(3, 3));
    let second = Line::new(V2::new(-1, -1), V2::new(-5, 1));
    assert_eq!(first.intersects(second), false);
}
