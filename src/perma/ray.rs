#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use super::v2::V2;

pub struct Ray {
    origin: V2,
    dir: V2,
}

impl Ray {
    pub fn new(origin: V2, dir: V2) -> Ray {
        Ray { origin, dir }
    }
}
