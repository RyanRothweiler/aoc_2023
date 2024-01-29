#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

pub fn run() {
    let contents = std::fs::read_to_string("resources/day_9/day_9_input.txt").unwrap();

    //let mut answers: Vec<i64> = vec![];
    let mut sum: i64 = 0;
    let lines: Vec<&str> = contents.split('\n').collect();

    for l in lines {
        if l.len() == 0 {
            continue;
        }

        // convert list of strings
        let mut seq: Vec<i64> = vec![];
        let nums_string: Vec<&str> = l.split(' ').collect();
        for ns in nums_string {
            let n: i64 = ns.trim().parse().unwrap_or_else(|error| 0);
            seq.push(n);
        }


        sum += pattern_next(seq);
    }

    println!("{sum}");
}

// not strictly necessary, here only for readabilty
struct Row {
    nums: Vec<i64>,
}

fn pattern_next(input: Vec<i64>) -> i64 {
    let mut rows: Vec<Row> = vec![];
    rows.push(Row { nums: input });

    let mut row_curr = 0;

    loop {
        let mut next_row = Row { nums: vec![] };

        let mut all_zero = true;
        for n in 1..rows[row_curr].nums.len() {
            let dif = rows[row_curr].nums[n] - rows[row_curr].nums[n - 1];
            next_row.nums.push(dif);

            //print!("{dif} ");

            if dif != 0 {
                all_zero = false;
            }
        }

        rows.push(next_row);
        row_curr += 1;

        //println!("");

        if all_zero {
            break;
        }
    }

    //println!(" --- back up ---");

    // move up the rows adding the new ending value
    // reverse the rows, so the bottom (the full 0 row) is at the top. easier to loop over.
    let mut new: i64 = 0;
    rows.reverse();
    for n in 1..rows.len() {
        for num in &rows[n].nums {
            //print!("{num} ");
        }

        let prev_row: i64 = *rows[n - 1].nums.last().unwrap();
        let curr_last: i64 = *rows[n].nums.last().unwrap();
        new = prev_row + curr_last;
        rows[n].nums.push(new);

        //print!("|| {prev_row} + {curr_last} = {new}");

        //println!("");
    }

    return new;
}

#[test]
fn two_layer() {
    assert_eq!(pattern_next(vec![0, 3, 6, 9, 12, 15]), 18);
}

#[test]
fn three_layer() {
    assert_eq!(pattern_next(vec![1, 3, 6, 10, 15, 21]), 28);
}

#[test]
fn sample_three() {
    assert_eq!(pattern_next(vec![10, 13, 16, 21, 30, 45]), 68);
}

#[test]
fn part_one() {
    let contents = std::fs::read_to_string("resources/day_9/day_9_input.txt").unwrap();

    //let mut answers: Vec<i64> = vec![];
    let mut sum: i64 = 0;
    let lines: Vec<&str> = contents.split('\n').collect();

    for l in lines {
        if l.len() == 0 {
            continue;
        }

        // convert list of strings
        let mut seq: Vec<i64> = vec![];
        let nums_string: Vec<&str> = l.split(' ').collect();
        for ns in nums_string {
            let n: i64 = ns.trim().parse().unwrap_or_else(|error| 0);
            seq.push(n);
        }


        sum += pattern_next(seq);
    }

    assert_eq!(sum, 2008960228);
}
