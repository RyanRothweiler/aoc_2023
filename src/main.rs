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
    days[1] = Day::new(day_two::part_one, day_two::part_two);
    days[2] = Day::new(day_three::part_one, day_three::part_two);
    days[3] = Day::new(day_four::part_one, day_four::part_two);
    days[4] = Day::new(day_five::part_one, day_five::part_two);
    days[5] = Day::new(day_six::part_one, day_six::part_two);
    days[6] = Day::new(day_seven::part_one, day_seven::part_two);
    days[7] = Day::new(day_eight::part_one, day_eight::part_two);
    days[8] = Day::new(day_nine::part_one, day_nine::part_two);
    days[9] = Day::new(day_ten::part_one, day_ten::part_two);
    days[10] = Day::new(day_eleven::part_one, day_eleven::part_two);
    days[11] = Day::new(day_twelve::part_one, day_twelve::part_two);
    days[12] = Day::new(day_thirteen::part_one, day_thirteen::part_two);
    days[13] = Day::new(day_fourteen::part_one, day_fourteen::part_two);
    days[14] = Day::new(day_fifteen::part_one, day_fifteen::part_two);

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
