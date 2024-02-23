#![allow(unused_variables, dead_code)]

use super::v2::V2;

// kinda magic math stuff here.
// https://en.wikipedia.org/wiki/Shoelace_formula
// https://www.youtube.com/watch?v=0KjG8Pg6LGk
pub fn shoelace_area(input: &Vec<V2>) -> i64 {
    let mut sum: i64 = 0;

    // adds
    for i in 0..input.len() {
        let i_next = (i + 1) % input.len();
        sum += (input[i].x * input[i_next].y) as i64;
    }

    // subs
    for i in 0..input.len() {
        let i_next = (i + 1) % input.len();
        sum -= (input[i].y * input[i_next].x) as i64;
    }

    return sum / 2;
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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

    assert_eq!(shoelace_area(&points), 55);
}

#[test]
fn lcm_test() {
    assert_eq!(lcm(&[1, 2, 3, 4, 5]), 60);
    assert_eq!(lcm(&[2, 4, 6, 8, 10]), 120);
    assert_eq!(lcm(&[3, 6, 9, 12, 15]), 180);
    assert_eq!(lcm(&[10]), 10);
    assert_eq!(lcm(&[21, 110]), 2310);
}
