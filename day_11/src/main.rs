use std::collections::HashMap;

use petgraph::{
    Direction::Incoming,
    Graph,
    graph::NodeIndex,
    visit::{EdgeRef, Topo},
};

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
    count_paths_via(graph, you, out, &[])
}

fn part_2(
    graph: &Graph<(), ()>,
    you: NodeIndex,
    out: NodeIndex,
    fft: NodeIndex,
    dac: NodeIndex,
) -> usize {
    count_paths_via(graph, you, out, &[fft, dac])
}

fn count_paths_via(
    graph: &Graph<(), ()>,
    from: NodeIndex,
    to: NodeIndex,
    via: &[NodeIndex],
) -> usize {
    // unnecessary optimization: store via in a bitfield
    assert!(via.len() < 8);

    let mut distances = HashMap::from([((from, 0), 1)]);
    let mut topo = Topo::new(graph);
    while let Some(node) = topo.next(graph) {
        // precompute a one-bit-set bitmask if this is one of the nodes that we must go via
        let or = match via.iter().position(|&i| node == i) {
            Some(i) => 1 << i,
            None => 0,
        };

        for prev in graph.edges_directed(node, Incoming).map(|e| e.source()) {
            for visited_via in 0..(1 << via.len()) {
                let Some(&prev_count) = distances.get(&(prev, visited_via)) else {
                    continue;
                };

                *distances.entry((node, visited_via | or)).or_default() += prev_count;
            }
        }
    }

    distances[&(to, (1 << via.len()) - 1)]
}

#[cfg(test)]
mod tests {
    use crate::{INPUT, parse, part_1, part_2};

    #[test]
    fn example_part1() {
        let (graph, you, out, _, _, _) = parse(
            "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
xxx: svr fft dac",
        );
        assert_eq!(5, part_1(&graph, you, out));
    }

    #[test]
    fn example_part2() {
        let (graph, _, out, svr, fft, dac) = parse(
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
you: xxx",
        );

        assert_eq!(2, part_2(&graph, svr, out, fft, dac));
    }

    #[test]
    fn personal_input() {
        let (graph, you, out, _, _, _) = parse(INPUT);
        assert_eq!(508, part_1(&graph, you, out));
    }
}
