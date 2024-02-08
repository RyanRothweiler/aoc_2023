#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

pub fn run() {
    let contents = std::fs::read_to_string("resources/inputs/day_15.txt").unwrap();

    let entries: Vec<&str> = contents.split(',').collect();
    let mut sum: i64 = 0;
    for e in entries {
        sum += hash(e);
    }
    println!("{sum}");
}

fn hash(input: &str) -> i64 {
    let mut v: i64 = 0;

    for c in input.bytes() {
        v += c as i64;
        v = v * 17;
        v = v % 256;
    }

    return v;
}

#[test]
fn hash_test() {
    assert_eq!(hash("rn=1"), 30);
    assert_eq!(hash("cm-"), 253);
    assert_eq!(hash("qp=3"), 97);
}
