#![allow(dead_code)]

use crate::perma::v2::V2;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct TwoD<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: std::clone::Clone> TwoD<T> {
    pub fn new(w: usize, h: usize, def: T) -> TwoD<T> {
        TwoD {
            cells: vec![def; w * h],
            width: w,
            height: h,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    // TODO change this to get_mut and add a non mut get option
    pub fn get(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let i = (y * self.width) + x;
        match self.cells.get_mut(i) {
            Some(t) => return Some(t),
            None => return None,
        }
    }

    pub fn get_i(&mut self, x: i64, y: i64) -> Option<&mut T> {
        return self.get(usize::try_from(x).unwrap(), usize::try_from(y).unwrap());
    }

    pub fn get_v2(&mut self, pos: V2) -> Option<&mut T> {
        return self.get(
            usize::try_from(pos.x).unwrap(),
            usize::try_from(pos.y).unwrap(),
        );
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        let i = (y * self.width) + x;
        self.cells[i] = val;
    }
}
