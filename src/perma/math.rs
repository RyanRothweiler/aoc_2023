#![allow(unused_variables, dead_code)]

use super::v2::V2;

// kinda magic math stuff here.
// https://en.wikipedia.org/wiki/Shoelace_formula
// https://www.youtube.com/watch?v=0KjG8Pg6LGk
pub fn shoelace_area(input: &Vec<V2>) -> f64 {
    let mut sum: f64 = 0.0;

    // adds
    for i in 0..input.len() {
        let i_next = (i + 1) % input.len();
        sum += (input[i].x * input[i_next].y) as f64;
    }

    // subs
    for i in 0..input.len() {
        let i_next = (i + 1) % input.len();
        sum -= (input[i].y * input[i_next].x) as f64;
    }

    return sum * 0.5;
}

#[test]
fn shoelace_area_one() {
    let mut points: Vec<V2> = vec![];
    points.push(V2::new(4, 4));
    points.push(V2::new(0, 1));
    points.push(V2::new(-2, 5));
    points.push(V2::new(-6, 0));
    points.push(V2::new(-1, -4));
    points.push(V2::new(5, -2));

    assert_eq!(shoelace_area(&points), 55.0);
}

/*
#[test]
fn shoelace_area_two() {
    let mut points: Vec<V2> = vec![];
    points.push(V2::new(0, 0));
    points.push(V2::new(6, 0));
    points.push(V2::new(6, 5));
    points.push(V2::new(4, 5));
    points.push(V2::new(4, 7));
    points.push(V2::new(6, 7));
    points.push(V2::new(6, 9));
    points.push(V2::new(1, 9));
    points.push(V2::new(1, 7));
    points.push(V2::new(0, 7));
    points.push(V2::new(0, 5));
    points.push(V2::new(2, 5));
    points.push(V2::new(2, 2));
    points.push(V2::new(0, 2));

    assert_eq!(shoelace_area(&points), 42.0);
}
*/

/*
#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######
*/
