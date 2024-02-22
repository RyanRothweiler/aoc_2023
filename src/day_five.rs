#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

mod farm_type;
mod mapping;

use mapping::Mapping;

pub fn run() {
    part_one();
}

// get lowest location from seeds
fn part_one() -> i64 {
    let mut all_mappings = FullMapping {
        seed_to_soil: vec![],
        soil_to_fertalizer: vec![],
        fertalizer_to_water: vec![],
        water_to_light: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        humidity_to_location: vec![],
    };

    let contents: String =
        std::fs::read_to_string("resources/inputs/day_5.txt").expect("Invalid file source");

    let lines: Vec<&str> = contents.split('\n').collect();

    // get mappings
    let mappings = FullMapping::parse(&contents).expect("Invalid file format.");

    // get seeds
    let mut seeds: Vec<i64> = vec![];
    let seed_line: Vec<&str> = lines[0].split(' ').collect();
    // 1 becuase first entry is 'seed:'
    for i in 1..seed_line.len() {
        let n: i64 = seed_line[i].trim().parse().unwrap_or_else(|error| 0);
        seeds.push(n);
    }

    // this assumes no seed of value -1
    let mut nearest_loc = -1;

    for seed in seeds {
        let loc = mappings.seed_to_location(seed);
        if nearest_loc == -1 || loc < nearest_loc {
            nearest_loc = loc;
        }
    }

    println!("{nearest_loc}");
    return nearest_loc;
}

// treat seeds as ranges, and find the smallest location
// uses brute force. Could be improved by just transforming the ranges
fn part_two() -> i64 {
    let mut all_mappings = FullMapping {
        seed_to_soil: vec![],
        soil_to_fertalizer: vec![],
        fertalizer_to_water: vec![],
        water_to_light: vec![],
        light_to_temperature: vec![],
        temperature_to_humidity: vec![],
        humidity_to_location: vec![],
    };

    let contents: String =
        std::fs::read_to_string("resources/inputs/day_5.txt").expect("Invalid file source");

    let lines: Vec<&str> = contents.split('\n').collect();

    // get mappings
    let mappings = FullMapping::parse(&contents).expect("Invalid file format.");

    // get seeds
    let mut seeds: Vec<SeedRange> = vec![];
    let seed_line: Vec<&str> = lines[0].split(' ').collect();

    // 1 becuase first entry is 'seed:'
    let mut ind: usize = 1;
    for i in 1..seed_line.len() {
        if ind + 1 < seed_line.len() {
            let mut rn = SeedRange {
                start: 0,
                length: 0,
            };
            rn.start = seed_line[ind].trim().parse().unwrap_or_else(|error| 0);
            rn.length = seed_line[ind + 1].trim().parse().unwrap_or_else(|error| 0);

            ind = ind + 2;

            seeds.push(rn);
        }
    }

    // this assumes no seed of value -1
    let mut nearest_loc = -1;
    for seed in seeds {
        println!("{seed:?}");

        for i in seed.start..seed.start + seed.length {
            let loc = mappings.seed_to_location(i);
            if nearest_loc == -1 || loc < nearest_loc {
                nearest_loc = loc;
            }
        }
    }

    println!("{nearest_loc}");
    return nearest_loc;
}

#[derive(Debug)]
struct SeedRange {
    start: i64,
    length: i64,
}

// a kind of tokenizer
struct Lineizer<'a> {
    lines: Vec<&'a str>,
    current_line: usize,
}

impl<'a> Lineizer<'a> {
    fn advance(&mut self) {
        self.current_line = self.current_line + 1;
    }

    fn valid(&self) -> bool {
        return self.current_line < self.lines.len();
    }
}

