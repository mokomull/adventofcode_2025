use std::collections::HashMap;

use petgraph::{Graph, graph::NodeIndex};

static INPUT: &str = include_str!("input.txt");

fn main() {
    let (graph, you, out) = parse(INPUT);
}

fn parse<'a>(input: &'a str) -> (Graph<(), ()>, NodeIndex, NodeIndex) {
    let mut nodes = HashMap::new();
    let mut graph = Graph::new();

    for line in input.lines() {
        let (from, tos) = line
            .split_once(": ")
            .expect("line must contain from and to");
        let tos = tos.split(' ');

        for node in std::iter::once(from).chain(tos.clone()) {
            nodes.entry(node).or_insert_with(|| graph.add_node(()));
        }

        let from = nodes[from];
        for to in tos {
            graph.add_edge(from, nodes[to], ());
        }
    }

    (graph, nodes["you"], nodes["out"])
}
