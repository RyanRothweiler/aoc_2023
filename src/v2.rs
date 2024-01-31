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
