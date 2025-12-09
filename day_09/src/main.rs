use std::{
    cmp::{Reverse, max, min},
    collections::HashSet,
};

use itertools::Itertools;

static INPUT: &str = include_str!("input.txt");

fn main() {
    let red_squares = parse(INPUT);
    println!("part 1: {}", part_1(&red_squares));
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|l| {
            let [a, b] = l
                .split(',')
                .map(|s| s.parse().expect("input must be an integer"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("input must be pairs");
            (a, b)
        })
        .collect()
}

fn part_1(red_squares: &[(u32, u32)]) -> u64 {
    red_squares
        .iter()
        .tuple_combinations()
        .map(|(&(x1, y1), &(x2, y2))| {
            ((x1 as i64 - x2 as i64).unsigned_abs() + 1)
                * ((y1 as i64 - y2 as i64).unsigned_abs() + 1)
        })
        .max()
        .expect("there was at least one pair of squares")
}

fn part_2(red_squares: &[(u32, u32)]) -> u64 {
    let red_or_green_squares: HashSet<(u32, u32)> = red_squares
        .iter()
        .chain([&red_squares[0]].into_iter())
        .tuple_windows()
        .flat_map(
            |(&(x1, y1), &(x2, y2))| -> Box<dyn Iterator<Item = (u32, u32)>> {
                if x1 == x2 {
                    Box::new((min(y1, y2)..=max(y1, y2)).map(move |y| (x1, y)))
                } else if y1 == y2 {
                    Box::new((min(x1, x2)..=max(x1, x2)).map(move |x| (x, y1)))
                } else {
                    panic!("the tiles did not form a straight horizontal or vertical line");
                }
            },
        )
        .collect();

    let mut areas = red_squares
        .iter()
        .tuple_combinations()
        .map(|(&(x1, y1), &(x2, y2))| {
            (
                (x1, y1),
                (x2, y2),
                ((x1 as i64 - x2 as i64).unsigned_abs() + 1)
                    * ((y1 as i64 - y2 as i64).unsigned_abs() + 1),
            )
        })
        .collect_vec();
    areas.sort_unstable_by_key(|&(_corner_a, _corner_b, area)| Reverse(area));

    areas
        .into_iter()
        .filter_map(|((x1, y1), (x2, y2), area)| {
            let xs = min(x1, x2)..=max(x1, x2);
            let ys = min(y1, y2)..=max(y1, y2);
            if xs
                .cartesian_product(ys)
                .all(|cell| red_or_green_squares.contains(&cell))
            {
                Some(area)
            } else {
                None
            }
        })
        .next()
        .expect("no rectangles that are filled with only red and green")
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2};

    #[test]
    fn example() {
        let red_squares = parse(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!(50, part_1(&red_squares));
        assert_eq!(24, part_2(&red_squares));
    }
}