struct FullMapping {
    seed_to_soil: Vec<Mapping>,
    soil_to_fertalizer: Vec<Mapping>,
    fertalizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl FullMapping {
    fn seed_to_location(&self, seed: i64) -> i64 {
        let mut val = seed;

        //println!("seed {val}");
        for c in &self.seed_to_soil {
            if c.within_range(val) {
                val = c.convert(val);
                break;
            }
        }
        //println!("soil {val}");
        for c in &self.soil_to_fertalizer {
            if c.within_range(val) {
                val = c.convert(val);
                break;
            }
        }
        //println!("fertalizer {val}");
        for c in &self.fertalizer_to_water {
            if c.within_range(val) {
                val = c.convert(val);
                break;
            }
        }
        //println!("water {val}");
        for c in &self.water_to_light {
            if c.within_range(val) {
                val = c.convert(val);
                break;
            }
        }
        //println!("light {val}");
        for c in &self.light_to_temperature {
            if c.within_range(val) {
                val = c.convert(val);
                break;
            }
        }
        //println!("temperature {val}");
        for c in &self.temperature_to_humidity {
            if c.within_range(val) {
                val = c.convert(val);
                break;
            }
        }
        //println!("humidity {val}");
        for c in &self.humidity_to_location {
            if c.within_range(val) {
                val = c.convert(val);
                break;
            }
        }

        val
    }

    fn parse(input: &str) -> Option<FullMapping> {
        let mut all_mappings = FullMapping {
            seed_to_soil: vec![],
            soil_to_fertalizer: vec![],
            fertalizer_to_water: vec![],
            water_to_light: vec![],
            light_to_temperature: vec![],
            temperature_to_humidity: vec![],
            humidity_to_location: vec![],
        };

        let mut lineizer = Lineizer {
            lines: input.split('\n').collect(),
            current_line: 0,
        };

        // seeds input
        lineizer.advance();

        // empty line
        lineizer.advance();

        // seeds-to-soil map
        lineizer.advance();
        all_mappings.seed_to_soil = Mapping::parse_list(&mut lineizer);
        lineizer.advance();

        // soil-to-fertalizer
        lineizer.advance();
        all_mappings.soil_to_fertalizer = Mapping::parse_list(&mut lineizer);
        lineizer.advance();

        // fertalizer_to_water
        lineizer.advance();
        all_mappings.fertalizer_to_water = Mapping::parse_list(&mut lineizer);
        lineizer.advance();

        // water-to-light
        lineizer.advance();
        all_mappings.water_to_light = Mapping::parse_list(&mut lineizer);
        lineizer.advance();

        // light-to-temperature
        lineizer.advance();
        all_mappings.light_to_temperature = Mapping::parse_list(&mut lineizer);
        lineizer.advance();

        // temperatur-to-humidity
        lineizer.advance();
        all_mappings.temperature_to_humidity = Mapping::parse_list(&mut lineizer);
        lineizer.advance();

        // humidity-to-location
        lineizer.advance();
        all_mappings.humidity_to_location = Mapping::parse_list(&mut lineizer);
        lineizer.advance();

        return Some(all_mappings);
    }
}

#[test]
fn mappings_parsing() {
    let contents: String =
        std::fs::read_to_string("resources/day_5/day_5_sample.txt").expect("Unable to read file.");
    let mappings = FullMapping::parse(&contents).expect("Invalid file format.");

    assert_eq!(mappings.light_to_temperature.len(), 3);
    assert_eq!(mappings.light_to_temperature[0].convert(77), 45);
    assert_eq!(mappings.light_to_temperature[1].convert(46), 82);
    assert_eq!(mappings.light_to_temperature[2].convert(66), 70);
}

#[test]
fn mapping_list() {
    let mut maps: Vec<Mapping> = vec![];
    maps.push(Mapping::new(50, 98, 2));
    maps.push(Mapping::new(52, 50, 48));

    let mut val: i64;

    val = 79;
    for m in &maps {
        val = m.convert(val);
    }
    assert_eq!(val, 81);

    val = 14;
    for m in &maps {
        val = m.convert(val);
    }
    assert_eq!(val, 14);

    val = 55;
    for m in &maps {
        val = m.convert(val);
    }
    assert_eq!(val, 57);

    val = 13;
    for m in &maps {
        val = m.convert(val);
    }
    assert_eq!(val, 13);
}

#[test]
fn mapping_list_water() {
    let mut maps: Vec<Mapping> = vec![];
    maps.push(Mapping::new(18, 25, 70));
    maps.push(Mapping::new(88, 18, 7));

    let mut val: i64 = 53;
    for m in &maps {
        val = m.convert(val);
    }
    assert_eq!(val, 46);
}

#[test]
fn mapping_list_fertalizer() {
    let mut maps: Vec<Mapping> = vec![];
    maps.push(Mapping::new(49, 53, 8));
    maps.push(Mapping::new(0, 11, 42));
    maps.push(Mapping::new(42, 0, 7));
    maps.push(Mapping::new(57, 7, 4));
    maps.reverse();

    let mut val: i64 = 53;
    for m in &maps {
        val = m.convert(val);
    }
    assert_eq!(val, 49);
}

#[test]
fn full_sample_conversions() {
    let contents: String =
        std::fs::read_to_string("resources/day_5/day_5_sample.txt").expect("Unable to read file.");
    let mappings = FullMapping::parse(&contents).expect("Invalid file format.");

    assert_eq!(mappings.seed_to_location(79), 82);
    assert_eq!(mappings.seed_to_location(14), 43);
    assert_eq!(mappings.seed_to_location(55), 86);
    assert_eq!(mappings.seed_to_location(13), 35);
}

#[test]
fn answer_part_one() {
    let val = part_one();
    assert_eq!(val, 600279879);
}
