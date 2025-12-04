use std::cmp::min;

use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    println!("part 1: {}", part_1(&input));
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| match b {
                    b'@' => true,
                    b'.' => false,
                    _ => panic!("invalid"),
                })
                .collect()
        })
        .collect()
}

fn part_1(map: &[Vec<bool>]) -> usize {
    (0..map.len())
        .flat_map(|i| {
            (0..map[0].len()).filter(move |&j| {
                if !map[i][j] {
                    return false;
                }

                let xs = i.saturating_sub(1)..min(map.len(), i + 2);
                let ys = j.saturating_sub(1)..min(map[0].len(), j + 2);

                xs.cartesian_product(ys)
                    .filter(|&(x, y)| (x, y) != (i, j) && map[x][y])
                    .count()
                    < 4
            })
        })
        .count()
}
