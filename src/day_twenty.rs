#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use core::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;

const BROADCASTER_ID: &str = "broadcaster";

pub fn run() {
    let v = part_one("resources/inputs/day_20.txt", 1000);
    println!("{v}");
}

fn part_one(file_dir: &str, press_count: i64) -> i64 {
    let mut all_nodes = parse_map(file_dir);

    let mut accum = Accumulator::new();
    for i in 0..press_count {
        press_button(&mut all_nodes, &mut accum);
    }

    return accum.low_count * accum.high_count;
}

fn press_button(all_nodes: &mut HashMap<String, RefCell<Node>>, accum: &mut Accumulator) {
    // send first pulse on broadcaster
    {
        queue_pulse(
            Pulse::new(PulseKind::Low, "button".to_string()),
            vec![BROADCASTER_ID.to_string()],
            all_nodes,
        );
    }

    // process all queues, until they're all empty
    loop {
        let mut did_process = false;

        for key in all_nodes.keys() {
            let mut step = false;

            // block to unborrow the node after checking its pulse count
            {
                let node = all_nodes.get(key).unwrap().borrow();
                step = node.pulses.len() != 0;
            }

            if step {
                did_process = true;
                step_node(key, all_nodes, accum);
            }
        }

        if !did_process {
            break;
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum PulseKind {
    High,
    Low,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Pulse {
    kind: PulseKind,
    source_id: String,
}

impl Pulse {
    fn new(kind: PulseKind, source_id: String) -> Pulse {
        Pulse { kind, source_id }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum NodeKind {
    FlipFlop(bool),
    Conjunction(ConjunctionState),
    Broadcast,
    Empty,
}

struct NodeState {
    //ids of nodes to send pulses to
    children: Vec<String>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct ConjunctionState {
    memory: HashMap<String, PulseKind>,
}

struct Node {
    children: Vec<String>,
    kind: NodeKind,
    pulses: VecDeque<Pulse>,
}

struct Accumulator {
    low_count: i64,
    high_count: i64,
}

impl Accumulator {
    fn new() -> Accumulator {
        Accumulator {
            low_count: 0,
            high_count: 0,
        }
    }
}

fn queue_pulse(pulse: Pulse, nodes: Vec<String>, all_nodes: &HashMap<String, RefCell<Node>>) {
    for id in nodes {
        let mut node = all_nodes.get(&id).unwrap().borrow_mut();
        node.pulses.push_back(pulse.clone());
    }
}

fn step_node(node_id: &str, all_nodes: &HashMap<String, RefCell<Node>>, accum: &mut Accumulator) {
    let mut node = all_nodes.get(node_id).unwrap().borrow_mut();

    let pulse = match node.pulses.pop_front() {
        Some(v) => v,
        None => return,
    };

    //println!("{:?} -{:?}-> {:?}", pulse.source_id, pulse.kind, node_id);

    // count pulses
    match pulse.kind {
        PulseKind::Low => accum.low_count += 1,
        PulseKind::High => accum.high_count += 1,
    }

    // sim the pulse
    match &mut node.kind {
        NodeKind::Empty => {
            // Do nothing for empty nodes.
        }

        NodeKind::FlipFlop(state) => {
            match pulse.kind {
                PulseKind::High => {
                    // Nothing happens on FlipFlops for high pulse
                    return;
                }
                PulseKind::Low => {
                    let p_state = *state;
                    *state = !(*state);

                    if p_state {
                        queue_pulse(
                            Pulse::new(PulseKind::Low, node_id.to_string()),
                            node.children.clone(),
                            all_nodes,
                        );
                    } else {
                        queue_pulse(
                            Pulse::new(PulseKind::High, node_id.to_string()),
                            node.children.clone(),
                            all_nodes,
                        );
                    }
                }
            }
        }

        NodeKind::Conjunction(state) => {
            // update memory
            if !state.memory.contains_key(&pulse.source_id) {
                state
                    .memory
                    .insert(pulse.source_id.to_string(), PulseKind::Low);
            }

            let mem = state.memory.get_mut(&pulse.source_id).unwrap();
            *mem = pulse.kind;

            // send a pulses
            let mut all_high = true;
            for (key, value) in &state.memory {
                if *value == PulseKind::Low {
                    all_high = false;
                    break;
                }
            }

            if all_high {
                queue_pulse(
                    Pulse::new(PulseKind::Low, node_id.to_string()),
                    node.children.clone(),
                    all_nodes,
                );
            } else {
                queue_pulse(
                    Pulse::new(PulseKind::High, node_id.to_string()),
                    node.children.clone(),
                    all_nodes,
                );
            }
        }

        NodeKind::Broadcast => {
            queue_pulse(
                Pulse::new(pulse.kind, node_id.to_string()),
                node.children.clone(),
                all_nodes,
            );
        }
    }
}

fn parse_map(file_dir: &str) -> HashMap<String, RefCell<Node>> {
    let contents = std::fs::read_to_string(file_dir).unwrap();

    let mut nodes: HashMap<String, RefCell<Node>> = HashMap::new();

    // create all the nodes
    for line in contents.lines() {
        // 0 is the node type
        // 1 is the arrow
        // 2..n is the node children
        let parts: Vec<&str> = line.split(' ').collect();

        // create node
        let mut node = Node {
            children: vec![],
            kind: NodeKind::Broadcast,
            pulses: VecDeque::new(),
        };
        let mut node_id: String = String::new();

        let first_chars: Vec<char> = parts[0].trim().chars().collect();
        match first_chars[0] {
            'b' => {
                node.kind = NodeKind::Broadcast;
                node_id = "broadcaster".to_string();
            }
            '%' => {
                node.kind = NodeKind::FlipFlop(false);
                node_id = parts[0].trim().to_string();
                node_id.remove(0);
            }
            '&' => {
                node.kind = NodeKind::Conjunction(ConjunctionState {
                    memory: HashMap::new(),
                });
                node_id = parts[0].trim().to_string();
                node_id.remove(0);
            }
            _ => panic!("Unknown node type"),
        }

        // setup node children
        for i in 2..parts.len() {
            let mut id = parts[i].trim().to_string();
            id = id.replace(",", "");
            node.children.push(id);
        }

        nodes.insert(node_id, RefCell::new(node));
    }

    // Do a second pass to init the conjunctions memory
    // This will mut borrow twice if a node is its own parent.
    // So don't do that.
    // This also creates empty nodes if the child doesn't exist.
    let mut empties: Vec<String> = vec![];
    for (key, value) in &nodes {
        let node = value.borrow();
        for child_id in &node.children {
            match nodes.get(child_id) {
                Some(child_cell) => {
                    let mut child = child_cell.borrow_mut();

                    match &mut child.kind {
                        NodeKind::Conjunction(state) => {
                            state.memory.insert(key.to_string(), PulseKind::Low);
                        }
                        _ => {
                            // do nothing for other types
                        }
                    }
                }
                None => {
                    empties.push(child_id.clone());
                }
            }
        }
    }

    // Create an empty nodes
    for id in empties {
        let mut n = Node {
            children: vec![],
            kind: NodeKind::Empty,
            pulses: VecDeque::new(),
        };

        nodes.insert(id.clone(), RefCell::new(n));
    }

    return nodes;
}

#[test]
fn parsing_one() {
    let all_nodes = parse_map("resources/day_20/day_20_sample_one.txt");

    assert_eq!(all_nodes.len(), 5);

    let node = all_nodes.get("broadcaster").unwrap().borrow();
    assert_eq!(node.kind, NodeKind::Broadcast);
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[0], "a".to_string());
    assert_eq!(node.children[1], "b".to_string());
    assert_eq!(node.children[2], "c".to_string());

    let node = all_nodes.get("a").unwrap().borrow();
    assert_eq!(node.kind, NodeKind::FlipFlop(false));
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0], "b".to_string());

    let node = all_nodes.get("b").unwrap().borrow();
    assert_eq!(node.kind, NodeKind::FlipFlop(false));
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0], "c".to_string());

    let node = all_nodes.get("c").unwrap().borrow();
    assert_eq!(node.kind, NodeKind::FlipFlop(false));
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0], "inv".to_string());

    let node = all_nodes.get("inv").unwrap().borrow();
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0], "a".to_string());
    match &node.kind {
        NodeKind::Conjunction(state) => {
            assert_eq!(state.memory.len(), 1);
            assert_eq!(state.memory.contains_key("c"), true);
        }
        _ => panic!("Wrong node type."),
    }
}

