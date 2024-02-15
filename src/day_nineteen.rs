#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use std::collections::HashMap;
use std::ops::Index;

pub fn run() {
    let v = part_one("resources/inputs/day_19.txt");
    println!("{v}");
}

fn part_one(file_dir: &str) -> i64 {
    let mut file_data = parse_file(file_dir);

    let mut sum: i64 = 0;
    for inst in file_data.parts {
        if filter_run(&inst, "in".to_string(), &mut file_data.groups) == FilterResult::Accept {
            sum += inst.x;
            sum += inst.m;
            sum += inst.a;
            sum += inst.s;
        }
    }

    return sum;
}

struct PartInstance {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl PartInstance {
    // format of
    // {x=787,m=2655,a=1222,s=2876}
    //  TODO should return a result.
    fn from_string(input: &str) -> PartInstance {
        let categories: Vec<&str> = input.split(',').collect();

        let mut ret = PartInstance {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };

        let str: Vec<&str> = categories[0].split('=').collect();
        ret.x = str[1].parse().unwrap();

        let str: Vec<&str> = categories[1].split('=').collect();
        ret.m = str[1].parse().unwrap();

        let str: Vec<&str> = categories[2].split('=').collect();
        ret.a = str[1].parse().unwrap();

        let str: Vec<&str> = categories[3].split('=').collect();
        let mut m_str: String = str[1].to_string();
        m_str.remove(m_str.len() - 1);
        ret.s = m_str.parse().unwrap();

        return ret;
    }

    fn pass_filter(&self, filt: &PartFilterData) -> bool {
        match filt.comp {
            Comparison::Less => return self[filt.category] < filt.val,
            Comparison::Greater => return self[filt.category] > filt.val,
        }
    }
}

impl Index<PartCategory> for PartInstance {
    type Output = i64;

