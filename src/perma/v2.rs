#![allow(dead_code)]

use std::ops;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct V2 {
    pub x: i64,
    pub y: i64,
}

impl V2 {
    pub fn new(x: i64, y: i64) -> V2 {
        V2 { x, y }
    }

    pub fn manhattan_dist(first: &V2, second: &V2) -> i64 {
        let x = (first.x - second.x).abs();
        let y = (first.y - second.y).abs();
        return x + y;
    }

    pub fn x_as_usize(self) -> usize {
        return usize::try_from(self.x).unwrap();
    }

    pub fn y_as_usize(self) -> usize {
        return usize::try_from(self.y).unwrap();
    }
}

impl ops::Add<V2> for V2 {
    type Output = V2;

    fn add(self, rhs: V2) -> V2 {
        V2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[test]
fn manhattan_distnace() {
    assert_eq!(V2::manhattan_dist(&V2::new(0, 0), &V2::new(5, 5)), 10);
    assert_eq!(V2::manhattan_dist(&V2::new(-5, 0), &V2::new(5, 5)), 15);
    assert_eq!(V2::manhattan_dist(&V2::new(5, 10), &V2::new(5, 5)), 5);
    assert_eq!(V2::manhattan_dist(&V2::new(0, 10), &V2::new(5, 0)), 15);
    assert_eq!(V2::manhattan_dist(&V2::new(1, 6), &V2::new(5, 11)), 9);
    assert_eq!(V2::manhattan_dist(&V2::new(4, 0), &V2::new(9, 10)), 15);
}