#[test]
fn parsing_two() {
    let all_nodes = parse_map("resources/day_20/day_20_sample_two.txt");

    assert_eq!(all_nodes.len(), 6);

    let node = all_nodes.get("broadcaster").unwrap().borrow();
    assert_eq!(node.kind, NodeKind::Broadcast);
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0], "a".to_string());

    let node = all_nodes.get("a").unwrap().borrow();
    assert_eq!(node.kind, NodeKind::FlipFlop(false));
    assert_eq!(node.children.len(), 2);
    assert_eq!(node.children[0], "inv".to_string());
    assert_eq!(node.children[1], "con".to_string());

    let node = all_nodes.get("inv").unwrap().borrow();
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0], "b".to_string());
    match &node.kind {
        NodeKind::Conjunction(state) => {
            assert_eq!(state.memory.len(), 1);
            assert_eq!(state.memory.contains_key("a"), true);
        }
        _ => panic!("Wrong node type."),
    }

    let node = all_nodes.get("b").unwrap().borrow();
    assert_eq!(node.kind, NodeKind::FlipFlop(false));
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0], "con".to_string());

    let node = all_nodes.get("con").unwrap().borrow();
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0], "output".to_string());
    match &node.kind {
        NodeKind::Conjunction(state) => {
            assert_eq!(state.memory.len(), 2);
            assert_eq!(state.memory.contains_key("b"), true);
            assert_eq!(state.memory.contains_key("a"), true);
        }
        _ => panic!("Wrong node type."),
    }

    let node = all_nodes.get("output").unwrap().borrow();
    assert_eq!(node.kind, NodeKind::Empty);
    assert_eq!(node.children.len(), 0);
}

