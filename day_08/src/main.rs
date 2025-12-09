use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let boxes = parse(INPUT);
    println!("part 1: {}", part_1(&boxes));
    println!("part 2: {}", part_2(&boxes));
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

fn generate_edge_heap(boxes: &[(u32, u32, u32)]) -> BinaryHeap<(Reverse<i64>, usize, usize)> {
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

    possible_edges
}

fn do_connections<const N: usize>(boxes: &[(u32, u32, u32)]) -> u32 {
    let mut possible_edges = generate_edge_heap(boxes);
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

fn part_2(boxes: &[(u32, u32, u32)]) -> u32 {
    let mut edges = generate_edge_heap(boxes);
    let mut neighborhood_ids_by_box = HashMap::new();

    // this outer loop variable exists simply to generate new neighborhood ids.  not all will be
    // used.
    for neighborhood in 0u32.. {
        let (_distance, i, j) = edges.pop().expect("we ran out of edges");

        let neighborhood_i = neighborhood_ids_by_box.get(&i);
        let neighborhood_j = neighborhood_ids_by_box.get(&j);

        // if they've both been assigned a neighborhood id, then we just need to pick a canonical
        // one and update the whole other subgraph
        if let Some(&neighborhood_a) = neighborhood_i
            && let Some(&neighborhood_b) = neighborhood_j
        {
            // if they're already in the same neighborhood, we can even skip our "are we done" check
            // that doesn't actually make anything any more done
            if neighborhood_a == neighborhood_b {
                continue;
            }

            // by fair dice roll, the canonical neighborhood id is the lower id
            let canonical_id = std::cmp::min(neighborhood_a, neighborhood_b);
            for v in neighborhood_ids_by_box.values_mut() {
                if *v == neighborhood_a || *v == neighborhood_b {
                    *v = canonical_id;
                }
            }
        } else if let Some(&existing_neighborhood) = neighborhood_i {
            // if i was already in a neighborhood, then j now belongs to it too
            assert!(
                neighborhood_ids_by_box
                    .insert(j, existing_neighborhood)
                    .is_none(),
                "oops overwrote a neighborhood"
            );
        } else if let Some(&existing_neighborhood) = neighborhood_j {
            // if j was already in a neighborhood, then i now belongs to it too
            assert!(
                neighborhood_ids_by_box
                    .insert(i, existing_neighborhood)
                    .is_none(),
                "oops overwrote a neighborhood"
            );
        } else {
            // and if neither was, then they both belong to a brand new neighborhood now.  This is
            // the only case that actually uses the outer-loop's loop variable.
            assert!(
                neighborhood_ids_by_box.insert(i, neighborhood).is_none(),
                "oops overwrote a neighborhood"
            );
            assert!(
                neighborhood_ids_by_box.insert(j, neighborhood).is_none(),
                "oops overwrote a neighborhood"
            );
        }

        if neighborhood_ids_by_box.len() == boxes.len()
            && neighborhood_ids_by_box.values().all_equal()
        {
            let (x1, _y1, _z1) = boxes[i];
            let (x2, _y2, _z2) = boxes[j];
            return x1 * x2;
        }
    }

    panic!("we never finished")
}

#[cfg(test)]
mod tests {
    use crate::{INPUT, do_connections, parse, part_1, part_2};

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
        assert_eq!(40, do_connections::<10>(&boxes));
        assert_eq!(25272, part_2(&boxes));
    }

    #[test]
    fn personal_input() {
        let boxes = parse(INPUT);
        assert_eq!(98696, part_1(&boxes));
        assert_eq!(2245203960, part_2(&boxes));
    }
}
