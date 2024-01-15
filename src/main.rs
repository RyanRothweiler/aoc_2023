use std::fs;

#[allow(unused_variables, unused_assignments, unused_mut)]
fn main() {
    let contents: String =
        fs::read_to_string("resources/day1_input.txt").expect("Could not find the file.");

    let mut sum: u32 = 0;

    let mut found_first = false;
    let mut found_last = false;
    let mut first_num: u32 = 0;
    let mut last_num: u32 = 0;

    for c in contents.chars() {
        if c == '\n' {

            let mut new = 0;
            if found_last {
                new = (first_num * 10) + last_num;
            } else {
                new = (first_num * 10) + first_num;
            }

            sum = sum + new;

            println!("{first_num}");
            println!("{last_num}");
            println!("break!");

            // reset data for next line
            found_first = false;
            found_last = false;
            first_num = 0;
            last_num = 0;
        }

        let v = c.to_digit(10);
        match v {
            Some(val) => {
                if !found_first {
                    found_first = true;
                    first_num = val;
                } else {
                    found_last = true;
                    last_num = val;
                }
            }
            None => {}
        }
        println!("{sum}");
    }
}
