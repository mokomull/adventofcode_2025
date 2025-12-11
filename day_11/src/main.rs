use std::{collections::HashMap, hash::RandomState};

use petgraph::{Graph, algo::all_simple_paths, graph::NodeIndex};

static INPUT: &str = include_str!("input.txt");

fn main() {
    let (graph, you, out, svr, fft, dac) = parse(INPUT);
    println!("part 1: {}", part_1(&graph, you, out));
    println!("part 2: {}", part_2(&graph, svr, out, fft, dac));
}

fn parse<'a>(
    input: &'a str,
) -> (
    Graph<(), ()>,
    NodeIndex,
    NodeIndex,
    NodeIndex,
    NodeIndex,
    NodeIndex,
) {
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

    (
        graph,
        nodes["you"],
        nodes["out"],
        nodes["svr"],
        nodes["fft"],
        nodes["dac"],
    )
}

fn part_1(graph: &Graph<(), ()>, you: NodeIndex, out: NodeIndex) -> usize {
    all_simple_paths::<Vec<_>, _, RandomState>(graph, you, out, 0, None).count()
}

fn part_2(
    graph: &Graph<(), ()>,
    you: NodeIndex,
    out: NodeIndex,
    fft: NodeIndex,
    dac: NodeIndex,
) -> usize {
    all_simple_paths::<Vec<_>, _, RandomState>(graph, you, out, 0, None)
        .filter(|path| path.contains(&fft) && path.contains(&dac))
        .count()
}
