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
enum Pulse {
    High,
    Low,
}

enum NodeKind {
    FlipFlop(bool),
    Conjunction(ConjunctionState),
    Broadcast,
}

struct NodeState {
    //ids of nodes to send pulses to
    children: Vec<String>,
}

struct ConjunctionState {
    states: Vec<bool>,
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
        node.pulses.push_back(pulse);
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
            match pulse {
                Pulse::High => {
                    // Nothing happens on FlipFlops for high pulse
                    return;
                }
                Pulse::Low => {
                    let p_state = *state;
                    *state = !(*state);

                    if p_state {
                        queue_pulse(Pulse::Low, node.children.clone(), all_nodes);
                    } else {
                        queue_pulse(Pulse::High, node.children.clone(), all_nodes);
                    }
                }
            }
        }

        NodeKind::Conjunction(state) => {
            todo!();
        }

        NodeKind::Broadcast => {
            todo!();
        }
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
    node_a.pulses.push_back(Pulse::Low);
    node_a.pulses.push_back(Pulse::High);
    node_a.pulses.push_back(Pulse::Low);
    node_a.pulses.push_back(Pulse::High);
    node_a.pulses.push_back(Pulse::Low);
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
    assert_eq!(bn.pulses[0], Pulse::High);
    assert_eq!(bn.pulses[1], Pulse::Low);
    assert_eq!(bn.pulses[2], Pulse::High);

    let cn = nodes.get("c").unwrap();
    assert_eq!(cn.pulses.len(), 3);
    assert_eq!(cn.pulses[0], Pulse::High);
    assert_eq!(cn.pulses[1], Pulse::Low);
    assert_eq!(cn.pulses[2], Pulse::High);
}