#[test]
fn node_flipflop() {
    let mut nodes: HashMap<String, RefCell<Node>> = HashMap::new();

    let mut node_a = Node {
        children: vec!["b".to_string(), "c".to_string()],
        kind: NodeKind::FlipFlop(false),
        pulses: VecDeque::new(),
    };
    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::Low, "b".to_string()));
    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::High, "b".to_string()));
    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::Low, "b".to_string()));
    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::High, "b".to_string()));
    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::Low, "b".to_string()));
    nodes.insert("tst".to_string(), RefCell::new(node_a));

    let mut node_b = Node {
        children: vec![],
        kind: NodeKind::FlipFlop(false),
        pulses: VecDeque::new(),
    };
    nodes.insert("b".to_string(), RefCell::new(node_b));

    let mut node_c = Node {
        children: vec![],
        kind: NodeKind::FlipFlop(false),
        pulses: VecDeque::new(),
    };
    nodes.insert("c".to_string(), RefCell::new(node_c));

    // three pulses to process
    let mut accum = Accumulator::new();
    step_node("tst", &mut nodes, &mut accum);
    step_node("tst", &mut nodes, &mut accum);
    step_node("tst", &mut nodes, &mut accum);
    step_node("tst", &mut nodes, &mut accum);
    step_node("tst", &mut nodes, &mut accum);

    // validate
    let bn = nodes.get("b").unwrap().borrow();
    assert_eq!(bn.pulses.len(), 3);
    assert_eq!(bn.pulses[0].kind, PulseKind::High);
    assert_eq!(bn.pulses[1].kind, PulseKind::Low);
    assert_eq!(bn.pulses[2].kind, PulseKind::High);

    let cn = nodes.get("c").unwrap().borrow();
    assert_eq!(cn.pulses.len(), 3);
    assert_eq!(cn.pulses[0].kind, PulseKind::High);
    assert_eq!(cn.pulses[1].kind, PulseKind::Low);
    assert_eq!(cn.pulses[2].kind, PulseKind::High);
}