    fn index(&self, input: PartCategory) -> &Self::Output {
        match input {
            PartCategory::X => &self.x,
            PartCategory::M => &self.m,
            PartCategory::A => &self.a,
            PartCategory::S => &self.s,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum PartCategory {
    X,
    M,
    A,
    S,
}

impl PartCategory {
    fn from_char<'a>(input: char) -> Result<PartCategory, &'a str> {
        match input {
            'x' => return Ok(PartCategory::X),
            'm' => return Ok(PartCategory::M),
            'a' => return Ok(PartCategory::A),
            's' => return Ok(PartCategory::S),
            _ => return Err("Invalid chracter for part category {input}."),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Comparison {
    Less,
    Greater,
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum PartFilter {
    Filter(PartFilterData),
    Goto(String),
    Accept,
    Reject,
}

impl PartFilter {
    // returns accept, reject or a goto with destination
    fn parse_dest<'a>(input: &str) -> Result<PartFilter, &'a str> {
        let dest_chars: Vec<char> = input.chars().collect();

        // this means the step is either A for R
        if dest_chars.len() == 1 {
            match dest_chars[0] {
                'A' => return Ok(PartFilter::Accept),
                'R' => return Ok(PartFilter::Reject),
                _ => return Err("Unable to parse the destination value."),
            }
        } else {
            return Ok(PartFilter::Goto(input.to_string()));
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct PartFilterData {
    category: PartCategory,
    comp: Comparison,
    val: i64,
    dest: FilterResult,
}

#[derive(Hash, Eq, PartialEq)]
struct FilterGroup {
    id: String,
    part_filters: Vec<PartFilter>,
}

impl FilterGroup {
    fn from_string(input: &str) -> Result<FilterGroup, &str> {
        let mut ret = FilterGroup {
            id: "".to_string(),
            part_filters: vec![],
        };

        let parts: Vec<&str> = input.trim().split('{').collect();
        if parts.len() != 2 {
            return Err("Invalid format");
        }

        // id
        ret.id = parts[0].to_string();

        // filter steps
        let steps = parts[1].replace("}", "");
        let filts: Vec<&str> = steps.split(',').collect();
        for f in filts {
            let comp_dest: Vec<&str> = f.split(":").collect();

            if comp_dest.len() == 1 {
                // either a destination or an accept/reject
                match PartFilter::parse_dest(comp_dest[0]) {
                    Ok(v) => ret.part_filters.push(v),
                    Err(v) => return Err(v),
                };
            } else if comp_dest.len() == 2 {
                // one filter
                // a<2006
                // qkq

                let chars: Vec<char> = comp_dest[0].chars().collect();
                let mut filter_data = PartFilterData {
                    category: PartCategory::X,
                    comp: Comparison::Less,
                    val: 0,
                    dest: FilterResult::Accept,
                };

                // category
                filter_data.category = PartCategory::from_char(chars[0]).unwrap();

                // comparison
                match chars[1] {
                    '>' => filter_data.comp = Comparison::Greater,
                    '<' => filter_data.comp = Comparison::Less,
                    _ => return Err("Invalid comparison character."),
                }

                // value
                let mut val_str: String = comp_dest[0].to_string();
                val_str.remove(0);
                val_str.remove(0);
                filter_data.val = val_str.parse().unwrap();

                // dest
                match PartFilter::parse_dest(comp_dest[1]) {
                    Ok(v) => {
                        // This conversion here hints at the idea the we don't need both these data
                        // structures. They're basically the same. So we should try compressing
                        // them down into one.
                        match v {
                            PartFilter::Accept => filter_data.dest = FilterResult::Accept,
                            PartFilter::Reject => filter_data.dest = FilterResult::Reject,
                            PartFilter::Goto(data) => filter_data.dest = FilterResult::Goto(data),
                            _ => return Err("Invalid"),
                        }
                    }
                    Err(v) => return Err(v),
                };

                ret.part_filters.push(PartFilter::Filter(filter_data));
            } else {
                return Err("Invalid format for filter group.");
            }
        }

        return Ok(ret);
    }
}

struct FileData {
    groups: HashMap<String, FilterGroup>,
    parts: Vec<PartInstance>,
}

fn parse_file(file_dir: &str) -> FileData {
    let mut ret = FileData {
        groups: HashMap::new(),
        parts: vec![],
    };

    let contents = std::fs::read_to_string(file_dir).unwrap();

    // 0 for groups first then 1 for part instances
    let mut chunk_count = 0;
    for line in contents.lines() {
        if line.trim().len() == 0 {
            // advance to next chunk on empty line
            chunk_count += 1;
        } else {
            if chunk_count == 0 {
                // parse group filter

                let group = FilterGroup::from_string(line).unwrap();
                ret.groups.insert(group.id.clone(), group);
            } else if chunk_count == 1 {
                // parse part instance

                let pi = PartInstance::from_string(line);
                ret.parts.push(pi);
            } else {
                panic!("Error parsing. Found a second empty line");
            }
        }
    }

    return ret;
}

fn build_filter_groups(file_dir: &str) -> HashMap<String, FilterGroup> {
    let mut ret: HashMap<String, FilterGroup> = HashMap::new();

    let contents = std::fs::read_to_string(file_dir).unwrap();
    for l in contents.lines() {
        let group = FilterGroup::from_string(l).unwrap();
        ret.insert(group.id.clone(), group);
    }

    return ret;
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum FilterResult {
    Accept,
    Reject,
    Goto(String),
}

fn filter_run(
    part: &PartInstance,
    filter_id: String,
    all_filters: &mut HashMap<String, FilterGroup>,
) -> FilterResult {
    let filter: &FilterGroup = all_filters.get(&filter_id).unwrap();

    for curr_filt in &filter.part_filters {
        match curr_filt {
            PartFilter::Accept => return FilterResult::Accept,
            PartFilter::Reject => return FilterResult::Reject,
            PartFilter::Goto(dest) => return filter_run(part, dest.clone(), all_filters),
            PartFilter::Filter(filter_data) => {
                if part.pass_filter(filter_data) {
                    match filter_data.dest.clone() {
                        FilterResult::Accept => return FilterResult::Accept,
                        FilterResult::Reject => return FilterResult::Reject,
                        FilterResult::Goto(next_filter_id) => {
                            return filter_run(part, next_filter_id, all_filters)
                        }
                    }
                } else {
                    // part failed this filter, continue onto the next filter in the outer loop.
                    continue;
                }
            }
        }
    }

    panic!("All filters should always end in either accept or reject. We should never iterate through the entire list without returning.");
}

#[test]
fn parse_one() {
    let fg = FilterGroup::from_string("px{a<2006:qkq,m>2090:A,rfg}").unwrap();

    assert_eq!(fg.id, "px".to_string());
    assert_eq!(fg.part_filters.len(), 3);

    assert_eq!(
        fg.part_filters[0],
        PartFilter::Filter(PartFilterData {
            category: PartCategory::A,
            comp: Comparison::Less,
            val: 2006,
            dest: FilterResult::Goto("qkq".to_string()),
        })
    );

    assert_eq!(
        fg.part_filters[1],
        PartFilter::Filter(PartFilterData {
            category: PartCategory::M,
            comp: Comparison::Greater,
            val: 2090,
            dest: FilterResult::Accept,
        })
    );

    assert_eq!(fg.part_filters[2], PartFilter::Goto("rfg".to_string()),);
}

#[test]
fn parse_two() {
    let fg = FilterGroup::from_string("qs{s>3448:A,lnx}").unwrap();

    assert_eq!(fg.id, "qs".to_string());
    assert_eq!(fg.part_filters.len(), 2);

    assert_eq!(
        fg.part_filters[0],
        PartFilter::Filter(PartFilterData {
            category: PartCategory::S,
            comp: Comparison::Greater,
            val: 3448,
            dest: FilterResult::Accept,
        })
    );

    assert_eq!(fg.part_filters[1], PartFilter::Goto("lnx".to_string()));
}

#[test]
fn parse_three() {
    let fg = FilterGroup::from_string("rfg{s<537:gd,x>2440:R,A}").unwrap();

    assert_eq!(fg.id, "rfg".to_string());
    assert_eq!(fg.part_filters.len(), 3);

    assert_eq!(
        fg.part_filters[0],
        PartFilter::Filter(PartFilterData {
            category: PartCategory::S,
            comp: Comparison::Less,
            val: 537,
            dest: FilterResult::Goto("gd".to_string()),
        })
    );

    assert_eq!(
        fg.part_filters[1],
        PartFilter::Filter(PartFilterData {
            category: PartCategory::X,
            comp: Comparison::Greater,
            val: 2440,
            dest: FilterResult::Reject,
        })
    );

    assert_eq!(fg.part_filters[2], PartFilter::Accept);
}

#[test]
fn parse_part_instance() {
    let pi = PartInstance::from_string("{x=787,m=2655,a=1222,s=2876}");
    assert_eq!(pi.x, 787);
    assert_eq!(pi.m, 2655);
    assert_eq!(pi.a, 1222);
    assert_eq!(pi.s, 2876);
}

#[test]
fn sample() {
    let mut file_data = parse_file("resources/day_19/day_19_sample.txt");

    assert_eq!(file_data.groups.len(), 11);
    assert_eq!(file_data.parts.len(), 5);

    assert_eq!(
        filter_run(&file_data.parts[0], "in".to_string(), &mut file_data.groups),
        FilterResult::Accept
    );

    assert_eq!(
        filter_run(&file_data.parts[1], "in".to_string(), &mut file_data.groups),
        FilterResult::Reject
    );

    assert_eq!(
        filter_run(&file_data.parts[2], "in".to_string(), &mut file_data.groups),
        FilterResult::Accept
    );

    assert_eq!(
        filter_run(&file_data.parts[3], "in".to_string(), &mut file_data.groups),
        FilterResult::Reject
    );

    assert_eq!(
        filter_run(&file_data.parts[4], "in".to_string(), &mut file_data.groups),
        FilterResult::Accept
    );
}

#[test]
fn part_one_test() {
    let v = part_one("resources/day_19/day_19_sample.txt");
    assert_eq!(v, 19114);
}
