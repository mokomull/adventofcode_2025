use std::cmp::min;

use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input(INPUT);

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(input));
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

fn part_2(mut map: Vec<Vec<bool>>) -> usize {
    let mut count = 0;

    loop {
        let to_remove: Vec<_> = (0..map.len())
            .flat_map(|i| {
                let map = &map; // move i into the next closure but not map
                (0..map[0].len()).filter_map(move |j| {
                    if !map[i][j] {
                        return None;
                    }

                    let xs = i.saturating_sub(1)..min(map.len(), i + 2);
                    let ys = j.saturating_sub(1)..min(map[0].len(), j + 2);

                    if xs
                        .cartesian_product(ys)
                        .filter(|&(x, y)| (x, y) != (i, j) && map[x][y])
                        .count()
                        < 4
                    {
                        Some((i, j))
                    } else {
                        None
                    }
                })
            })
            .collect();

        if to_remove.is_empty() {
            return count;
        }

        count += to_remove.len();

        for (i, j) in to_remove {
            assert!(map[i][j]);
            map[i][j] = false;
        }
    }
}
