#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

const MIRROR_BOX_COUNT: i64 = 256;

use std::collections::HashMap;

pub fn part_one() {
    let contents = std::fs::read_to_string("resources/inputs/day_15.txt").unwrap();

    let entries: Vec<&str> = contents.split(',').collect();
    let mut sum: i64 = 0;
    for e in entries {
        sum += hash(e);
    }
    println!("{sum}");
}

pub fn part_two() {
    let contents = std::fs::read_to_string("resources/inputs/day_15.txt").unwrap();
    let instructions = Instruction::parse_string(&contents);
    let v = align_mirrors_calc(&instructions);
    println!("{v}");
}

#[derive(Eq, PartialEq, Debug)]
enum InstructionType {
    Remove,
    Add(i64),
}

#[derive(Eq, PartialEq, Debug)]
struct Instruction {
    id: String,
    box_hash: i64,
    instruction: InstructionType,
}

impl Instruction {
    // this only accepts formats of single digit lenses. Double digit won't work.
    fn new(inst: &str) -> Self {
        let mut inst_type = InstructionType::Remove;
        let mut found_type = false;

        let chars: Vec<char> = inst.chars().collect();
        let mut id: String = String::new();

        // pull instruction
        for i in 0..chars.len() {
            let c = chars[i];
            if c == '-' {
                found_type = true;
                inst_type = InstructionType::Remove;
                break;
            } else if c == '=' {
                found_type = true;
                let lense: i64 = chars[i + 1].to_digit(10).unwrap() as i64;
                inst_type = InstructionType::Add(lense);
            } else {
                if c.is_alphabetic() {
                    id.push(c);
                }
            }
        }
        if !found_type {
            panic!("Invalid instruction format");
        }

        Self {
            id: id.clone(),
            box_hash: hash(&id),
            instruction: inst_type,
        }
    }

    fn parse_string(input: &str) -> Vec<Self> {
        let mut ret: Vec<Self> = vec![];

        let entries: Vec<&str> = input.split(',').collect();
        for e in entries {
            ret.push(Self::new(e));
        }

        return ret;
    }
}

#[derive(Debug)]
struct Lense {
    id: String,
    focal_length: i64,
}

#[derive(Debug)]
struct Mirror {
    lenses: Vec<Lense>,
}

fn align_mirrors_calc(instructions: &Vec<Instruction>) -> i64 {
    let mut mirrors: HashMap<i64, Mirror> = HashMap::new();

    // go through all the steps
    for i in instructions {
        match i.instruction {
            InstructionType::Remove => {
                if mirrors.contains_key(&i.box_hash) {
                    let mir: &mut Mirror = mirrors.get_mut(&i.box_hash).unwrap();

                    match mir.lenses.iter().position(|x| x.id == i.id) {
                        Some(v) => {
                            mir.lenses.remove(v);
                        }
                        None => {}
                    };
                }
            }
            InstructionType::Add(focal_adding) => {
                if !mirrors.contains_key(&i.box_hash) {
                    let mut mr = Mirror { lenses: vec![] };
                    mr.lenses.push(Lense {
                        id: i.id.clone(),
                        focal_length: focal_adding,
                    });
                    mirrors.insert(i.box_hash, mr);
                } else {
                    let mut mir = mirrors.get_mut(&i.box_hash).unwrap();

                    match mir.lenses.iter().position(|x| x.id == i.id) {
                        Some(v) => {
                            // that lense is in the box, so replace it with the new one.
                            // (update the focal length)
                            mir.lenses[v].focal_length = focal_adding;
                        }
                        None => {
                            // that lense isn't in the box yet, so insert it
                            mir.lenses.push(Lense {
                                id: i.id.clone(),
                                focal_length: focal_adding,
                            });
                        }
                    };
                }
            }
        }
    }

    // do final calculations
    let mut sum: i64 = 0;
    for (key, value) in mirrors {
        for i in 0..value.lenses.len() {
            sum += (key + 1) * (i as i64 + 1) * value.lenses[i].focal_length;
        }
    }

    return sum;
}

fn hash(input: &str) -> i64 {
    let mut v: i64 = 0;

    for c in input.bytes() {
        v += c as i64;
        v = v * 17;
        v = v % MIRROR_BOX_COUNT;
    }

    return v;
}

#[test]
fn hash_test() {
    assert_eq!(hash("rn=1"), 30);
    assert_eq!(hash("cm-"), 253);
    assert_eq!(hash("qp=3"), 97);
    assert_eq!(hash("rn"), 0);
}

#[test]
fn build_instruction_eq() {
    let str = "rn=1";
    let correct = Instruction {
        id: "rn".to_string(),
        box_hash: hash("rn"),
        instruction: InstructionType::Add(1),
    };

    assert_eq!(Instruction::new(str), correct);
}

#[test]
fn build_instruction_remove() {
    let str = "cm-";
    let correct = Instruction {
        id: "cm".to_string(),
        box_hash: hash("cm"),
        instruction: InstructionType::Remove,
    };

    assert_eq!(Instruction::new(str), correct);
}

#[test]
fn part_two_sample() {
    let instructions =
        Instruction::parse_string("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
    assert_eq!(align_mirrors_calc(&instructions), 145);
}

#[test]
fn part_one_sample() {
    let contents = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let entries: Vec<&str> = contents.split(',').collect();
    let mut sum: i64 = 0;
    for e in entries {
        sum += hash(e);
    }
    assert_eq!(sum, 1320);
}