#[test]
fn node_conjunction() {
    let mut nodes: HashMap<String, RefCell<Node>> = HashMap::new();

    let mut memory: HashMap<String, PulseKind> = HashMap::new();
    memory.insert("first".to_string(), PulseKind::Low);
    memory.insert("second".to_string(), PulseKind::High);

    let mut node_a = Node {
        children: vec!["b".to_string()],
        kind: NodeKind::Conjunction(ConjunctionState { memory: memory }),
        pulses: VecDeque::new(),
    };

    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::Low, "first".to_string()));
    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::High, "second".to_string()));
    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::High, "first".to_string()));
    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::Low, "first".to_string()));
    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::Low, "second".to_string()));
    nodes.insert("tst".to_string(), RefCell::new(node_a));

    let mut node_b = Node {
        children: vec![],
        kind: NodeKind::FlipFlop(false),
        pulses: VecDeque::new(),
    };
    nodes.insert("b".to_string(), RefCell::new(node_b));

    // pulses to process
    let mut accum = Accumulator::new();
    step_node("tst", &mut nodes, &mut accum);
    step_node("tst", &mut nodes, &mut accum);
    step_node("tst", &mut nodes, &mut accum);
    step_node("tst", &mut nodes, &mut accum);
    step_node("tst", &mut nodes, &mut accum);

    // validate
    let bn = nodes.get("b").unwrap().borrow();
    assert_eq!(bn.pulses.len(), 5);
    assert_eq!(bn.pulses[0].kind, PulseKind::High);
    assert_eq!(bn.pulses[1].kind, PulseKind::High);
    assert_eq!(bn.pulses[2].kind, PulseKind::Low);
    assert_eq!(bn.pulses[3].kind, PulseKind::High);
    assert_eq!(bn.pulses[4].kind, PulseKind::High);
}

#[test]
fn node_broadcast() {
    let mut nodes: HashMap<String, RefCell<Node>> = HashMap::new();

    let mut node_a = Node {
        children: vec!["b".to_string(), "c".to_string()],
        kind: NodeKind::Broadcast,
        pulses: VecDeque::new(),
    };

    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::Low, "first".to_string()));
    node_a
        .pulses
        .push_back(Pulse::new(PulseKind::High, "second".to_string()));
    nodes.insert("tst".to_string(), RefCell::new(node_a));

    let mut node_b = Node {
        children: vec![],
        kind: NodeKind::FlipFlop(false),
        pulses: VecDeque::new(),
    };
    nodes.insert("b".to_string(), RefCell::new(node_b));

    let mut node_c = Node {
        children: vec![],
        kind: NodeKind::FlipFlop(false),
        pulses: VecDeque::new(),
    };
    nodes.insert("c".to_string(), RefCell::new(node_c));

    // pulses to process
    let mut accum = Accumulator::new();
    step_node("tst", &mut nodes, &mut accum);
    step_node("tst", &mut nodes, &mut accum);

    // validate
    let bn = nodes.get("b").unwrap().borrow();
    assert_eq!(bn.pulses.len(), 2);
    assert_eq!(bn.pulses[0].kind, PulseKind::Low);
    assert_eq!(bn.pulses[1].kind, PulseKind::High);

    let cn = nodes.get("c").unwrap().borrow();
    assert_eq!(cn.pulses.len(), 2);
    assert_eq!(cn.pulses[0].kind, PulseKind::Low);
    assert_eq!(cn.pulses[1].kind, PulseKind::High);
}

#[test]
fn button_single_sample_one() {
    let mut all_nodes = parse_map("resources/day_20/day_20_sample_one.txt");
    let mut accum = Accumulator::new();
    press_button(&mut all_nodes, &mut accum);

    assert_eq!(accum.low_count, 8);
    assert_eq!(accum.high_count, 4);
}

#[test]
fn part_one_sample_one() {
    let v = part_one("resources/day_20/day_20_sample_one.txt", 1000);
    assert_eq!(v, 32_000_000);
}

#[test]
fn button_single_sample_two() {
    let mut all_nodes = parse_map("resources/day_20/day_20_sample_two.txt");
    let mut accum = Accumulator::new();
    press_button(&mut all_nodes, &mut accum);

    assert_eq!(accum.low_count, 4);
    assert_eq!(accum.high_count, 4);
}

/*
#[test]
fn part_one_sample_two() {
    let v = part_one("resources/day_20/day_20_sample_two.txt", 1000);
    assert_eq!(v, 11_687_500);
}
*/
