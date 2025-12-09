use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let boxes = parse(INPUT);
    println!("part 1: {}", part_1(&boxes));
}

fn parse(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .map(|line| {
            let [a, b, c] = line
                .split(',')
                .map(|s| s.parse().expect("input must be an integer"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("input must have exactly 3 components");
            (a, b, c)
        })
        .collect()
}

fn do_connections<const N: usize>(boxes: &[(u32, u32, u32)]) -> u32 {
    let mut possible_edges = BinaryHeap::new();

    for (i, j) in (0..boxes.len()).tuple_combinations() {
        let (x1, y1, z1) = boxes[i];
        let (x2, y2, z2) = boxes[j];

        // since we are asked for straight-line distance, just compute the square of the hypotenuse
        // which is easier.  Convert to signed so I don't have to worry about subtraction, and
        // expand to 64 bits because some of my input data looked like it might be larger than 16
        // bits (so squaring it would be more than 32).
        let distance_squared = (x1 as i64 - x2 as i64).pow(2)
            + (y1 as i64 - y2 as i64).pow(2)
            + (z1 as i64 - z2 as i64).pow(2);

        possible_edges.push((Reverse(distance_squared), i, j));
    }

    let mut actual_edges = HashMap::<usize, Vec<usize>>::new();
    for _ in 0..N {
        let Some((_distance, i, j)) = possible_edges.pop() else {
            panic!("we had fewer than {N} edges");
        };

        // double the edges in the list so I don't have to care that this is an undirected graph
        actual_edges.entry(i).or_default().push(j);
        actual_edges.entry(j).or_default().push(i);
    }

    let mut seen = HashSet::new();
    let mut neighborhoods = Vec::new();
    for i in 0..boxes.len() {
        if !seen.insert(i) {
            continue;
        }

        let mut neighborhood = HashSet::from([i]);

        let mut queue = actual_edges.get(&i).cloned().unwrap_or_default();
        while let Some(j) = queue.pop() {
            if !seen.insert(j) {
                continue;
            }
            neighborhood.insert(j);
            queue.extend_from_slice(&actual_edges[&j]);
        }

        neighborhoods.push(neighborhood.len());
    }

    neighborhoods.sort_unstable_by_key(|&count| std::cmp::Reverse(count));
    neighborhoods
        .into_iter()
        .take(3)
        .map(|i| i as u32)
        .product()
}

fn part_1(boxes: &[(u32, u32, u32)]) -> u32 {
    do_connections::<1000>(boxes)
}

#[cfg(test)]
mod tests {
    use crate::{do_connections, parse};

    #[test]
    fn example() {
        let boxes = parse(
            "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        );
        assert_eq!(40, do_connections::<10>(&boxes))
    }
}
