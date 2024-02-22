#![allow(unused_variables, dead_code, unused_imports)]

mod perma;

mod day_eight;
mod day_eighteen;
mod day_eleven;
mod day_fifteen;
mod day_five;
mod day_four;
mod day_fourteen;
mod day_nine;
mod day_nineteen;
mod day_one;
mod day_seven;
mod day_seventeen;
mod day_six;
mod day_sixteen;
mod day_ten;
mod day_thirteen;
mod day_three;
mod day_twelve;
mod day_twenty;
mod day_two;

use std::collections::HashMap;

fn main() {
    // format {day} {part}
    let args: Vec<String> = std::env::args().collect();

    let day = match args.get(1) {
        Some(v) => v,
        _ => {
            eprintln!("Invalid argument format.");
            return;
        }
    };
    let day: i64 = day.parse().expect("Day argument is not a number");

    let part = match args.get(2) {
        Some(v) => v,
        _ => {
            eprintln!("Invalid argument format.");
            return;
        }
    };
    let part: i64 = part.parse().expect("Part argument is not a number");
    if part > 2 {
        eprintln!("Part must either be 1 or 2");
        return;
    }

    println!("Running day {day} part {part}");

    // build days
    let mut days: [Day; 20] = [Day::new(day_one::part_one, day_one::part_two); 20];
    days[0] = Day::new(day_one::part_one, day_one::part_two);

    // run
    match days.get(usize::try_from(day - 1).unwrap()) {
        Some(d) => {
            d.parts[usize::try_from(part - 1).unwrap()]();
        }
        None => {
            eprintln!("No solution exists for that day.");
        }
    }
}

#[derive(Copy, Clone)]
struct Day {
    parts: [fn(); 2],
}

impl Day {
    fn new(part_one: fn(), part_two: fn()) -> Day {
        Day {
            parts: [part_one, part_two],
        }
    }
}
