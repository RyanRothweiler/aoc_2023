#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

pub fn run() {
    part_one();
}

pub fn part_one() -> usize {
    let first_track = Track {
        time: 44,
        record: 208,
    };
    let second_track = Track {
        time: 80,
        record: 1580,
    };
    let third_track = Track {
        time: 65,
        record: 1050,
    };
    let fourth_track = Track {
        time: 72,
        record: 1102,
    };

    return first_track.get_winning_options().len()
        * second_track.get_winning_options().len()
        * third_track.get_winning_options().len()
        * fourth_track.get_winning_options().len();
}

struct Track {
    time: u64,
    record: u64,
}

impl Track {
    fn get_dist_moved(&self, time_charged: u64) -> u64 {
        // no charge no movement
        if time_charged <= 0 {
            return 0;
        }

        // hold charge for the entire time, then it never moves
        if time_charged >= self.time {
            return 0;
        }

        let move_time: u64 = self.time - time_charged;
        return time_charged * move_time;
    }

    fn get_winning_options(&self) -> Vec<u64> {
        let mut ret: Vec<u64> = vec![];

        for i in 0..self.time {
            let v = self.get_dist_moved(i);
            if v > self.record {
                ret.push(i);
            }
        }

        let c = ret.len();
        println!("{c}");

        ret
    }

    fn mult_accum_winning(&self) -> u64 {
        let winning = self.get_winning_options();
        let mut accum: u64 = 1;
        for w in winning {
            println!("{w} {accum}");
            accum = accum * w;
        }
        accum
    }
}

#[test]
fn charge_moves() {
    let tr = Track { time: 7, record: 0 };

    assert_eq!(tr.get_dist_moved(0), 0);
    assert_eq!(tr.get_dist_moved(1), 6);
    assert_eq!(tr.get_dist_moved(2), 10);
    assert_eq!(tr.get_dist_moved(3), 12);
    assert_eq!(tr.get_dist_moved(4), 12);
    assert_eq!(tr.get_dist_moved(5), 10);
    assert_eq!(tr.get_dist_moved(6), 6);
    assert_eq!(tr.get_dist_moved(7), 0);
}

#[test]
fn winning_options() {
    let tr = Track { time: 7, record: 9 };
    let opts = tr.get_winning_options();

    assert_eq!(opts.len(), 4);
    assert_eq!(opts[0], 2);
    assert_eq!(opts[1], 3);
    assert_eq!(opts[2], 4);
    assert_eq!(opts[3], 5);
}

#[test]
fn part_one_answer() {
    assert_eq!(part_one(), 32076);
}
