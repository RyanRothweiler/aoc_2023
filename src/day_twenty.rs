#![allow(
    unused_variables,
    dead_code,
    unused_mut,
    unused_imports,
    unused_assignments,
    unused_labels
)]

use std::collections::HashMap;
use std::collections::VecDeque;

pub fn run() {}

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
}

struct NodeState {
    //ids of nodes to send pulses to
    children: Vec<String>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct ConjunctionState {
    memory: HashMap<String, PulseKind>,
}

struct BroadcastState {}

struct Node {
    children: Vec<String>,
    kind: NodeKind,
    pulses: VecDeque<Pulse>,
}

fn queue_pulse(pulse: Pulse, nodes: Vec<String>, all_nodes: &mut HashMap<String, Node>) {
    for id in nodes {
        let mut node = all_nodes.get_mut(&id).unwrap();
        node.pulses.push_back(pulse.clone());
    }
}

fn step_node(node_id: &str, all_nodes: &mut HashMap<String, Node>) {
    let node = all_nodes.get_mut(node_id).unwrap();

    let pulse = match node.pulses.pop_front() {
        Some(v) => v,
        None => return,
    };

    match &mut node.kind {
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
            queue_pulse(pulse, node.children.clone(), all_nodes);
        }
    }
}

fn parse_map(file_dir: &str) -> HashMap<String, Node> {
    let contents = std::fs::read_to_string(file_dir).unwrap();

    let mut nodes: HashMap<String, Node> = HashMap::new();

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

        nodes.insert(node_id, node);
    }

    /*
    // do a second pass to init the conjunctions memory
    for (key, value) in &nodes {
        for c in &value.children {
            let child = nodes.get_mut(c).unwrap();
            //child.kind = NodeKind::Broadcast;
        }

        /*
        match &mut value.kind {
            NodeKind::Conjunction(state) => {
                for c in &value.children {
                    state.memory.insert(c.to_string(), PulseKind::Low);
                }
            }
            _ => {
                // do nothing for other types
            }
        }
        */
    }
    */

    return nodes;
}

#[test]
fn parsing() {
    let all_nodes = parse_map("resources/day_20/day_20_sample_one.txt");

    assert_eq!(all_nodes.len(), 5);

    let node = all_nodes.get("broadcaster").unwrap();
    assert_eq!(node.kind, NodeKind::Broadcast);
    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[0], "a".to_string());
    assert_eq!(node.children[1], "b".to_string());
    assert_eq!(node.children[2], "c".to_string());

    let node = all_nodes.get("a").unwrap();
    assert_eq!(node.kind, NodeKind::FlipFlop(false));
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0], "b".to_string());

    let node = all_nodes.get("b").unwrap();
    assert_eq!(node.kind, NodeKind::FlipFlop(false));
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0], "c".to_string());

    let node = all_nodes.get("c").unwrap();
    assert_eq!(node.kind, NodeKind::FlipFlop(false));
    assert_eq!(node.children.len(), 1);
    assert_eq!(node.children[0], "inv".to_string());

    let node = all_nodes.get("inv").unwrap();
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
fn node_flipflop() {
    let mut nodes: HashMap<String, Node> = HashMap::new();

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
    nodes.insert("tst".to_string(), node_a);

    let mut node_b = Node {
        children: vec![],
        kind: NodeKind::FlipFlop(false),
        pulses: VecDeque::new(),
    };
    nodes.insert("b".to_string(), node_b);

    let mut node_c = Node {
        children: vec![],
        kind: NodeKind::FlipFlop(false),
        pulses: VecDeque::new(),
    };
    nodes.insert("c".to_string(), node_c);

    // three pulses to process
    step_node("tst", &mut nodes);
    step_node("tst", &mut nodes);
    step_node("tst", &mut nodes);
    step_node("tst", &mut nodes);
    step_node("tst", &mut nodes);

    // validate
    let bn = nodes.get("b").unwrap();
    assert_eq!(bn.pulses.len(), 3);
    assert_eq!(bn.pulses[0].kind, PulseKind::High);
    assert_eq!(bn.pulses[1].kind, PulseKind::Low);
    assert_eq!(bn.pulses[2].kind, PulseKind::High);

    let cn = nodes.get("c").unwrap();
    assert_eq!(cn.pulses.len(), 3);
    assert_eq!(cn.pulses[0].kind, PulseKind::High);
    assert_eq!(cn.pulses[1].kind, PulseKind::Low);
    assert_eq!(cn.pulses[2].kind, PulseKind::High);
}

#[test]
fn node_conjunction() {
    let mut nodes: HashMap<String, Node> = HashMap::new();

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
    nodes.insert("tst".to_string(), node_a);

    let mut node_b = Node {
        children: vec![],
        kind: NodeKind::FlipFlop(false),
        pulses: VecDeque::new(),
    };
    nodes.insert("b".to_string(), node_b);

    // pulses to process
    step_node("tst", &mut nodes);
    step_node("tst", &mut nodes);
    step_node("tst", &mut nodes);
    step_node("tst", &mut nodes);
    step_node("tst", &mut nodes);

    // validate
    let bn = nodes.get("b").unwrap();
    assert_eq!(bn.pulses.len(), 5);
    assert_eq!(bn.pulses[0].kind, PulseKind::High);
    assert_eq!(bn.pulses[1].kind, PulseKind::High);
    assert_eq!(bn.pulses[2].kind, PulseKind::Low);
    assert_eq!(bn.pulses[3].kind, PulseKind::High);
    assert_eq!(bn.pulses[4].kind, PulseKind::High);
}

#[test]
fn node_broadcast() {
    let mut nodes: HashMap<String, Node> = HashMap::new();

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
    nodes.insert("tst".to_string(), node_a);

    let mut node_b = Node {
        children: vec![],
        kind: NodeKind::FlipFlop(false),
        pulses: VecDeque::new(),
    };
    nodes.insert("b".to_string(), node_b);

    let mut node_c = Node {
        children: vec![],
        kind: NodeKind::FlipFlop(false),
        pulses: VecDeque::new(),
    };
    nodes.insert("c".to_string(), node_c);

    // pulses to process
    step_node("tst", &mut nodes);
    step_node("tst", &mut nodes);

    // validate
    let bn = nodes.get("b").unwrap();
    assert_eq!(bn.pulses.len(), 2);
    assert_eq!(bn.pulses[0].kind, PulseKind::Low);
    assert_eq!(bn.pulses[1].kind, PulseKind::High);

    let cn = nodes.get("c").unwrap();
    assert_eq!(cn.pulses.len(), 2);
    assert_eq!(cn.pulses[0].kind, PulseKind::Low);
    assert_eq!(cn.pulses[1].kind, PulseKind::High);
}
